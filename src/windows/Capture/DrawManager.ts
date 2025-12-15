import Konva from 'konva'
import { createMosaicNode } from './MosaicTool'

export type ToolType =
  | 'select'
  | 'rect'
  | 'ellipse'
  | 'line'
  | 'arrow'
  | 'pencil'
  | 'marker'
  | 'text'
  | 'mosaic'

export interface DrawManagerOptions {
  container: HTMLDivElement
  width: number
  height: number
  backgroundDataUrl: string
}

export interface DrawManagerApi {
  setTool: (tool: ToolType) => void
  getTool: () => ToolType
  setBrushWidth: (w: number) => void
  getBrushWidth: () => number

  setStyle: (patch: Partial<AnnotationStyle>) => void
  getStyle: () => AnnotationStyle
  applyStyleToSelection: () => void

  undo: () => void
  redo: () => void
  canUndo: () => boolean
  canRedo: () => boolean
  exportBlob: (opts?: { pixelRatio?: number }) => Promise<Blob>
  destroy: () => void

  // optional hooks for external UI
  onSelectionChanged?: (node: Konva.Node | null) => void
}

const clamp = (v: number, min: number, max: number) => Math.max(min, Math.min(max, v))

export interface AnnotationStyle {
  stroke: string
  opacity: number
  fillEnabled: boolean
  fill: string
  strokeWidth: number
  dashEnabled: boolean
  dash: number[]
  arrowMode: 'end' | 'start' | 'both'
  arrowHeadStyle: 'filled' | 'outline'
  arrowPointerLength: number
  arrowPointerWidth: number
  fontFamily: string
  textBold: boolean
  textItalic: boolean
  fontSize: number
  textBgEnabled: boolean
  textBgColor: string
  textBgOpacity: number
  textPadding: number
  textBgRadius: number
  mosaicPixelSize: number
}

export class DrawManager implements DrawManagerApi {
  private readonly container: HTMLDivElement
  private stage: Konva.Stage
  private bgLayer: Konva.Layer
  private drawLayer: Konva.Layer
  private transformer: Konva.Transformer
  private backgroundImageEl: HTMLImageElement
  private backgroundNode: Konva.Image

  private tool: ToolType = 'select'
  private brushWidth = 3

  private style: AnnotationStyle = {
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
  }

  private isSpaceDown = false
  private isShiftDown = false

  private isDrawing = false
  private activeNode: Konva.Node | null = null
  private activePoints: number[] = []
  private startPoint: { x: number; y: number } | null = null

  private rafPending = false
  private lastPointer: { x: number; y: number } | null = null

  private undoStack: string[] = []
  private redoStack: string[] = []

  onSelectionChanged?: (node: Konva.Node | null) => void

  constructor(opts: DrawManagerOptions) {
    this.container = opts.container

    this.stage = new Konva.Stage({
      container: opts.container,
      width: opts.width,
      height: opts.height,
      draggable: false,
    })

    this.bgLayer = new Konva.Layer({ listening: false })
    this.drawLayer = new Konva.Layer({ listening: true })

    this.stage.add(this.bgLayer)
    this.stage.add(this.drawLayer)

    this.backgroundImageEl = new Image()
    this.backgroundNode = new Konva.Image({
      x: 0,
      y: 0,
      width: opts.width,
      height: opts.height,
      // Konva v9 typings require `image` in config; will be updated onload as well.
      image: this.backgroundImageEl,
      listening: false,
    })
    this.bgLayer.add(this.backgroundNode)

    this.transformer = new Konva.Transformer({
      rotateEnabled: true,
      ignoreStroke: true,
      enabledAnchors: [
        'top-left',
        'top-right',
        'bottom-left',
        'bottom-right',
        'middle-left',
        'middle-right',
        'top-center',
        'bottom-center',
      ],
    })
    this.drawLayer.add(this.transformer)

    this.bindStageEvents()
    this.bindKeyboard()

    // Load background
    this.backgroundImageEl.onload = () => {
      this.backgroundNode.image(this.backgroundImageEl)
      this.bgLayer.batchDraw()
      this.commitHistory(true)
    }
    this.backgroundImageEl.src = opts.backgroundDataUrl

    // High perf: disable auto draw on drag; use batchDraw
    this.stage.on('contentMousemove', () => {
      /* noop; events bound explicitly */
    })
  }

