import { Component, createEffect, createMemo, onCleanup } from 'solid-js'

export interface MagnifierProps {
  /** Source canvas that already contains the captured screen image at window size */
  sourceCanvas: () => HTMLCanvasElement | undefined

  /** Virtual cursor position (in window coordinates) */
  cursor: () => { x: number; y: number }

  /** Whether magnifier is enabled (e.g., Alt held) */
  enabled: () => boolean

  /** Pixel zoom factor */
  zoom?: number

  /** Canvas size in px (square) */
  size?: number
}

const clamp = (v: number, min: number, max: number) => Math.max(min, Math.min(max, v))

/**
 * Snipaste-like magnifier (pixel preview) drawn near the cursor.
 *
 * Rendering strategy:
 * - Samples a small square around (cursorX,cursorY) from `sourceCanvas`
 * - Draws it scaled up on a 120x120 canvas
 * - Overlays crosshair + RGB value
 */
const Magnifier: Component<MagnifierProps> = (props) => {
  const zoom = createMemo(() => props.zoom ?? 4)
  const size = createMemo(() => props.size ?? 120)

  let canvasRef: HTMLCanvasElement | undefined

  const draw = () => {
    if (!props.enabled()) return
    const dst = canvasRef
    const src = props.sourceCanvas()
    if (!dst || !src) return

    const dstCtx = dst.getContext('2d')
    const srcCtx = src.getContext('2d', { willReadFrequently: true })
    if (!dstCtx || !srcCtx) return

    const s = size()
    const z = zoom()
    const sampleSize = Math.floor(s / z) // e.g. 30 when s=120,z=4
    const half = Math.floor(sampleSize / 2)

    const { x, y } = props.cursor()

    const sx = clamp(Math.round(x) - half, 0, Math.max(0, src.width - sampleSize))
    const sy = clamp(Math.round(y) - half, 0, Math.max(0, src.height - sampleSize))

    // Copy a small block then scale to destination
    const block = srcCtx.getImageData(sx, sy, sampleSize, sampleSize)
    // Put into an offscreen canvas to scale with nearest-neighbor
    const off = document.createElement('canvas')
    off.width = sampleSize
    off.height = sampleSize
    const offCtx = off.getContext('2d')
    if (!offCtx) return
    offCtx.putImageData(block, 0, 0)

    dst.width = s
    dst.height = s

    dstCtx.imageSmoothingEnabled = false
    dstCtx.clearRect(0, 0, s, s)
    dstCtx.drawImage(off, 0, 0, sampleSize, sampleSize, 0, 0, s, s)

    // Crosshair at center
    const c = Math.floor(s / 2)
    dstCtx.strokeStyle = 'rgba(255,255,255,0.95)'
    dstCtx.lineWidth = 1
    dstCtx.beginPath()
    dstCtx.moveTo(c + 0.5, 0)
    dstCtx.lineTo(c + 0.5, s)
    dstCtx.moveTo(0, c + 0.5)
    dstCtx.lineTo(s, c + 0.5)
    dstCtx.stroke()

    // Sample center pixel color (from block)
    const cx = clamp(Math.round(x) - sx, 0, sampleSize - 1)
    const cy = clamp(Math.round(y) - sy, 0, sampleSize - 1)
    const i = (cy * sampleSize + cx) * 4
    const r = block.data[i]
    const g = block.data[i + 1]
    const b = block.data[i + 2]

    // RGB label background
    const label = `RGB(${r}, ${g}, ${b})`
    dstCtx.font = '12px ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace'
    const padX = 6
    const padY = 4
    const metrics = dstCtx.measureText(label)
    const w = Math.ceil(metrics.width) + padX * 2
    const h = 18

    dstCtx.fillStyle = 'rgba(0,0,0,0.65)'
    dstCtx.fillRect(6, s - h - 6, w, h)
    dstCtx.fillStyle = 'rgba(255,255,255,0.95)'
    dstCtx.fillText(label, 6 + padX, s - 6 - padY)

    // Frame
    dstCtx.strokeStyle = 'rgba(255,255,255,0.35)'
    dstCtx.strokeRect(0.5, 0.5, s - 1, s - 1)
  }

  // Redraw whenever cursor/enabled changes
  createEffect(() => {
    props.enabled()
    props.cursor()
    props.sourceCanvas()
    // RAF to coalesce multiple signals in one paint
    const id = requestAnimationFrame(draw)
    onCleanup(() => cancelAnimationFrame(id))
  })

  return (
    <canvas
      ref={canvasRef}
      class="pointer-events-none"
      width={size()}
      height={size()}
    />
  )
}

export default Magnifier
