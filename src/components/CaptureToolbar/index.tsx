import { Component, Show, createMemo } from 'solid-js'

export interface Selection {
  x: number
  y: number
  w: number
  h: number
}

export interface CaptureToolbarProps {
  selection: Selection
  onCopy: () => void
  onSave: () => void
  onPin: () => void
  onCancel: () => void
  onDraw?: () => void
  onRect?: () => void
  onMosaic?: () => void
  onText?: () => void
  onOcr?: () => void
}

/**
 * Smart positioning algorithm for toolbar
 * - Default: below selection, aligned to right edge
 * - If too close to bottom: move above selection or inside
 * - If too close to right: align to left edge
 */
const useToolbarPosition = (selection: () => Selection) => {
  return createMemo(() => {
    const sel = selection()
    const toolbarHeight = 44
    const toolbarWidth = 320
    const margin = 10
    const windowWidth = window.innerWidth
    const windowHeight = window.innerHeight

    let top = sel.y + sel.h + margin
    let left = sel.x + sel.w - toolbarWidth

    // Bottom boundary check
    if (top + toolbarHeight > windowHeight - margin) {
      // Try above selection
      if (sel.y - toolbarHeight - margin > 0) {
        top = sel.y - toolbarHeight - margin
      } else {
        // Inside selection at bottom
        top = sel.y + sel.h - toolbarHeight - margin
      }
    }

    // Right boundary check
    if (left < margin) {
      left = sel.x
    }

    // Left boundary check
    if (left + toolbarWidth > windowWidth - margin) {
      left = windowWidth - toolbarWidth - margin
    }

    return { top, left }
  })
}

const ToolButton: Component<{
  icon: string
  label: string
  onClick: () => void
  primary?: boolean
}> = (props) => (
  <button
    class={`flex items-center gap-1 rounded px-2 py-1.5 text-xs transition ${
      props.primary 
        ? 'bg-blue-500 hover:bg-blue-600 text-white' 
        : 'hover:bg-white/20 text-white'
    }`}
    onClick={(e) => {
      e.stopPropagation()
      props.onClick()
    }}
    title={props.label}
  >
    <span>{props.icon}</span>
    <span class="hidden sm:inline">{props.label}</span>
  </button>
)

const Divider: Component = () => (
  <div class="h-5 w-px bg-white/20" />
)

export const CaptureToolbar: Component<CaptureToolbarProps> = (props) => {
  const position = useToolbarPosition(() => props.selection)

  return (
    <div
      data-capture-toolbar="true"
      class="pointer-events-auto absolute z-50 flex items-center gap-1 rounded-lg bg-gray-900/95 px-2 py-1.5 shadow-2xl backdrop-blur-sm border border-white/10"
      style={{
        top: `${position().top}px`,
        left: `${position().left}px`,
      }}
      onMouseDown={(e) => e.stopPropagation()}
    >
      {/* Drawing tools */}
      <Show when={props.onRect}>
        <ToolButton icon="▢" label="矩形" onClick={props.onRect!} />
      </Show>
      <Show when={props.onDraw}>
        <ToolButton icon="✎" label="画笔" onClick={props.onDraw!} />
      </Show>
      <Show when={props.onMosaic}>
        <ToolButton icon="▦" label="马赛克" onClick={props.onMosaic!} />
      </Show>
      <Show when={props.onText}>
        <ToolButton icon="T" label="文字" onClick={props.onText!} />
      </Show>
      
      <Show when={props.onRect || props.onDraw || props.onMosaic || props.onText}>
        <Divider />
      </Show>

      {/* Actions */}
      <ToolButton icon="📋" label="复制" onClick={props.onCopy} />
      <ToolButton icon="📌" label="贴图" onClick={props.onPin} />
      <Show when={props.onOcr}>
        <ToolButton icon="🔍" label="OCR" onClick={props.onOcr!} />
      </Show>
      
      <Divider />

      {/* Primary actions */}
      <ToolButton icon="✓" label="保存" onClick={props.onSave} primary />
      <ToolButton icon="✕" label="取消" onClick={props.onCancel} />
    </div>
  )
}

export default CaptureToolbar