  destroy() {
    this.cleanupKeyboard()
    this.stage.destroy()
  }

  setTool(tool: ToolType) {
    this.tool = tool
    // Cursor update
    const cursor = this.getCursorForTool(tool)
    this.container.style.cursor = cursor

    if (tool !== 'select') {
      this.clearSelection()
    }
  }

  getTool() {
    return this.tool
  }

  setBrushWidth(w: number) {
    this.brushWidth = clamp(w, 1, 50)
    this.style.strokeWidth = this.brushWidth
  }

  getBrushWidth() {
    return this.brushWidth
  }

  setStyle(patch: Partial<AnnotationStyle>) {
    this.style = { ...this.style, ...patch }
    // keep legacy brushWidth in sync
    if (typeof patch.strokeWidth === 'number') {
      this.brushWidth = clamp(patch.strokeWidth, 1, 50)
    }
    // If fill is enabled and no explicit fill provided, follow stroke
    if (patch.stroke && this.style.fillEnabled && !patch.fill) {
      this.style.fill = patch.stroke
    }

    if (typeof patch.opacity === 'number') {
      this.style.opacity = clamp(patch.opacity, 0.05, 1)
    }

    if (typeof patch.textBgOpacity === 'number') {
      this.style.textBgOpacity = clamp(patch.textBgOpacity, 0, 1)
    }
  }

  getStyle() {
    return { ...this.style }
  }

  applyStyleToSelection() {
    const node = this.transformer.nodes()[0]
    if (!node) return
    this.applyStyleToNode(node)
    this.drawLayer.batchDraw()
    this.commitHistory()
  }

  private applyStyleToNode(node: Konva.Node) {
    const s = this.style

    const fontStyle = s.textBold && s.textItalic ? 'bold italic' : s.textBold ? 'bold' : s.textItalic ? 'italic' : 'normal'

    // Text background entity is stored as a Group (Rect + Text)
    if (node instanceof Konva.Group && node.name() === 'textGroup') {
      const text = node.findOne('.textNode') as Konva.Text | null
      const bg = node.findOne('.textBg') as Konva.Rect | null
      if (text) {
        text.fill(s.stroke)
        text.fontFamily(s.fontFamily)
        text.fontStyle(fontStyle)
        text.fontSize(s.fontSize)
        text.padding(s.textPadding)
      }
      if (bg) {
        bg.visible(s.textBgEnabled)
        bg.fill(s.textBgColor)
        bg.opacity(s.textBgOpacity)
        bg.cornerRadius(s.textBgRadius)
      }
      node.opacity(s.opacity)
      this.syncTextGroupLayout(node)
      return
    }

    if (node instanceof Konva.Text) {
      node.fill(s.stroke)
      node.fontFamily(s.fontFamily)
      node.fontStyle(fontStyle)
      node.fontSize(s.fontSize)
      node.opacity(s.opacity)
      return
    }

    if (node instanceof Konva.Arrow) {
      node.stroke(s.stroke)
      node.fill(s.arrowHeadStyle === 'filled' ? s.stroke : 'rgba(0,0,0,0)')
      node.strokeWidth(s.strokeWidth)
      node.dash(s.dashEnabled ? s.dash : [])
      node.pointerLength(s.arrowPointerLength)
      node.pointerWidth(s.arrowPointerWidth)
      node.pointerAtBeginning(s.arrowMode === 'start' || s.arrowMode === 'both')
      node.pointerAtEnding(s.arrowMode === 'end' || s.arrowMode === 'both')
      node.opacity(s.opacity)
      return
    }

    if (node instanceof Konva.Line) {
      node.stroke(s.stroke)
      node.strokeWidth(s.strokeWidth)
      node.dash(s.dashEnabled ? s.dash : [])
      node.opacity(s.opacity)
      return
    }

    if (node instanceof Konva.Rect || node instanceof Konva.Ellipse) {
      ;(node as any).stroke?.(s.stroke)
      ;(node as any).strokeWidth?.(s.strokeWidth)
      ;(node as any).dash?.(s.dashEnabled ? s.dash : [])
      if (s.fillEnabled) {
        ;(node as any).fill?.(s.fill)
      } else {
        ;(node as any).fill?.('rgba(0,0,0,0)')
      }
      ;(node as any).opacity?.(s.opacity)
      return
    }
  }

