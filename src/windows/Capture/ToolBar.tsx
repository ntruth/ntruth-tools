import { Component, createMemo } from 'solid-js'
import type { ToolType } from './DrawManager'

export interface ToolBarProps {
  tool: ToolType
  onToolChange: (tool: ToolType) => void

  onUndo: () => void
  onRedo: () => void
  canUndo: boolean
  canRedo: boolean

  onBrushThin: () => void
  onBrushThick: () => void
}

const ToolButton: Component<{
  label: string
  active?: boolean
  onClick: () => void
}> = (props) => (
  <button
    class={
      props.active
        ? 'rounded bg-white/15 px-2 py-1 text-xs text-white'
        : 'rounded px-2 py-1 text-xs text-white/80 hover:bg-white/10 hover:text-white'
    }
    onClick={(e) => {
      e.stopPropagation()
      props.onClick()
    }}
    title={props.label}
  >
    {props.label}
  </button>
)

/**
 * Floating toolbar for annotation tools.
 *
 * Keyboard shortcuts (handled in DrawManager):
 * - Tab: line <-> arrow
 * - 1/2: brush width
 * - Ctrl+Z / Ctrl+Y: undo/redo
 * - Space: pan stage
 */
const ToolBar: Component<ToolBarProps> = (props) => {
  const tools = createMemo(() => [
    { t: 'select' as const, label: '选择' },
    { t: 'rect' as const, label: '矩形' },
    { t: 'ellipse' as const, label: '椭圆' },
    { t: 'line' as const, label: '直线' },
    { t: 'arrow' as const, label: '箭头' },
    { t: 'pencil' as const, label: '铅笔' },
    { t: 'marker' as const, label: '马克笔' },
    { t: 'text' as const, label: '文字' },
    { t: 'mosaic' as const, label: '马赛克' },
  ])

  return (
    <div
      class="pointer-events-auto absolute left-3 top-3 z-50 flex flex-wrap items-center gap-1 rounded-lg bg-gray-900/95 px-2 py-1.5 backdrop-blur-sm border border-white/10"
      onMouseDown={(e) => e.stopPropagation()}
    >
      {tools().map((x) => (
        <ToolButton
          label={x.label}
          active={props.tool === x.t}
          onClick={() => props.onToolChange(x.t)}
        />
      ))}

      <div class="mx-1 h-5 w-px bg-white/10" />

      <ToolButton label="撤销" onClick={props.onUndo} active={false} />
      <ToolButton label="重做" onClick={props.onRedo} active={false} />

      <div class="mx-1 h-5 w-px bg-white/10" />

      <ToolButton label="细(1)" onClick={props.onBrushThin} />
      <ToolButton label="粗(2)" onClick={props.onBrushThick} />

      <div class="text-[11px] text-white/50 ml-1">
        Tab切换直线/箭头 · Space拖拽 · Shift旋转吸附
      </div>
    </div>
  )
}

export default ToolBar
