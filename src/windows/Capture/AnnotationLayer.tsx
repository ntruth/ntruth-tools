import { Component, onCleanup, onMount, createEffect } from 'solid-js'
import { DrawManager, type DrawManagerApi, type ToolType, type AnnotationStyle } from './DrawManager'

export interface AnnotationLayerApi {
  setTool: (tool: ToolType) => void
  setBrushWidth: (w: number) => void
  setStyle: (patch: Partial<AnnotationStyle>) => void
  getStyle: () => AnnotationStyle
  applyStyleToSelection: () => void
  undo: () => void
  redo: () => void
  canUndo: () => boolean
  canRedo: () => boolean
  exportBlob: (opts?: { pixelRatio?: number }) => Promise<Blob>
}

export interface AnnotationLayerProps {
  width: number
  height: number
  backgroundDataUrl: string

  /** Receives the API when ready */
  onApi?: (api: AnnotationLayerApi) => void
}

/**
 * AnnotationLayer
 * - Creates a Konva stage with background + drawing layer
 * - Delegates all interaction + history to DrawManager
 *
 * High-performance notes:
 * - Pointer move is RAF-throttled in DrawManager
 * - Layer updates use batchDraw
 */
const AnnotationLayer: Component<AnnotationLayerProps> = (props) => {
  let containerRef: HTMLDivElement | undefined
  let manager: DrawManagerApi | null = null

  onMount(() => {
    if (!containerRef) return

    manager = new DrawManager({
      container: containerRef,
      width: props.width,
      height: props.height,
      backgroundDataUrl: props.backgroundDataUrl,
    })

    props.onApi?.({
      setTool: manager.setTool.bind(manager),
      setBrushWidth: manager.setBrushWidth.bind(manager),
      setStyle: manager.setStyle.bind(manager),
      getStyle: manager.getStyle.bind(manager),
      applyStyleToSelection: manager.applyStyleToSelection.bind(manager),
      undo: manager.undo.bind(manager),
      redo: manager.redo.bind(manager),
      canUndo: manager.canUndo.bind(manager),
      canRedo: manager.canRedo.bind(manager),
      exportBlob: manager.exportBlob.bind(manager),
    })

    onCleanup(() => {
      manager?.destroy()
      manager = null
    })
  })

  createEffect(() => {
    // Keep container size in sync (CSS) even though Konva uses internal width/height
    if (!containerRef) return
    containerRef.style.width = `${props.width}px`
    containerRef.style.height = `${props.height}px`
  })

  return <div ref={containerRef} class="absolute left-0 top-0" />
}

export default AnnotationLayer