  canUndo() {
    return this.undoStack.length > 1
  }

  canRedo() {
    return this.redoStack.length > 0
  }

  undo() {
    if (!this.canUndo()) return
    const current = this.undoStack.pop()
    if (current) this.redoStack.push(current)
    const prev = this.undoStack[this.undoStack.length - 1]
    if (prev) this.restoreFromSnapshot(prev)
  }

  redo() {
    if (!this.canRedo()) return
    const next = this.redoStack.pop()
    if (!next) return
    this.undoStack.push(next)
    this.restoreFromSnapshot(next)
  }

  async exportBlob(opts?: { pixelRatio?: number }): Promise<Blob> {
    const pixelRatio = Math.max(1, Math.min(3, Number(opts?.pixelRatio ?? 1)))
    const canvas = this.stage.toCanvas({ pixelRatio })
    return new Promise((resolve, reject) => {
      canvas.toBlob((blob: Blob | null) => {
        if (!blob) reject(new Error('Failed to export blob'))
        else resolve(blob)
      }, 'image/png')
    })
  }

  private bindStageEvents() {
    this.stage.on('mousedown touchstart', (evt: Konva.KonvaEventObject<MouseEvent | TouchEvent>) => this.onPointerDown(evt))
    this.stage.on('mousemove touchmove', (evt: Konva.KonvaEventObject<MouseEvent | TouchEvent>) => this.onPointerMove(evt))
    this.stage.on('mouseup touchend', (evt: Konva.KonvaEventObject<MouseEvent | TouchEvent>) => this.onPointerUp(evt))
    this.stage.on('click tap', (evt: Konva.KonvaEventObject<MouseEvent | TouchEvent>) => this.onClick(evt))

    // Commit after transform
    this.drawLayer.on('transformend', () => this.commitHistory())
    this.drawLayer.on('dragend', () => this.commitHistory())
  }

  private bindKeyboard() {
    const isTextInputActive = () => {
      const el = document.activeElement as HTMLElement | null
      if (!el) return false
      const tag = el.tagName
      if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') return true
      return (el as any).isContentEditable === true
    }

    const onKeyDown = (e: KeyboardEvent) => {
      // Don't steal keys from text editing/IME
      if (isTextInputActive() || (e as any).isComposing) return

      if (e.key === 'Shift') {
        this.isShiftDown = true
        this.transformer.rotationSnaps(this.isShiftDown ? [0, 45, 90, 135, 180, 225, 270, 315] : [])
        this.drawLayer.batchDraw()
        return
      }

      if (e.key === ' ') {
        e.preventDefault()
        if (!this.isSpaceDown) {
          this.isSpaceDown = true
          this.stage.draggable(true)
          this.container.style.cursor = 'grab'
        }
        return
      }

      // Undo/Redo
      if ((e.ctrlKey || e.metaKey) && (e.key === 'z' || e.key === 'Z')) {
        e.preventDefault()
        this.undo()
        return
      }
      if ((e.ctrlKey || e.metaKey) && (e.key === 'y' || e.key === 'Y')) {
        e.preventDefault()
        this.redo()
        return
      }

      // Tab toggles line <-> arrow
      if (e.key === 'Tab') {
        if (this.tool === 'line') this.setTool('arrow')
        else if (this.tool === 'arrow') this.setTool('line')
        e.preventDefault()
        return
      }

      // 1 / 2 brush width quick toggle
      if (e.key === '1') {
        this.setBrushWidth(2)
        return
      }
      if (e.key === '2') {
        this.setBrushWidth(6)
        return
      }
    }

    const onKeyUp = (e: KeyboardEvent) => {
      if (isTextInputActive() || (e as any).isComposing) return
      if (e.key === 'Shift') {
        this.isShiftDown = false
        this.transformer.rotationSnaps([])
        this.drawLayer.batchDraw()
        return
      }
      if (e.key === ' ') {
        this.isSpaceDown = false
        this.stage.draggable(false)
        this.container.style.cursor = this.getCursorForTool(this.tool)
      }
    }

    window.addEventListener('keydown', onKeyDown, { capture: true })
    window.addEventListener('keyup', onKeyUp, { capture: true })

    // store for cleanup
    ;(this as any)._kmCleanup = () => {
      window.removeEventListener('keydown', onKeyDown, { capture: true } as any)
      window.removeEventListener('keyup', onKeyUp, { capture: true } as any)
    }
  }

