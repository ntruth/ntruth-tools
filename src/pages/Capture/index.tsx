import { Component, Show, createSignal, createEffect, onCleanup, onMount, createMemo } from 'solid-js'
import { convertFileSrc, invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { save } from '@tauri-apps/plugin-dialog'
import { CaptureToolbar, type Selection } from '../../components/CaptureToolbar'
import AnnotationLayer, { type AnnotationLayerApi } from '../../windows/Capture/AnnotationLayer'
import type { AnnotationStyle } from '../../windows/Capture/DrawManager'
import OCRResult from '../../windows/Capture/OCRResult'

// Capture status state machine
type CaptureStatus = 'idle' | 'capturing' | 'selecting' | 'editing'

interface CaptureData {
  data?: string
  path?: string
  width: number
  height: number
  monitorX?: number  // Window screen X offset (for multi-monitor)
  monitorY?: number  // Window screen Y offset (for multi-monitor)
}

interface ElementRect {
  left: number
  top: number
  right: number
  bottom: number
}

const MIN_SELECTION_SIZE = 10 // Minimum selection size to trigger editing mode
const ELEMENT_DETECT_THROTTLE = 50 // ms between UI element detection requests

const CapturePage: Component = () => {
  // State
  const [status, setStatus] = createSignal<CaptureStatus>('idle')
  const [bgImage, setBgImage] = createSignal<HTMLImageElement | null>(null)
  const [selection, setSelection] = createSignal<Selection | null>(null)
  const [startPoint, setStartPoint] = createSignal({ x: 0, y: 0 })
  const [isDragging, setIsDragging] = createSignal(false)

  // Konva annotation
  const [cropDataUrl, setCropDataUrl] = createSignal<string>('')
  const [annotationTool, setAnnotationTool] = createSignal<
    'select' | 'rect' | 'ellipse' | 'line' | 'arrow' | 'pencil' | 'marker' | 'text' | 'mosaic'
  >('select')
  const [annotationApi, setAnnotationApi] = createSignal<AnnotationLayerApi | null>(null)
  const [annotationStyle, setAnnotationStyle] = createSignal<AnnotationStyle>({
    stroke: '#ff3b30',
    opacity: 1,
    fillEnabled: false,
    fill: '#ff3b30',
    strokeWidth: 3,
    dashEnabled: false,
    dash: [8, 4],
    arrowMode: 'end',
    arrowHeadStyle: 'filled',
    arrowPointerLength: 12,
    arrowPointerWidth: 12,
    fontFamily: 'Microsoft YaHei UI',
    textBold: false,
    textItalic: false,
    fontSize: 22,
    textBgEnabled: false,
    textBgColor: '#000000',
    textBgOpacity: 0.6,
    textPadding: 6,
    textBgRadius: 4,
    mosaicPixelSize: 12,
  })

  // OCR
  const [ocrOpen, setOcrOpen] = createSignal(false)
  const [ocrLoading, setOcrLoading] = createSignal(false)
  const [ocrText, setOcrText] = createSignal('')
  const [ocrPreviewSrc, setOcrPreviewSrc] = createSignal('')

  // UI Element auto-detection
  const [elementRect, setElementRect] = createSignal<ElementRect | null>(null)
  const [autoDetectEnabled, setAutoDetectEnabled] = createSignal(true)
  const [monitorOffset, setMonitorOffset] = createSignal({ x: 0, y: 0 })  // Window screen position
  let lastDetectTime = 0
  let detectPending = false

  // Refs
  let canvasRef: HTMLCanvasElement | undefined

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

  // Build cropped background data URL when entering editing
  createEffect(() => {
    const image = bgImage()
    const sel = selection()
    const st = status()
    if (!image || !sel || st !== 'editing') return
    if (sel.w <= 0 || sel.h <= 0) return

    // Crop once for Konva background
    const scaleX = image.width / window.innerWidth
    const scaleY = image.height / window.innerHeight

    const srcX = Math.max(0, Math.round(sel.x * scaleX))
    const srcY = Math.max(0, Math.round(sel.y * scaleY))
    const srcW = Math.max(1, Math.round(sel.w * scaleX))
    const srcH = Math.max(1, Math.round(sel.h * scaleY))

    const cropCanvas = document.createElement('canvas')
    // Keep native pixels to avoid blurry annotation background on HiDPI screens.
    cropCanvas.width = srcW
    cropCanvas.height = srcH
    const ctx = cropCanvas.getContext('2d')
    if (!ctx) return

    ctx.imageSmoothingEnabled = true
    ctx.imageSmoothingQuality = 'high'
    ctx.drawImage(image, srcX, srcY, srcW, srcH, 0, 0, srcW, srcH)
    setCropDataUrl(cropCanvas.toDataURL('image/png'))
  })

  onMount(async () => {
    console.log('[Capture] Page mounted, status:', status())
    
    // IMPORTANT: Signal backend IMMEDIATELY that we're alive
    // Don't wait for listeners to be set up - backend needs to know ASAP
    invoke('capture_frontend_ready').catch(err => {
      console.warn('[Capture] capture_frontend_ready failed:', err)
    })
    
    window.addEventListener('keydown', onKeyDown)

    const syncCanvasSize = () => {
      if (canvasRef) {
        const dpr = window.devicePixelRatio || 1
        canvasRef.width = Math.max(1, Math.round(window.innerWidth * dpr))
        canvasRef.height = Math.max(1, Math.round(window.innerHeight * dpr))
        canvasRef.style.width = `${window.innerWidth}px`
        canvasRef.style.height = `${window.innerHeight}px`
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
      console.log('[Capture] Received capture:ready event:', event.payload.width, 'x', event.payload.height,
        'monitor offset:', event.payload.monitorX, event.payload.monitorY,
        'has data:', !!event.payload.data, 'has path:', !!event.payload.path)

      // Store monitor offset for coordinate conversion
      setMonitorOffset({ 
        x: event.payload.monitorX ?? 0, 
        y: event.payload.monitorY ?? 0 
      })

      // Fresh capture: clear any previous selection/overlay state first.
      setSelection(null)
      setCropDataUrl('')
      setAnnotationApi(null)
      setAnnotationTool('select')
      setIsDragging(false)
      setElementRect(null)
      setStatus('capturing')
      
      const img = new Image()
      img.onload = () => {
        console.log('[Capture] Image loaded successfully:', img.width, 'x', img.height)
        setBgImage(img)
        setStatus('selecting')
        // Force redraw
        if (canvasRef) {
          drawCanvas(img)
        }
      }
      img.onerror = (e) => {
        console.error('[Capture] Failed to load image:', e)
        setStatus('idle')
      }
      
      // Build image source
      if (event.payload.data) {
        const src = `data:image/png;base64,${event.payload.data}`
        console.log('[Capture] Loading from base64, length:', event.payload.data.length)
        img.src = src
      } else if (event.payload.path) {
        const src = convertFileSrc(event.payload.path)
        console.log('[Capture] Loading from path:', src)
        img.src = src
      } else {
        console.error('[Capture] No image data in payload!')
        setStatus('idle')
      }
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
    setCropDataUrl('')
    setAnnotationTool('select')
    setAnnotationApi(null)
    setStatus('idle')
    setElementRect(null) // Clear UI element highlight
    // Clear canvases
    if (canvasRef) {
      const ctx = canvasRef.getContext('2d')
      ctx?.clearRect(0, 0, canvasRef.width, canvasRef.height)
    }
  }

  const onKeyDown = (e: KeyboardEvent) => {
    // Toggle auto-detect with 'A' key
    if (e.key === 'a' || e.key === 'A') {
      if (status() === 'selecting' && !isDragging()) {
        setAutoDetectEnabled(prev => !prev)
        if (!autoDetectEnabled()) {
          setElementRect(null)
        }
        return
      }
    }
    
    if (e.key === 'Escape') {
      if (ocrOpen()) {
        setOcrOpen(false)
        setOcrLoading(false)
        return
      }
      handleCancel()
    }
  }

  const drawCanvas = (image: HTMLImageElement) => {
    const canvas = canvasRef
    if (!canvas) {
      console.error('[Capture] drawCanvas: canvas ref is null!')
      return
    }

    const ctx = canvas.getContext('2d')
    if (!ctx) {
      console.error('[Capture] drawCanvas: failed to get 2d context!')
      return
    }

    const dpr = window.devicePixelRatio || 1
    const w = window.innerWidth
    const h = window.innerHeight
    
    console.log('[Capture] drawCanvas:', { w, h, dpr, imgW: image.width, imgH: image.height })
    
    canvas.width = Math.max(1, Math.round(w * dpr))
    canvas.height = Math.max(1, Math.round(h * dpr))
    canvas.style.width = `${w}px`
    canvas.style.height = `${h}px`

    // Draw using CSS pixels
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0)

    // Draw background image
    ctx.clearRect(0, 0, w, h)
    ctx.drawImage(image, 0, 0, w, h)

    // Draw dark overlay
    ctx.fillStyle = 'rgba(0, 0, 0, 0.4)'
    ctx.fillRect(0, 0, w, h)

    // Cut out selection area
    const sel = selection()
    if (sel && sel.w > 0 && sel.h > 0) {
      ctx.clearRect(sel.x, sel.y, sel.w, sel.h)
      // Redraw image in selection
      const scaleX = image.width / window.innerWidth
      const scaleY = image.height / window.innerHeight
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

  // Konva annotations are handled by AnnotationLayer

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
    if (ocrOpen() || ocrLoading()) return
    const st = status()
    if (st !== 'selecting' && st !== 'editing') return

    // Ignore clicks on toolbar
    const path = (e.composedPath?.() ?? []) as unknown[]
    const clickedToolbar = path.some((el) => {
      const node = el as HTMLElement
      return node?.dataset?.captureToolbar === 'true'
    })
    if (clickedToolbar) return

    // In editing mode:
    // - click inside selection: keep selection fixed (Konva/tools handle interaction)
    // - click outside selection: exit editing and clear selection so user can reselect
    if (st === 'editing') {
      const sel = selection()
      if (sel && isInsideSelection(e.clientX, e.clientY, sel)) {
        return
      }
      setStatus('selecting')
      setSelection(null)
      setCropDataUrl('')
      setAnnotationApi(null)
      return
    }

    // If clicking on a detected UI element, use it as selection
    const elem = elementRect()
    if (elem && autoDetectEnabled() && !isDragging()) {
      const elemWidth = elem.right - elem.left
      const elemHeight = elem.bottom - elem.top
      if (elemWidth >= MIN_SELECTION_SIZE && elemHeight >= MIN_SELECTION_SIZE) {
        setSelection({ x: elem.left, y: elem.top, w: elemWidth, h: elemHeight })
        setStatus('editing')
        setElementRect(null)
        return
      }
    }

    const x = e.clientX
    const y = e.clientY
    setStartPoint({ x, y })
    setSelection({ x, y, w: 0, h: 0 })
    setStatus('selecting')
    setIsDragging(true)
    setElementRect(null) // Clear element highlight when starting manual selection

    // Ensure mouseup finalizes selection even if the pointer ends outside the window.
    const onWindowMouseUp = () => onMouseUp()
    window.addEventListener('mouseup', onWindowMouseUp, { once: true })
  }

  const isInsideSelection = (x: number, y: number, sel: Selection) => {
    return x >= sel.x && x <= sel.x + sel.w && y >= sel.y && y <= sel.y + sel.h
  }

  // Throttled UI element detection
  const detectElementAt = async (screenX: number, screenY: number) => {
    const now = Date.now()
    if (now - lastDetectTime < ELEMENT_DETECT_THROTTLE || detectPending) return
    
    // Convert screen coordinates to window-local coordinates for comparison
    const offset = monitorOffset()
    const localX = screenX - offset.x
    const localY = screenY - offset.y
    
    // Smart detection: skip if mouse is still inside current element (using local coords)
    const current = elementRect()
    if (current && localX >= current.left && localX < current.right && 
        localY >= current.top && localY < current.bottom) {
      return
    }
    
    lastDetectTime = now
    detectPending = true
    
    try {
      const rect = await invoke<ElementRect | null>('get_element_rect_at', { x: screenX, y: screenY })
      if (rect && rect.right > rect.left && rect.bottom > rect.top) {
        // Convert screen coordinates to window-local coordinates
        // The elementRect needs to be in window coordinates for rendering
        setElementRect({
          left: rect.left - offset.x,
          top: rect.top - offset.y,
          right: rect.right - offset.x,
          bottom: rect.bottom - offset.y,
        })
      } else {
        setElementRect(null)
      }
    } catch (err) {
      // Silently ignore errors (e.g., when not on Windows)
      console.debug('[Capture] Element detection failed:', err)
    } finally {
      detectPending = false
    }
  }

  const onMouseMove = (e: MouseEvent) => {
    if (ocrOpen() || ocrLoading()) return
    
    const st = status()
    
    // UI element auto-detection when not dragging
    if (st === 'selecting' && !isDragging() && autoDetectEnabled()) {
      // Get screen coordinates (approximate - assumes window is at screen origin)
      // For accurate detection, we'd need the window position from Rust
      detectElementAt(e.screenX, e.screenY)
    }
    
    if (st !== 'selecting' || !isDragging()) return
    
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
    if (ocrOpen() || ocrLoading()) return
    if (status() !== 'selecting' || !isDragging()) return
    setIsDragging(false)
    
    // Check if selection is large enough
    if (hasValidSelection()) {
      setStatus('editing')
      console.log('[Capture] Selection complete, entering editing mode')
      return
    }

    // Too small: clear selection so user can try again
    setSelection(null)
  }

  // Legacy overlay handlers removed

  // Get selection image as Blob
  const getSelectionBlob = async (): Promise<Blob | null> => {
    const sel = selection()
    const image = bgImage()
    if (!sel || !image) return null

    // Prefer Konva export when available (includes annotations)
    const api = annotationApi()
    if (api) {
      try {
        const dpr = window.devicePixelRatio || 1
        // Preserve native quality; do not cap pixel ratio.
        const pixelRatio = Math.max(1, dpr)
        return await api.exportBlob({ pixelRatio })
      } catch {
        // fallback to legacy crop
      }
    }

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

    // (Konva path should have returned above)

    return new Promise((resolve) => cropCanvas.toBlob(resolve, 'image/png'))
  }

  // Get selection image for OCR (native pixel crop from original screenshot)
  // - Avoids low-res Konva export (background is window-scaled)
  // - Keeps annotations out of the OCR input (more accurate)
  const getSelectionBlobForOcr = async (): Promise<Blob | null> => {
    const sel = selection()
    const image = bgImage()
    if (!sel || !image) return null

    const scaleX = image.width / window.innerWidth
    const scaleY = image.height / window.innerHeight

    const srcX = Math.max(0, Math.round(sel.x * scaleX))
    const srcY = Math.max(0, Math.round(sel.y * scaleY))
    const srcW = Math.max(1, Math.round(sel.w * scaleX))
    const srcH = Math.max(1, Math.round(sel.h * scaleY))

    const cropCanvas = document.createElement('canvas')
    // OCR should use the original pixel crop for maximum fidelity.
    cropCanvas.width = srcW
    cropCanvas.height = srcH
    const ctx = cropCanvas.getContext('2d')
    if (!ctx) return null

    ctx.imageSmoothingEnabled = false
    ctx.drawImage(image, srcX, srcY, srcW, srcH, 0, 0, srcW, srcH)
    return new Promise((resolve) => cropCanvas.toBlob(resolve, 'image/png'))
  }

  const blobToBase64 = async (blob: Blob): Promise<string> => {
    return await new Promise((resolve, reject) => {
      const reader = new FileReader()
      reader.onerror = () => reject(new Error('Failed to read blob'))
      reader.onload = () => {
        const result = String(reader.result || '')
        const b64 = result.includes(',') ? result.split(',')[1] : result
        resolve(b64)
      }
      reader.readAsDataURL(blob)
    })
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
      // Fallback to Rust clipboard (base64 IPC avoids huge JSON arrays)
      const blob = await getSelectionBlob()
      if (!blob) return
      const base64 = await blobToBase64(blob)
      await invoke('copy_capture_base64', { imageData: base64 })
      handleCancel()
    }
  }

  const getOcrAutoCopy = async (): Promise<boolean> => {
    try {
      type Cfg = { screenshot?: { ocr_auto_copy?: boolean } }
      const cfg = await invoke<Cfg>('get_config')
      return Boolean(cfg?.screenshot?.ocr_auto_copy)
    } catch {
      return false
    }
  }

  const handleOcr = async () => {
    try {
      const blob = await getSelectionBlobForOcr()
      if (!blob) return

      const base64 = await blobToBase64(blob)
      setOcrPreviewSrc(`data:image/png;base64,${base64}`)
      setOcrText('')
      setOcrOpen(true)
      setOcrLoading(true)

      const text = await invoke<string>('recognize_text', { base64Image: base64 })
      const trimmed = (text || '').trim()
      setOcrText(trimmed ? text : '未识别到文字')

      // Auto copy (optional)
      if (await getOcrAutoCopy()) {
        try {
          await navigator.clipboard.writeText(text || '')
        } catch {
          // ignore
        }
      }
    } catch (err) {
      console.error('[Capture] OCR failed:', err)
      setOcrText('OCR 失败，请重试')
      window.alert(`OCR 失败：${String((err as any)?.message ?? err)}`)
    } finally {
      setOcrLoading(false)
    }
  }

  const handleSave = async () => {
    try {
      const blob = await getSelectionBlob()
      if (!blob) return

      const now = new Date()
      const pad = (n: number) => String(n).padStart(2, '0')
      const name = `Snip-${now.getFullYear()}${pad(now.getMonth() + 1)}${pad(now.getDate())}-${pad(now.getHours())}${pad(now.getMinutes())}${pad(now.getSeconds())}.png`

      const filePath = await save({
        filters: [{ name: 'PNG', extensions: ['png'] }],
        defaultPath: name,
      })

      if (!filePath) return

      const base64 = await blobToBase64(blob)
      await invoke('save_capture_file', { path: filePath, imageData: base64 })
      handleCancel()
    } catch (err) {
      console.error('[Capture] Save failed:', err)
    }
  }

  const handlePin = async () => {
    try {
      const sel = selection()
      if (!sel) return

      // Export the selected region including annotations.
      // Use pixelRatio=1 for PIN to keep it fast and responsive (PIN window matches selection size).
      const api = annotationApi()
      const blob = api
        ? await api.exportBlob({ pixelRatio: 1 })
        : await getSelectionBlob()
      if (!blob) return
      const base64 = await blobToBase64(blob)

      // Improve perceived performance: immediately hide the fullscreen capture overlay.
      resetState()
      try {
        await invoke('hide_capture_window')
      } catch {
        // ignore
      }

      // Create the pin directly from the exported image (already cropped + annotated).
      await invoke('create_pin_window', {
        imageData: base64,
        width: Math.round(sel.w),
        height: Math.round(sel.h),
        x: Math.round(sel.x),
        y: Math.round(sel.y),
      })
    } catch (err) {
      console.error('[Capture] Pin failed:', err)
      window.alert(`Pin 失败：${String((err as any)?.message ?? err)}`)
    }
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
      class={`relative h-screen w-screen select-none overflow-hidden bg-black ${cursorClass()}`}
      onMouseDown={onMouseDown}
      onMouseMove={onMouseMove}
      onMouseUp={onMouseUp}
    >
      {/* Main canvas */}
      <Show when={status() !== 'idle'}>
        <canvas
          ref={canvasRef}
          class="absolute left-0 top-0 h-full w-full"
        />
      </Show>

      {/* Konva annotation stage, constrained to selection */}
      <Show when={status() === 'editing' && hasValidSelection() && cropDataUrl()}>
        <div
          class={`absolute z-40 ${ocrOpen() || ocrLoading() ? 'pointer-events-none opacity-90' : ''}`}
          style={{
            left: `${selection()!.x}px`,
            top: `${selection()!.y}px`,
            width: `${selection()!.w}px`,
            height: `${selection()!.h}px`,
          }}
          onMouseDown={(e) => e.stopPropagation()}
        >
          <AnnotationLayer
            width={Math.round(selection()!.w)}
            height={Math.round(selection()!.h)}
            backgroundDataUrl={cropDataUrl()}
            onApi={(api) => {
              setAnnotationApi(api)
              api.setTool(annotationTool())
              api.setStyle(annotationStyle())
            }}
          />
        </div>
      </Show>

      {/* UI Element auto-detection highlight */}
      <Show when={status() === 'selecting' && !isDragging() && elementRect() && autoDetectEnabled()}>
        {(() => {
          const rect = elementRect()!
          const width = rect.right - rect.left
          const height = rect.bottom - rect.top
          return (
            <div
              class="pointer-events-none absolute border-2 border-green-400 bg-green-400/10"
              style={{
                left: `${rect.left}px`,
                top: `${rect.top}px`,
                width: `${width}px`,
                height: `${height}px`,
              }}
            >
              {/* Size indicator for detected element */}
              <div class="absolute -top-6 left-0 rounded bg-green-600/90 px-1.5 py-0.5 text-xs text-white font-mono">
                {width} × {height}
              </div>
            </div>
          )
        })()}
      </Show>

      {/* Annotation tools toolbar */}
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
          tool={annotationTool()}
          onToolChange={(t) => {
            setAnnotationTool(t)
            annotationApi()?.setTool(t)
          }}
          style={annotationStyle()}
          onStyleChange={(patch) => {
            setAnnotationStyle((prev) => ({ ...prev, ...patch }))
            annotationApi()?.setStyle(patch)
            // Snipaste-like: if a shape is selected, apply immediately
            annotationApi()?.applyStyleToSelection()
          }}
          onApplyStyleToSelection={() => annotationApi()?.applyStyleToSelection()}
          onUndo={() => annotationApi()?.undo()}
          onRedo={() => annotationApi()?.redo()}
          canUndo={annotationApi()?.canUndo() ?? false}
          canRedo={annotationApi()?.canRedo() ?? false}
          onBrushThin={() => annotationApi()?.setBrushWidth(2)}
          onBrushThick={() => annotationApi()?.setBrushWidth(6)}
          onCopy={handleCopy}
          onOcr={handleOcr}
          ocrLoading={ocrLoading()}
          onSave={handleSave}
          onPin={handlePin}
          onCancel={handleCancel}
        />
      </Show>

      <OCRResult
        open={ocrOpen()}
        loading={ocrLoading()}
        previewSrc={ocrPreviewSrc()}
        text={ocrText()}
        onTextChange={setOcrText}
        onCopy={async () => {
          try {
            await navigator.clipboard.writeText(ocrText())
          } catch (err) {
            console.error('[Capture] Copy text failed:', err)
          }
        }}
        onTranslate={() => {
          // reserved
        }}
        onClose={() => {
          if (ocrLoading()) return
          setOcrOpen(false)
        }}
      />

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
