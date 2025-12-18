import { Component, createSignal, onMount, onCleanup, Show } from 'solid-js'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { LogicalSize } from '@tauri-apps/api/dpi'

/**
 * PinPage - A floating screenshot window with advanced interactions
 * 
 * Features:
 * - Scroll wheel: Scale window (0.1x - 5x)
 * - Ctrl + Scroll: Adjust opacity (10% - 100%)
 * - Ctrl + T: Toggle click-through mode
 * - Double-click: Close window
 * - Drag: Move window
 */
const PinPage: Component = () => {
  const [imageUrl, setImageUrl] = createSignal<string>('')
  const [scale, setScale] = createSignal(1)
  const [opacity, setOpacity] = createSignal(1)
  const [isClickThrough, setIsClickThrough] = createSignal(false)
  const [originalSize, setOriginalSize] = createSignal({ width: 100, height: 100 })
  
  let imageRef: HTMLImageElement | undefined
  const currentWindow = getCurrentWebviewWindow()

  onMount(() => {
    // Parse query params from URL
    const params = new URLSearchParams(window.location.search)
    const data = params.get('data')
    const w = params.get('w')
    const h = params.get('h')
    
    console.log('[Pin] Mounting with params:', { w, h, dataLength: data?.length })
    
    if (data) setImageUrl(`data:image/png;base64,${data}`)
    if (w && h) setOriginalSize({ width: parseInt(w), height: parseInt(h) })

    // Also support event-based payload (preferred)
    const unlistenPromise = listen<{ data: string }>('pin:set_image', (event) => {
      if (event.payload?.data) {
        setImageUrl(`data:image/png;base64,${event.payload.data}`)
      }
    })

    // Reliable fallback: pull payload from backend by window label
    ;(async () => {
      try {
        const label = currentWindow.label
        const payload = await invoke<{ data: string; width: number; height: number } | null>('get_pin_payload', {
          label,
        })
        if (payload?.data) {
          setImageUrl(`data:image/png;base64,${payload.data}`)
          setOriginalSize({ width: payload.width, height: payload.height })
        }
      } catch (e) {
        console.warn('[Pin] get_pin_payload failed:', e)
      }
    })()

    // Keyboard shortcuts
    const handleKeyDown = async (e: KeyboardEvent) => {
      if (e.ctrlKey && (e.key === 't' || e.key === 'T')) {
        e.preventDefault()
        await toggleClickThrough()
      }
      if (e.key === 'Escape') {
        await closeSelf()
      }
    }
    window.addEventListener('keydown', handleKeyDown)

    // Cleanup
    unlistenPromise.then((unlisten) => {
      window.addEventListener('beforeunload', () => unlisten())
    })
    
    onCleanup(() => {
      window.removeEventListener('keydown', handleKeyDown)
    })
  })

  const closeSelf = async () => {
    try {
      await invoke('close_pin_window', { label: currentWindow.label })
      return
    } catch (err) {
      console.warn('[Pin] close_pin_window failed, fallback to window.close():', err)
    }

    try {
      await currentWindow.close()
    } catch (err) {
      console.warn('[Pin] Close failed:', err)
    }
  }

  const toggleClickThrough = async () => {
    const newState = !isClickThrough()
    setIsClickThrough(newState)
    try {
      await currentWindow.setIgnoreCursorEvents(newState)
      console.log('[Pin] Click-through:', newState)
    } catch (err) {
      console.warn('[Pin] setIgnoreCursorEvents failed:', err)
    }
  }

  const handleWheel = async (e: WheelEvent) => {
    e.preventDefault()
    e.stopPropagation()
    
    if (e.ctrlKey) {
      // Ctrl + Wheel: Adjust opacity
      const delta = e.deltaY > 0 ? -0.1 : 0.1
      const newOpacity = Math.max(0.1, Math.min(1, opacity() + delta))
      setOpacity(newOpacity)
      
      if (imageRef) {
        imageRef.style.opacity = newOpacity.toString()
      }
      console.log('[Pin] Opacity:', newOpacity.toFixed(1))
    } else {
      // Wheel: Adjust scale
      const delta = e.deltaY > 0 ? 0.9 : 1.1
      const newScale = Math.max(0.1, Math.min(5, scale() * delta))
      setScale(newScale)
      
      const newWidth = Math.round(originalSize().width * newScale)
      const newHeight = Math.round(originalSize().height * newScale)
      
      try {
        await currentWindow.setSize(new LogicalSize(newWidth, newHeight))
        console.log('[Pin] Scale:', newScale.toFixed(2), 'Size:', newWidth, 'x', newHeight)
      } catch (err) {
        console.warn('[Pin] setSize failed:', err)
      }
    }
  }

  const handleDoubleClick = async (e: MouseEvent) => {
    e.stopPropagation()
    await closeSelf()
  }

  const handlePointerDown = async (e: PointerEvent) => {
    if ((e as any).button !== 0) return
    const target = e.target as HTMLElement | null
    if (target?.closest?.('[data-pin-toolbar="true"]')) return

    try {
      await currentWindow.startDragging()
    } catch {
      // ignore
    }
  }

  const handleClose = async (e: MouseEvent) => {
    e.stopPropagation()
    await closeSelf()
  }

  const handleCopy = async (e: MouseEvent) => {
    e.stopPropagation()
    const img = imageUrl()
    if (!img) return
    
    try {
      const response = await fetch(img)
      const blob = await response.blob()
      await navigator.clipboard.write([
        new ClipboardItem({ 'image/png': blob })
      ])
      console.log('[Pin] Image copied to clipboard')
    } catch (err) {
      console.error('[Pin] Copy failed:', err)
    }
  }

  const handleToggleClickThrough = async (e: MouseEvent) => {
    e.stopPropagation()
    await toggleClickThrough()
  }

  return (
    <div
      class="group relative h-full w-full cursor-move select-none overflow-hidden bg-transparent"
      data-tauri-drag-region
      onDblClick={handleDoubleClick}
      onPointerDown={handlePointerDown}
      onWheel={handleWheel}
    >
      {/* Near-invisible background for hit-testing on transparent windows */}
      <div class="absolute inset-0" style={{ background: 'rgba(0,0,0,0.01)' }} />

      {/* Image */}
      <Show when={imageUrl()}>
        <img
          ref={imageRef}
          src={imageUrl()}
          alt="Pinned screenshot"
          class="h-full w-full object-contain transition-opacity duration-150"
          draggable={false}
          onDblClick={handleDoubleClick}
        />
      </Show>

      {/* Click-through indicator */}
      <Show when={isClickThrough()}>
        <div class="absolute left-1 top-1 rounded bg-yellow-500/80 px-1.5 py-0.5 text-xs font-medium text-black">
          穿透模式
        </div>
      </Show>

      {/* Status bar - scale and opacity */}
      <div class="absolute bottom-1 left-1 flex gap-2 rounded bg-black/60 px-2 py-0.5 text-xs text-white/80 opacity-0 transition-opacity group-hover:opacity-100">
        <span>{Math.round(scale() * 100)}%</span>
        <span>·</span>
        <span>透明度 {Math.round(opacity() * 100)}%</span>
      </div>

      {/* Toolbar - visible on hover */}
      <div
        class="absolute right-1 top-1 flex gap-1 rounded-md bg-black/60 p-1 opacity-0 transition-opacity group-hover:opacity-100"
        data-tauri-drag-region="false"
        data-pin-toolbar="true"
      >
        {/* Toggle click-through */}
        <button
          class={`flex h-6 w-6 items-center justify-center rounded transition-colors ${
            isClickThrough() 
              ? 'bg-yellow-500/80 text-black' 
              : 'text-white/80 hover:bg-white/20 hover:text-white'
          }`}
          onClick={handleToggleClickThrough}
          title="切换鼠标穿透 (Ctrl+T)"
        >
          <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 15l-2 5L9 9l11 4-5 2zm0 0l5 5M7.188 2.239l.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656l-2.12 2.122" />
          </svg>
        </button>
        
        {/* Copy button */}
        <button
          class="flex h-6 w-6 items-center justify-center rounded text-white/80 hover:bg-white/20 hover:text-white"
          onClick={handleCopy}
          title="复制到剪贴板"
        >
          <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
          </svg>
        </button>
        
        {/* Close button */}
        <button
          class="flex h-6 w-6 items-center justify-center rounded text-white/80 hover:bg-red-500/80 hover:text-white"
          onClick={handleClose}
          title="关闭"
        >
          <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      {/* Border effect on hover */}
      <div class="pointer-events-none absolute inset-0 rounded border-2 border-transparent transition-colors group-hover:border-blue-400/50" />
      
      {/* Resize hint on corners */}
      <div class="pointer-events-none absolute bottom-0 right-0 h-3 w-3 opacity-0 transition-opacity group-hover:opacity-50">
        <svg class="h-full w-full text-white" viewBox="0 0 12 12">
          <path fill="currentColor" d="M12 0v12H0L12 0z"/>
        </svg>
      </div>
    </div>
  )
}

export default PinPage