  private cleanupKeyboard() {
    const fn = (this as any)._kmCleanup
    if (typeof fn === 'function') fn()
  }

  private getCursorForTool(tool: ToolType) {
    switch (tool) {
      case 'select':
        return 'default'
      case 'text':
        return 'text'
      case 'pencil':
      case 'marker':
      case 'rect':
      case 'ellipse':
      case 'line':
      case 'arrow':
      case 'mosaic':
        return 'crosshair'
      default:
        return 'default'
    }
  }

  private getPointerPosition() {
    const pos = this.stage.getPointerPosition()
    if (!pos) return null
    return { x: pos.x, y: pos.y }
  }

  private onPointerDown(evt: Konva.KonvaEventObject<MouseEvent | TouchEvent>) {
    if (this.isSpaceDown) return

    const pos = this.getPointerPosition()
    if (!pos) return

    // Text tool: click to create OR edit text
    if (this.tool === 'text') {
      const target = evt.target

      // If clicking on an existing text entity (Group or Text node), edit it instead of creating new
      if (target && target !== this.stage && target !== this.backgroundNode) {
        // Check if it's a textGroup or inside one
        const textGroup =
          target instanceof Konva.Group && target.name() === 'textGroup'
            ? target
            : target instanceof Konva.Text || target instanceof Konva.Rect
              ? (target.getParent() as Konva.Group | null)
              : null

        if (textGroup && textGroup.name() === 'textGroup') {
          this.editTextEntity(textGroup)
          return
        }

        // If clicking directly on a standalone Text node
        if (target instanceof Konva.Text) {
          this.editTextNode(target)
          return
        }
      }

      // Otherwise, create a new text at this position
      const g = this.createTextAt(pos.x, pos.y)
      // Snipaste-like: immediately enter editing after placing
      this.editTextEntity(g)
      return
    }

    // Select tool: selection handled on click
    if (this.tool === 'select') return

    this.isDrawing = true
    this.startPoint = pos
    this.activePoints = [pos.x, pos.y]

    const color = this.style.stroke

    switch (this.tool) {
      case 'rect': {
        const rect = new Konva.Rect({
          x: pos.x,
          y: pos.y,
          width: 0,
          height: 0,
          stroke: color,
          strokeWidth: this.style.strokeWidth,
          fill: this.style.fillEnabled ? this.style.fill : 'rgba(0,0,0,0)',
          dash: this.style.dashEnabled ? this.style.dash : [],
          opacity: this.style.opacity,
          fillEnabled: false,
          draggable: true,
        })
        this.drawLayer.add(rect)
        this.activeNode = rect
        break
      }
      case 'ellipse': {
        const ellipse = new Konva.Ellipse({
          x: pos.x,
          y: pos.y,
          radiusX: 0,
          radiusY: 0,
          stroke: color,
          strokeWidth: this.style.strokeWidth,
          fill: this.style.fillEnabled ? this.style.fill : 'rgba(0,0,0,0)',
          dash: this.style.dashEnabled ? this.style.dash : [],
          opacity: this.style.opacity,
          draggable: true,
        })
        this.drawLayer.add(ellipse)
        this.activeNode = ellipse
        break
      }
      case 'line': {
        const line = new Konva.Line({
          points: [pos.x, pos.y, pos.x, pos.y],
          stroke: color,
          strokeWidth: this.style.strokeWidth,
          dash: this.style.dashEnabled ? this.style.dash : [],
          opacity: this.style.opacity,
          lineCap: 'round',
          lineJoin: 'round',
          draggable: true,
        })
        this.drawLayer.add(line)
        this.activeNode = line
        break
      }
      case 'arrow': {
        const arrow = new Konva.Arrow({
          points: [pos.x, pos.y, pos.x, pos.y],
          stroke: color,
          fill: this.style.arrowHeadStyle === 'filled' ? color : 'rgba(0,0,0,0)',
          strokeWidth: this.style.strokeWidth,
          dash: this.style.dashEnabled ? this.style.dash : [],
          pointerLength: this.style.arrowPointerLength,
          pointerWidth: this.style.arrowPointerWidth,
          pointerAtBeginning: this.style.arrowMode === 'start' || this.style.arrowMode === 'both',
          pointerAtEnding: this.style.arrowMode === 'end' || this.style.arrowMode === 'both',
          opacity: this.style.opacity,
          lineCap: 'round',
          lineJoin: 'round',
          draggable: true,
        })
        this.drawLayer.add(arrow)
        this.activeNode = arrow
        break
      }
      case 'pencil':
      case 'marker': {
        const opacity = this.tool === 'marker' ? 0.35 : 1
        const line = new Konva.Line({
          points: [pos.x, pos.y],
          stroke: color,
          strokeWidth: this.style.strokeWidth,
          opacity: opacity * this.style.opacity,
          lineCap: 'round',
          lineJoin: 'round',
          draggable: true,
        })
        this.drawLayer.add(line)
        this.activeNode = line
        break
      }
      case 'mosaic': {
        const rect = new Konva.Rect({
          x: pos.x,
          y: pos.y,
          width: 0,
          height: 0,
          stroke: 'rgba(255,255,255,0.5)',
          strokeWidth: 1,
          dash: [6, 4],
          draggable: false,
          listening: false,
        })
        this.drawLayer.add(rect)
        this.activeNode = rect
        break
      }
    }

    this.drawLayer.batchDraw()
  }

