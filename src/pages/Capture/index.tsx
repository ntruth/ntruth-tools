import { Component, Show, createSignal, createEffect, onCleanup, onMount, createMemo } from 'solid-js'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { CaptureToolbar, type Selection } from '../../components/CaptureToolbar'

// Capture status state machine
type CaptureStatus = 'idle' | 'capturing' | 'selecting' | 'editing'

interface CaptureData {
  data: string
  width: number
  height: number
}

type AnnotTool = 'none' | 'draw' | 'rect'

type OverlayShape =
  | { kind: 'stroke'; points: Array<{ x: number; y: number }>; width: number; color: string }
  | { kind: 'rect'; x: number; y: number; w: number; h: number; width: number; color: string }

const MIN_SELECTION_SIZE = 10 // Minimum selection size to trigger editing mode

const CapturePage: Component = () => {
  // State
  const [status, setStatus] = createSignal<CaptureStatus>('idle')
  const [bgImage, setBgImage] = createSignal<HTMLImageElement | null>(null)
  const [selection, setSelection] = createSignal<Selection | null>(null)
  const [startPoint, setStartPoint] = createSignal({ x: 0, y: 0 })

  // Annotation
  const [tool, setTool] = createSignal<AnnotTool>('none')
  const [overlayShapes, setOverlayShapes] = createSignal<OverlayShape[]>([])
  const [isAnnotating, setIsAnnotating] = createSignal(false)
  const [annotStart, setAnnotStart] = createSignal({ x: 0, y: 0 })
  const [activeStroke, setActiveStroke] = createSignal<OverlayShape | null>(null)

  // Refs
  let canvasRef: HTMLCanvasElement | undefined
  let overlayCanvasRef: HTMLCanvasElement | undefined // For drawing annotations

  // Computed: check if selection is valid for editing
  const hasValidSelection = createMemo(() => {
    const sel = selection()
    return sel && sel.w >= MIN_SELECTION_SIZE && sel.h >= MIN_SELECTION_SIZE
  })

  // Reactive effect: redraw canvas whenever bgImage or selection changes
  createEffect(() => {
    const image = bgImage()
    selection() // Track selection changes for reactivity
    if (image && canvasRef) {
      drawCanvas(image)
    }
  })

  onMount(async () => {
    console.log('[Capture] Page mounted, status:', status())
    window.addEventListener('keydown', onKeyDown)

    const syncCanvasSize = () => {
      if (canvasRef) {
        canvasRef.width = window.innerWidth
        canvasRef.height = window.innerHeight
      }
      if (overlayCanvasRef) {
        overlayCanvasRef.width = window.innerWidth
        overlayCanvasRef.height = window.innerHeight
        redrawOverlay()
      }
      const image = bgImage()
      if (image && canvasRef) {
        drawCanvas(image)
      }
    }
    syncCanvasSize()
    window.addEventListener('resize', syncCanvasSize)

    // Listen for capture:ready event from backend
    const unlisten = await listen<CaptureData>('capture:ready', (event) => {
      console.log('[Capture] Received capture:ready event:', event.payload.width, 'x', event.payload.height)
      setStatus('capturing')
      
      const img = new Image()
      img.onload = () => {
        console.log('[Capture] Image loaded:', img.width, 'x', img.height)
        setBgImage(img)
        setStatus('selecting')
      }
      img.onerror = (e) => {
        console.error('[Capture] Failed to load image:', e)
        setStatus('idle')
      }
      img.src = `data:image/png;base64,${event.payload.data}`
    })

    // Listen for reset event (when window is hidden and re-shown)
    const unlistenReset = await listen('capture:reset', () => {
      console.log('[Capture] Reset event received')
      resetState()
    })

    onCleanup(() => {
      console.log('[Capture] Cleaning up...')
      unlisten()
      unlistenReset()
      window.removeEventListener('keydown', onKeyDown)
      window.removeEventListener('resize', syncCanvasSize)
    })
  })

  const resetState = () => {
    setBgImage(null)
    setSelection(null)
    setTool('none')
    setOverlayShapes([])
    setActiveStroke(null)
    setIsAnnotating(false)
    setStatus('idle')
    // Clear canvases
    if (canvasRef) {
      const ctx = canvasRef.getContext('2d')
      ctx?.clearRect(0, 0, canvasRef.width, canvasRef.height)
    }
    if (overlayCanvasRef) {
      const ctx = overlayCanvasRef.getContext('2d')
      ctx?.clearRect(0, 0, overlayCanvasRef.width, overlayCanvasRef.height)
    }
  }

  const onKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'Escape') {
      handleCancel()
    }
  }

  const drawCanvas = (image: HTMLImageElement) => {
    const canvas = canvasRef
    if (!canvas) return

    const ctx = canvas.getContext('2d')
    if (!ctx) return

    canvas.width = window.innerWidth
    canvas.height = window.innerHeight

    // Draw background image
    ctx.drawImage(image, 0, 0, canvas.width, canvas.height)

    // Draw dark overlay
    ctx.fillStyle = 'rgba(0, 0, 0, 0.4)'
    ctx.fillRect(0, 0, canvas.width, canvas.height)

    // Cut out selection area
    const sel = selection()
    if (sel && sel.w > 0 && sel.h > 0) {
      ctx.clearRect(sel.x, sel.y, sel.w, sel.h)
      // Redraw image in selection
      const scaleX = image.width / canvas.width
      const scaleY = image.height / canvas.height
      ctx.drawImage(
        image,
        sel.x * scaleX, sel.y * scaleY, sel.w * scaleX, sel.h * scaleY,
        sel.x, sel.y, sel.w, sel.h
      )
      // Selection border
      ctx.strokeStyle = '#4f9cff'
      ctx.lineWidth = 2
      ctx.setLineDash([])
      ctx.strokeRect(sel.x, sel.y, sel.w, sel.h)

      // Corner handles when editing
      if (status() === 'editing') {
        drawResizeHandles(ctx, sel)
      }
    }
  }

  const redrawOverlay = () => {
    const canvas = overlayCanvasRef
    if (!canvas) return
    const ctx = canvas.getContext('2d')
    if (!ctx) return

    ctx.clearRect(0, 0, canvas.width, canvas.height)

    const shapes = overlayShapes()
    for (const shape of shapes) {
      ctx.strokeStyle = shape.color
      ctx.lineWidth = shape.width
      ctx.lineCap = 'round'
      ctx.lineJoin = 'round'

      if (shape.kind === 'stroke') {
        const pts = shape.points
        if (pts.length < 2) continue
        ctx.beginPath()
        ctx.moveTo(pts[0].x, pts[0].y)
        for (let i = 1; i < pts.length; i++) ctx.lineTo(pts[i].x, pts[i].y)
        ctx.stroke()
      } else if (shape.kind === 'rect') {
        ctx.strokeRect(shape.x, shape.y, shape.w, shape.h)
      }
    }

    const active = activeStroke()
    if (active && active.kind === 'stroke') {
      ctx.strokeStyle = active.color
      ctx.lineWidth = active.width
      ctx.lineCap = 'round'
      ctx.lineJoin = 'round'
      const pts = active.points
      if (pts.length >= 2) {
        ctx.beginPath()
        ctx.moveTo(pts[0].x, pts[0].y)
        for (let i = 1; i < pts.length; i++) ctx.lineTo(pts[i].x, pts[i].y)
        ctx.stroke()
      }
    }
  }

  const drawResizeHandles = (ctx: CanvasRenderingContext2D, sel: Selection) => {
    const handleSize = 8
    ctx.fillStyle = '#4f9cff'
    const corners = [
      [sel.x, sel.y], // top-left
      [sel.x + sel.w, sel.y], // top-right
      [sel.x, sel.y + sel.h], // bottom-left
      [sel.x + sel.w, sel.y + sel.h], // bottom-right
    ]
    corners.forEach(([x, y]) => {
      ctx.fillRect(x - handleSize / 2, y - handleSize / 2, handleSize, handleSize)
    })
  }

  // Mouse handlers
  const onMouseDown = (e: MouseEvent) => {
    if (status() !== 'selecting' && status() !== 'editing') return

    // Ignore clicks on toolbar
    const path = (e.composedPath?.() ?? []) as unknown[]
    const clickedToolbar = path.some((el) => {
      const node = el as HTMLElement
      return node?.dataset?.captureToolbar === 'true'
    })
    if (clickedToolbar) return

    // If annotating, don't start a new selection
    if (status() === 'editing' && tool() !== 'none') return
    
    // If in editing mode, check if clicking outside selection to reset
    if (status() === 'editing') {
      const sel = selection()
      if (sel && !isInsideSelection(e.clientX, e.clientY, sel)) {
        setStatus('selecting')
        setTool('none')
      }
    }

    const x = e.clientX
    const y = e.clientY
    setStartPoint({ x, y })
    setSelection({ x, y, w: 0, h: 0 })
    setStatus('selecting')
  }

  const isInsideSelection = (x: number, y: number, sel: Selection) => {
    return x >= sel.x && x <= sel.x + sel.w && y >= sel.y && y <= sel.y + sel.h
  }

  const onMouseMove = (e: MouseEvent) => {
    if (status() !== 'selecting') return
    
    const x = e.clientX
    const y = e.clientY
    const s = startPoint()
    
    setSelection({
      x: Math.min(s.x, x),
      y: Math.min(s.y, y),
      w: Math.abs(x - s.x),
      h: Math.abs(y - s.y),
    })
  }

  const onMouseUp = () => {
    if (status() !== 'selecting') return
    
    // Check if selection is large enough
    if (hasValidSelection()) {
      setStatus('editing')
      console.log('[Capture] Selection complete, entering editing mode')
    }
  }

  // Overlay annotation handlers (active in editing mode)
  const onOverlayMouseDown = (e: MouseEvent) => {
    if (status() !== 'editing' || tool() === 'none') return
    const sel = selection()
    if (!sel || !isInsideSelection(e.clientX, e.clientY, sel)) return

    e.stopPropagation()
    setIsAnnotating(true)
    setAnnotStart({ x: e.clientX, y: e.clientY })

    if (tool() === 'draw') {
      setActiveStroke({ kind: 'stroke', points: [{ x: e.clientX, y: e.clientY }], width: 3, color: '#ff3b30' })
    }
  }

  const onOverlayMouseMove = (e: MouseEvent) => {
    if (!isAnnotating() || status() !== 'editing' || tool() === 'none') return
    e.stopPropagation()

    if (tool() === 'draw') {
      const active = activeStroke()
      if (!active || active.kind !== 'stroke') return
      active.points.push({ x: e.clientX, y: e.clientY })
      setActiveStroke({ ...active, points: [...active.points] })
      redrawOverlay()
    } else if (tool() === 'rect') {
      const start = annotStart()
      const rect: OverlayShape = {
        kind: 'rect',
        x: Math.min(start.x, e.clientX),
        y: Math.min(start.y, e.clientY),
        w: Math.abs(e.clientX - start.x),
        h: Math.abs(e.clientY - start.y),
        width: 3,
        color: '#ff3b30',
      }
      // preview rect by setting activeStroke as null and drawing preview last
      setActiveStroke(null)
      redrawOverlay()
      const canvas = overlayCanvasRef
      const ctx = canvas?.getContext('2d')
      if (ctx) {
        ctx.strokeStyle = rect.color
        ctx.lineWidth = rect.width
        ctx.strokeRect(rect.x, rect.y, rect.w, rect.h)
      }
    }
  }

  const onOverlayMouseUp = (e: MouseEvent) => {
    if (!isAnnotating() || status() !== 'editing' || tool() === 'none') return
    e.stopPropagation()

    if (tool() === 'draw') {
      const active = activeStroke()
      if (active && active.kind === 'stroke' && active.points.length >= 2) {
        setOverlayShapes([...overlayShapes(), active])
      }
      setActiveStroke(null)
      redrawOverlay()
    } else if (tool() === 'rect') {
      const start = annotStart()
      const rect: OverlayShape = {
        kind: 'rect',
        x: Math.min(start.x, e.clientX),
        y: Math.min(start.y, e.clientY),
        w: Math.abs(e.clientX - start.x),
        h: Math.abs(e.clientY - start.y),
        width: 3,
        color: '#ff3b30',
      }
      // ignore tiny rect
      if (rect.w >= 2 && rect.h >= 2) setOverlayShapes([...overlayShapes(), rect])
      redrawOverlay()
    }

    setIsAnnotating(false)
  }

  // Get selection image as Blob
  const getSelectionBlob = async (): Promise<Blob | null> => {
    const sel = selection()
    const image = bgImage()
    if (!sel || !image) return null

    const cropCanvas = document.createElement('canvas')
    cropCanvas.width = sel.w
    cropCanvas.height = sel.h
    const ctx = cropCanvas.getContext('2d')
    if (!ctx) return null

    const scaleX = image.width / window.innerWidth
    const scaleY = image.height / window.innerHeight
    ctx.drawImage(
      image,
      sel.x * scaleX, sel.y * scaleY, sel.w * scaleX, sel.h * scaleY,
      0, 0, sel.w, sel.h
    )

    // Composite overlay annotations if any
    if (overlayCanvasRef) {
      try {
        ctx.drawImage(overlayCanvasRef, sel.x, sel.y, sel.w, sel.h, 0, 0, sel.w, sel.h)
      } catch {
        // ignore
      }
    }

    return new Promise((resolve) => cropCanvas.toBlob(resolve, 'image/png'))
  }

  // Toolbar handlers
  const handleCopy = async () => {
    try {
      const blob = await getSelectionBlob()
      if (!blob) return

      await navigator.clipboard.write([
        new ClipboardItem({ 'image/png': blob })
      ])
      console.log('[Capture] Copied to clipboard')
      handleCancel()
    } catch (err) {
      console.error('[Capture] Copy failed:', err)
      // Fallback to Rust clipboard
      const blob = await getSelectionBlob()
      if (!blob) return
      const buffer = new Uint8Array(await blob.arrayBuffer())
      await invoke('save_capture', { png_bytes: Array.from(buffer) })
      handleCancel()
    }
  }

  const handleSave = async () => {
    const blob = await getSelectionBlob()
    if (!blob) return

    const buffer = new Uint8Array(await blob.arrayBuffer())
    await invoke('save_capture', { png_bytes: Array.from(buffer) })
    handleCancel()
  }

  const handlePin = async () => {
    const sel = selection()
    const blob = await getSelectionBlob()
    if (!sel || !blob) return

    // Convert blob to base64
    const reader = new FileReader()
    reader.onload = async () => {
      const base64 = (reader.result as string).split(',')[1]
      try {
        await invoke('create_pin_window', {
          image_data: base64,
          width: Math.round(sel.w),
          height: Math.round(sel.h),
          x: Math.round(sel.x),
          y: Math.round(sel.y),
        })
        handleCancel()
      } catch (err) {
        console.error('[Capture] Pin failed:', err)
      }
    }
    reader.readAsDataURL(blob)
  }

  const handleCancel = async () => {
    resetState()
    await invoke('hide_capture_window')
  }

  // Cursor style based on status
  const cursorClass = createMemo(() => {
    switch (status()) {
      case 'selecting': return 'cursor-crosshair'
      case 'editing': return 'cursor-default'
      default: return 'cursor-default'
    }
  })

  return (
    <div
      class={`relative h-screen w-screen select-none overflow-hidden ${cursorClass()}`}
      onMouseDown={onMouseDown}
      onMouseMove={onMouseMove}
      onMouseUp={onMouseUp}
    >
      {/* Main canvas */}
      <Show when={status() !== 'idle'}>
        <canvas
          ref={canvasRef}
          class="absolute left-0 top-0"
          width={window.innerWidth}
          height={window.innerHeight}
        />
      </Show>

      {/* Overlay canvas for annotations */}
      <Show when={status() === 'editing'}>
        <canvas
          ref={overlayCanvasRef}
          class={tool() === 'none' ? 'absolute left-0 top-0 pointer-events-none' : 'absolute left-0 top-0 pointer-events-auto cursor-crosshair'}
          width={window.innerWidth}
          height={window.innerHeight}
          onMouseDown={onOverlayMouseDown}
          onMouseMove={onOverlayMouseMove}
          onMouseUp={onOverlayMouseUp}
        />
      </Show>

      {/* Selection size indicator */}
      <Show when={status() === 'selecting' && selection() && selection()!.w > 0}>
        <div
          class="pointer-events-none absolute rounded bg-black/80 px-2 py-1 text-xs text-white font-mono"
          style={{
            left: `${selection()!.x}px`,
            top: `${Math.max(0, selection()!.y - 28)}px`,
          }}
        >
          {Math.round(selection()!.w)} × {Math.round(selection()!.h)}
        </div>
      </Show>

      {/* Smart Toolbar - appears after selection complete */}
      <Show when={status() === 'editing' && hasValidSelection()}>
        <CaptureToolbar
          selection={selection()!}
          onCopy={handleCopy}
          onSave={handleSave}
          onPin={handlePin}
          onCancel={handleCancel}
          onDraw={() => setTool(tool() === 'draw' ? 'none' : 'draw')}
          onRect={() => setTool(tool() === 'rect' ? 'none' : 'rect')}
        />
      </Show>

      {/* Loading/idle state */}
      <Show when={status() === 'idle' || status() === 'capturing'}>
        <div class="flex h-full w-full items-center justify-center">
          <Show when={status() === 'capturing'}>
            <div class="text-white/50 text-sm">正在截取屏幕...</div>
          </Show>
        </div>
      </Show>
    </div>
  )
}

export default CapturePage
