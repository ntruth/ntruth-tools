import { Component, createSignal, onMount, Show } from 'solid-js'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'

/**
 * PinPage - A floating screenshot window that can be dragged around
 * 
 * Query params:
 * - data: base64 encoded PNG image
 * - w: width
 * - h: height
 */
const PinPage: Component = () => {
  const [imageUrl, setImageUrl] = createSignal<string>('')
  
  const currentWindow = getCurrentWindow()

  onMount(() => {
    // Parse query params from URL
    const params = new URLSearchParams(window.location.search)
    const data = params.get('data')
    const w = params.get('w')
    const h = params.get('h')
    
    console.log('[Pin] Mounting with params:', { w, h, dataLength: data?.length })
    
    if (data) setImageUrl(`data:image/png;base64,${data}`)

    // Also support event-based payload (preferred)
    const unlistenPromise = listen<{ data: string }>('pin:set_image', (event) => {
      if (event.payload?.data) {
        setImageUrl(`data:image/png;base64,${event.payload.data}`)
      }
    })

    // Cleanup
    unlistenPromise.then((unlisten) => {
      // Pin window lives long; still clean on unload
      window.addEventListener('beforeunload', () => unlisten())
    })
  })

  // Start dragging - uses Tauri's native window drag
  const onMouseDown = async (e: MouseEvent) => {
    if (e.button !== 0) return // Only left click
    
    // Use Tauri's native window drag
    try {
      await currentWindow.startDragging()
    } catch (err) {
      console.warn('[Pin] Drag failed:', err)
    }
  }

  // Close pin window
  const handleClose = async (e: MouseEvent) => {
    e.stopPropagation()
    await currentWindow.close()
  }

  // Copy image to clipboard
  const handleCopy = async (e: MouseEvent) => {
    e.stopPropagation()
    const img = imageUrl()
    if (!img) return
    
    try {
      // Get the image data and copy to clipboard
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

  return (
    <div
      class="group relative h-full w-full cursor-move select-none overflow-hidden bg-transparent"
      onMouseDown={onMouseDown}
    >
      {/* Image */}
      <Show when={imageUrl()}>
        <img
          src={imageUrl()}
          alt="Pinned screenshot"
          class="h-full w-full object-contain"
          draggable={false}
        />
      </Show>

      {/* Toolbar - visible on hover */}
      <div class="absolute right-1 top-1 flex gap-1 rounded-md bg-black/60 p-1 opacity-0 transition-opacity group-hover:opacity-100">
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
    </div>
  )
}

export default PinPage