  private onPointerMove(_evt: Konva.KonvaEventObject<MouseEvent | TouchEvent>) {
    if (!this.isDrawing) return
    const pos = this.getPointerPosition()
    if (!pos) return

    this.lastPointer = pos
    if (this.rafPending) return
    this.rafPending = true

    requestAnimationFrame(() => {
      this.rafPending = false
      if (!this.lastPointer || !this.startPoint || !this.activeNode) return

      const p = this.lastPointer
      const s = this.startPoint

      if (this.tool === 'rect' && this.activeNode instanceof Konva.Rect) {
        this.activeNode.setAttrs({
          x: Math.min(s.x, p.x),
          y: Math.min(s.y, p.y),
          width: Math.abs(p.x - s.x),
          height: Math.abs(p.y - s.y),
        })
      } else if (this.tool === 'ellipse' && this.activeNode instanceof Konva.Ellipse) {
        this.activeNode.setAttrs({
          x: (s.x + p.x) / 2,
          y: (s.y + p.y) / 2,
          radiusX: Math.abs(p.x - s.x) / 2,
          radiusY: Math.abs(p.y - s.y) / 2,
        })
      } else if ((this.tool === 'line' || this.tool === 'arrow') && this.activeNode instanceof Konva.Line) {
        this.activeNode.points([s.x, s.y, p.x, p.y])
      } else if ((this.tool === 'pencil' || this.tool === 'marker') && this.activeNode instanceof Konva.Line) {
        this.activePoints.push(p.x, p.y)
        this.activeNode.points(this.activePoints)
      } else if (this.tool === 'mosaic' && this.activeNode instanceof Konva.Rect) {
        this.activeNode.setAttrs({
          x: Math.min(s.x, p.x),
          y: Math.min(s.y, p.y),
          width: Math.abs(p.x - s.x),
          height: Math.abs(p.y - s.y),
        })
      }

      this.drawLayer.batchDraw()
    })
  }

  private onPointerUp(_evt: Konva.KonvaEventObject<MouseEvent | TouchEvent>) {
    if (!this.isDrawing) return
    this.isDrawing = false

    const node = this.activeNode
    this.activeNode = null
    this.activePoints = []
    this.startPoint = null

    if (!node) return

    // Mosaic: replace the preview rect with pixelated image
    if (this.tool === 'mosaic' && node instanceof Konva.Rect) {
      const w = node.width()
      const h = node.height()
      const x = node.x()
      const y = node.y()
      node.destroy()

      if (w >= 4 && h >= 4) {
        const mosaic = createMosaicNode(
          this.backgroundImageEl,
          { x, y, width: w, height: h },
          { pixelSize: this.style.mosaicPixelSize },
        )
        this.drawLayer.add(mosaic)
      }
    }

    this.drawLayer.batchDraw()
    this.commitHistory()
  }

  private onClick(evt: Konva.KonvaEventObject<MouseEvent | TouchEvent>) {
    if (this.tool !== 'select') return

    const target = evt.target
    if (!target || target === this.stage || target === this.backgroundNode) {
      this.clearSelection()
      return
    }

    // Don't select transformer anchors/handles
    const parent = (target as any).getParent?.()
    if (parent && parent.className === 'Transformer') return

    this.setSelection(target)
  }

  private setSelection(node: Konva.Node) {
    this.transformer.nodes([node])
    this.drawLayer.batchDraw()
    this.onSelectionChanged?.(node)
  }

  private clearSelection() {
    this.transformer.nodes([])
    this.drawLayer.batchDraw()
    this.onSelectionChanged?.(null)
  }

  private createTextAt(x: number, y: number): Konva.Group {
    const s = this.style
    const fontStyle = s.textBold && s.textItalic ? 'bold italic' : s.textBold ? 'bold' : s.textItalic ? 'italic' : 'normal'
    const group = new Konva.Group({
      x,
      y,
      draggable: true,
      name: 'textGroup',
      opacity: s.opacity,
    })

    const bg = new Konva.Rect({
      x: 0,
      y: 0,
      width: 10,
      height: 10,
      fill: s.textBgColor,
      opacity: s.textBgOpacity,
      cornerRadius: s.textBgRadius,
      visible: s.textBgEnabled,
      name: 'textBg',
      listening: false,
    })

    const text = new Konva.Text({
      x: 0,
      y: 0,
      text: '',
      fontFamily: s.fontFamily,
      fontStyle,
      fontSize: s.fontSize,
      fill: s.stroke,
      padding: s.textPadding,
      name: 'textNode',
    })

    group.add(bg)
    group.add(text)
    this.syncTextGroupLayout(group)

    this.drawLayer.add(group)
    this.setSelection(group)
    this.drawLayer.batchDraw()

    // Double click to edit
    group.on('dblclick dbltap', () => this.editTextEntity(group))
    text.on('dblclick dbltap', () => this.editTextEntity(group))

    return group
  }

  private syncTextGroupLayout(group: Konva.Group) {
    const text = group.findOne('.textNode') as Konva.Text | null
    const bg = group.findOne('.textBg') as Konva.Rect | null
    if (!text || !bg) return

    // Ensure background wraps text client rect
    const rect = text.getClientRect({ skipTransform: true })
    bg.position({ x: rect.x, y: rect.y })
    bg.size({ width: rect.width, height: rect.height })
    bg.visible(this.style.textBgEnabled)
  }

  private editTextEntity(node: Konva.Node) {
    if (node instanceof Konva.Group && node.name() === 'textGroup') {
      const text = node.findOne('.textNode') as Konva.Text | null
      if (text) {
        this.editTextNode(text, node)
      }
      return
    }
    if (node instanceof Konva.Text) {
      this.editTextNode(node)
    }
  }

  private editTextNode(textNode: Konva.Text, owningGroup?: Konva.Group) {
    const initialText = textNode.text()
    const absPos = textNode.getAbsolutePosition()
    const stageBox = this.container.getBoundingClientRect()
    const stageWidth = this.stage.width()
    const stageHeight = this.stage.height()

    const area = document.createElement('textarea')
    area.value = initialText
    area.placeholder = '文本'
    area.style.position = 'fixed'
    area.style.left = `${stageBox.left + absPos.x}px`
    area.style.top = `${stageBox.top + absPos.y}px`
    // Calculate max width before hitting right edge of stage
    const maxWidth = Math.max(100, stageWidth - absPos.x - 10)
    area.style.minWidth = '60px'
    area.style.maxWidth = `${maxWidth}px`
    area.style.width = 'auto'
    area.style.height = 'auto'
    area.style.minHeight = '28px'
    area.style.fontFamily = `${textNode.fontFamily() || 'Microsoft YaHei UI'}`
    area.style.fontSize = `${textNode.fontSize()}px`
    area.style.lineHeight = '1.3'
    const fs = (textNode.fontStyle?.() as any) || 'normal'
    area.style.fontWeight = String(fs).includes('bold') ? '700' : '400'
    area.style.fontStyle = String(fs).includes('italic') ? 'italic' : 'normal'
    area.style.border = '1px solid rgba(255,255,255,0.35)'
    area.style.padding = '4px 6px'
    area.style.margin = '0'
    area.style.background = 'rgba(0,0,0,0.75)'
    area.style.color = 'white'
    area.style.outline = 'none'
    area.style.resize = 'none'
    area.style.zIndex = '9999'
    area.style.pointerEvents = 'auto'
    area.style.userSelect = 'text'
    area.style.whiteSpace = 'pre-wrap'
    area.style.wordBreak = 'break-word'
    area.style.overflowWrap = 'break-word'
    area.style.overflow = 'hidden'
    ;(area as any).tabIndex = 0

    document.body.appendChild(area)

    // Auto-resize textarea to fit content
    const autoResize = () => {
      // Reset height to auto to measure scrollHeight
      area.style.height = 'auto'
      // Measure content width with a hidden span
      const span = document.createElement('span')
      span.style.visibility = 'hidden'
      span.style.position = 'absolute'
      span.style.whiteSpace = 'pre'
      span.style.font = area.style.font
      span.style.fontSize = area.style.fontSize
      span.style.fontFamily = area.style.fontFamily
      span.style.fontWeight = area.style.fontWeight
      span.style.padding = area.style.padding
      span.textContent = area.value || ' '
      document.body.appendChild(span)
      const contentWidth = Math.min(span.offsetWidth + 20, maxWidth)
      document.body.removeChild(span)

      // Set width based on content, capped at maxWidth
      if (!area.value.includes('\n') && contentWidth < maxWidth) {
        area.style.width = `${Math.max(60, contentWidth)}px`
      } else {
        area.style.width = `${maxWidth}px`
      }
      // Set height based on scrollHeight
      area.style.height = `${Math.max(28, area.scrollHeight)}px`

      // If textarea would go below stage, adjust top position
      const areaRect = area.getBoundingClientRect()
      const stageBottom = stageBox.top + stageHeight
      if (areaRect.bottom > stageBottom) {
        const newTop = Math.max(stageBox.top, stageBottom - areaRect.height - 5)
        area.style.top = `${newTop}px`
      }
    }

    // Prevent Konva/canvas handlers from stealing the click/focus.
    // WebView2 can be picky about focus; retry focus in rAF/timeout.
    const stop = (e: Event) => {
      e.stopPropagation()
    }
    area.addEventListener('pointerdown', stop)
    area.addEventListener('mousedown', stop)
    area.addEventListener('input', autoResize)

    const focusArea = () => {
      try {
        area.focus({ preventScroll: true } as any)
      } catch {
        area.focus()
      }
    }

    focusArea()
    autoResize()
    requestAnimationFrame(() => {
      focusArea()
      autoResize()
    })
    setTimeout(() => {
      focusArea()
      autoResize()
    }, 0)
    try {
      const len = area.value.length
      area.setSelectionRange(len, len)
    } catch {
      // ignore
    }

    let cleaned = false
    const cleanup = () => {
      if (cleaned) return
      cleaned = true
      area.removeEventListener('pointerdown', stop)
      area.removeEventListener('mousedown', stop)
      area.removeEventListener('input', autoResize)
      area.removeEventListener('keydown', onKeyDown)
      area.removeEventListener('blur', onBlur)
      if (area.parentElement) area.parentElement.removeChild(area)
    }

    const finish = () => {
      if (cleaned) return
      const trimmed = area.value.trim()
      cleanup()

      // If user leaves it empty, remove the annotation entirely.
      if (trimmed.length === 0) {
        this.clearSelection()
        if (owningGroup) owningGroup.destroy()
        else textNode.destroy()
        this.drawLayer.batchDraw()
        this.commitHistory()
        return
      }

      textNode.text(area.value)
      if (owningGroup) this.syncTextGroupLayout(owningGroup)
      this.drawLayer.batchDraw()
      this.commitHistory()
    }

    const cancel = () => {
      if (cleaned) return
      cleanup()

      // If this text started empty (newly created) and user cancels, discard it.
      if (initialText.trim().length === 0) {
        this.clearSelection()
        if (owningGroup) owningGroup.destroy()
        else textNode.destroy()
        this.drawLayer.batchDraw()
        this.commitHistory()
        return
      }

      this.drawLayer.batchDraw()
    }

    const onKeyDown = (e: KeyboardEvent) => {
      // Enter confirms, Ctrl+Enter or Shift+Enter for newline
      if (e.key === 'Enter') {
        if (e.ctrlKey || e.shiftKey) {
          // Insert newline at cursor position
          const start = area.selectionStart
          const end = area.selectionEnd
          const value = area.value
          area.value = value.substring(0, start) + '\n' + value.substring(end)
          area.selectionStart = area.selectionEnd = start + 1
          autoResize()
          e.preventDefault()
          return
        }
        e.preventDefault()
        finish()
        return
      }
      if (e.key === 'Escape') {
        e.preventDefault()
        cancel()
      }
    }

    const onBlur = () => finish()

    area.addEventListener('keydown', onKeyDown)
    area.addEventListener('blur', onBlur)
  }

  private commitHistory(isInit = false) {
    // snapshot draw layer only (keep background separate)
    const snapshot = this.drawLayer.toJSON()

    // Avoid duplicate snapshots
    if (!isInit && this.undoStack.length > 0 && this.undoStack[this.undoStack.length - 1] === snapshot) return

    this.undoStack.push(snapshot)
    this.redoStack = []
  }

  private restoreFromSnapshot(snapshot: string) {
    // Preserve transformer config but re-create layer content
    const oldTransformer = this.transformer

    // Create a new layer from JSON
    const recreated = Konva.Node.create(snapshot) as Konva.Layer

    // Remove existing nodes (except transformer)
    this.drawLayer.destroyChildren()

    // Move children from recreated into drawLayer
    ;(recreated.getChildren() as unknown as any[]).forEach((n) => {
      // Skip transformer nodes if any
      if (n.className === 'Transformer') return
      n.remove()
      this.drawLayer.add(n as any)

      // Rebind text edit
      if (n instanceof Konva.Text) {
        n.off('dblclick dbltap')
        n.on('dblclick dbltap', () => this.editTextNode(n))
      }

      if (n instanceof Konva.Group && n.name() === 'textGroup') {
        n.off('dblclick dbltap')
        n.on('dblclick dbltap', () => this.editTextEntity(n))
        const t = n.findOne('.textNode') as Konva.Text | null
        if (t) {
          t.off('dblclick dbltap')
          t.on('dblclick dbltap', () => this.editTextEntity(n))
        }
        this.syncTextGroupLayout(n)
      }
    })

    // Ensure transformer stays on top
    this.transformer = oldTransformer
    this.drawLayer.add(this.transformer)
    this.clearSelection()

    this.drawLayer.batchDraw()
  }
}
