import { Component, createMemo, createSignal, Show, For } from 'solid-js'
import type { ToolType, AnnotationStyle } from '../../windows/Capture/DrawManager'

export interface Selection {
  x: number
  y: number
  w: number
  h: number
}

export interface CaptureToolbarProps {
  selection: Selection

  // Annotation tools
  tool: ToolType
  onToolChange: (tool: ToolType) => void

  style: AnnotationStyle
  onStyleChange: (patch: Partial<AnnotationStyle>) => void
  onApplyStyleToSelection: () => void

  onUndo: () => void
  onRedo: () => void
  canUndo: boolean
  canRedo: boolean
  onBrushThin: () => void
  onBrushThick: () => void

  // Actions
  onCopy: () => void
  onOcr: () => void
  ocrLoading?: boolean
  onSave: () => void
  onPin: () => void
  onCancel: () => void
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
    const toolbarHeight = 84
    const toolbarWidth = 380
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

const IconButton: Component<{
  label: string
  onClick: () => void
  active?: boolean
  disabled?: boolean
  primary?: boolean
  danger?: boolean
  children: any
}> = (props) => (
  <button
    class={
      `group relative flex h-7 w-7 items-center justify-center rounded transition-all duration-150 ` +
      (props.primary
        ? 'bg-blue-500 text-white hover:bg-blue-600 shadow-sm'
        : props.danger
          ? 'bg-red-500/80 text-white hover:bg-red-600'
          : props.active
            ? 'bg-white/20 text-white shadow-inner'
            : 'text-white/80 hover:bg-white/10 hover:text-white') +
      (props.disabled ? ' opacity-40 pointer-events-none' : '')
    }
    onClick={(e) => {
      e.stopPropagation()
      props.onClick()
    }}
    aria-label={props.label}
  >
    <span class="h-[14px] w-[14px]">{props.children}</span>
    <span class="pointer-events-none absolute -top-7 left-1/2 -translate-x-1/2 whitespace-nowrap rounded bg-black/90 px-1.5 py-0.5 text-[10px] text-white/95 opacity-0 shadow-lg transition-opacity group-hover:opacity-100">
      {props.label}
    </span>
  </button>
)

const Icon: Component<{ path: string }> = (props) => (
  <svg viewBox="0 0 24 24" class="h-[14px] w-[14px]" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d={props.path} />
  </svg>
)

const FilledIcon: Component<{ path: string }> = (props) => (
  <svg viewBox="0 0 24 24" class="h-[14px] w-[14px]" fill="currentColor">
    <path d={props.path} />
  </svg>
)

const Divider: Component = () => (
  <div class="mx-0.5 h-5 w-px bg-white/15" />
)

const SquareSwatch: Component<{
  color: string
  active?: boolean
  onClick: () => void
  size?: 'sm' | 'md'
}> = (props) => (
  <button
    class={
      `border transition-all ` +
      (props.size === 'sm' ? 'h-3 w-3 ' : 'h-4 w-4 ') +
      (props.active ? 'border-white scale-110 shadow-sm' : 'border-white/25 hover:border-white/60 hover:scale-105')
    }
    style={{ background: props.color }}
    onMouseDown={(e) => {
      e.stopPropagation()
      props.onClick()
    }}
    aria-label="color"
  />
)

const MiniPopover: Component<{
  children: any
}> = (props) => (
  <div
    class="absolute bottom-8 left-0 z-50 rounded-md bg-gray-900/98 p-2 shadow-2xl backdrop-blur-md border border-white/15"
    onMouseDown={(e) => e.stopPropagation()}
  >
    {props.children}
  </div>
)

export const CaptureToolbar: Component<CaptureToolbarProps> = (props) => {
  const position = useToolbarPosition(() => props.selection)
  const [openMenu, setOpenMenu] = createSignal<'more' | 'arrow' | null>(null)

  const palette = createMemo(() => {
    // Snipaste-like: provide a richer palette + allow custom.
    return [
      '#ff3b30', // red
      '#ff9500', // orange
      '#ffcc00', // yellow
      '#34c759', // green
      '#007aff', // blue
      '#af52de', // purple
      '#ff2d55', // pink
      '#00c7be', // teal
      '#5856d6', // indigo
      '#8e8e93', // gray
      '#c7c7cc', // light gray
      '#5ac8fa', // light blue
      '#30d158', // vivid green
      '#ffd60a', // vivid yellow
      '#ffffff', // white
      '#000000', // black
    ]
  })

  const arrowStyleOptions = createMemo(() => {
    return [
      { key: 'end_filled', label: '→ 实心', patch: { arrowMode: 'end' as const, arrowHeadStyle: 'filled' as const } },
      { key: 'end_outline', label: '→ 空心', patch: { arrowMode: 'end' as const, arrowHeadStyle: 'outline' as const } },
      { key: 'both_filled', label: '↔ 实心', patch: { arrowMode: 'both' as const, arrowHeadStyle: 'filled' as const } },
      { key: 'both_outline', label: '↔ 空心', patch: { arrowMode: 'both' as const, arrowHeadStyle: 'outline' as const } },
      { key: 'start_filled', label: '← 实心', patch: { arrowMode: 'start' as const, arrowHeadStyle: 'filled' as const } },
      { key: 'start_outline', label: '← 空心', patch: { arrowMode: 'start' as const, arrowHeadStyle: 'outline' as const } },
    ]
  })

  const fontFamilies = createMemo(() => [
    'Microsoft YaHei UI',
    'Microsoft YaHei',
    'Segoe UI',
    'Arial',
    'SimSun',
  ])

  const apply = (patch: Partial<AnnotationStyle>) => {
    props.onStyleChange(patch)
    props.onApplyStyleToSelection()
  }

  const toggleMore = () => setOpenMenu((cur) => (cur === 'more' ? null : 'more'))
  const toggleArrow = () => setOpenMenu((cur) => (cur === 'arrow' ? null : 'arrow'))

  const onPickStroke = (c: string) => apply({ stroke: c })

  const showFillRow = createMemo(() => props.tool === 'rect' || props.tool === 'ellipse')
  const showArrowRow = createMemo(() => props.tool === 'arrow')
  const showTextRow = createMemo(() => props.tool === 'text')

  return (
    <div
      data-capture-toolbar="true"
      class="pointer-events-auto absolute z-50 flex flex-col rounded-lg bg-gray-900/98 px-1.5 py-1 shadow-2xl backdrop-blur-md border border-white/15"
      style={{
        top: `${position().top}px`,
        left: `${position().left}px`,
      }}
      onMouseDown={(e) => e.stopPropagation()}
    >
      {/* Top row: tools + actions */}
      <div class="flex flex-wrap items-center gap-0.5">
        <IconButton label="选择" active={props.tool === 'select'} onClick={() => props.onToolChange('select')} disabled={props.ocrLoading}>
          <Icon path="M3 3l7 18 2-7 7-2L3 3z" />
        </IconButton>
        <IconButton label="矩形" active={props.tool === 'rect'} onClick={() => props.onToolChange('rect')} disabled={props.ocrLoading}>
          <Icon path="M4 6h16v12H4z" />
        </IconButton>
        <IconButton label="椭圆" active={props.tool === 'ellipse'} onClick={() => props.onToolChange('ellipse')} disabled={props.ocrLoading}>
          <Icon path="M12 5c5 0 8 3 8 7s-3 7-8 7-8-3-8-7 3-7 8-7z" />
        </IconButton>
        <IconButton label="箭头" active={props.tool === 'arrow'} onClick={() => props.onToolChange('arrow')} disabled={props.ocrLoading}>
          <Icon path="M5 12h12m-4-4 4 4-4 4" />
        </IconButton>
        <IconButton label="铅笔" active={props.tool === 'pencil'} onClick={() => props.onToolChange('pencil')} disabled={props.ocrLoading}>
          <Icon path="M12 20h9M16.5 3.5a2.1 2.1 0 0 1 3 3L7 19l-4 1 1-4 12.5-12.5z" />
        </IconButton>
        <IconButton label="文字" active={props.tool === 'text'} onClick={() => props.onToolChange('text')} disabled={props.ocrLoading}>
          <Icon path="M4 6h16M10 6v14M14 6v14" />
        </IconButton>
        <IconButton label="马赛克" active={props.tool === 'mosaic'} onClick={() => props.onToolChange('mosaic')} disabled={props.ocrLoading}>
          <Icon path="M4 4h7v7H4zM13 4h7v7h-7zM4 13h7v7H4zM13 13h7v7h-7z" />
        </IconButton>

        <Divider />

        <IconButton label="撤销" onClick={props.onUndo} disabled={!props.canUndo || props.ocrLoading}>
          <Icon path="M9 14l-4-4 4-4M5 10h9a5 5 0 0 1 0 10h-2" />
        </IconButton>
        <IconButton label="重做" onClick={props.onRedo} disabled={!props.canRedo || props.ocrLoading}>
          <Icon path="M15 14l4-4-4-4M19 10H10a5 5 0 0 0 0 10h2" />
        </IconButton>

        <Divider />

        <IconButton label="细(1)" onClick={props.onBrushThin} disabled={props.ocrLoading}>
          <FilledIcon path="M4 12h16v2H4z" />
        </IconButton>
        <IconButton label="粗(2)" onClick={props.onBrushThick} disabled={props.ocrLoading}>
          <FilledIcon path="M4 11h16v4H4z" />
        </IconButton>

        {/* Actions */}
        <IconButton label="复制" onClick={props.onCopy} disabled={props.ocrLoading}>
          <Icon path="M8 8h10v12H8zM6 16H4V4h12v2" />
        </IconButton>
        <IconButton label={props.ocrLoading ? '识别中...' : 'OCR'} onClick={props.onOcr} disabled={props.ocrLoading}>
          <Icon path="M4 6h16M4 12h10M4 18h16" />
        </IconButton>
        <IconButton label="贴图" onClick={props.onPin} disabled={props.ocrLoading}>
          <Icon path="M9 3l6 6m-6 0V3m6 6-3 3v8l-2-2-2 2v-8l-3-3" />
        </IconButton>
        <IconButton label="保存" onClick={props.onSave} primary disabled={props.ocrLoading}>
          <Icon path="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2zM7 21v-8h10v8" />
        </IconButton>
        <IconButton label="取消" onClick={props.onCancel} danger>
          <Icon path="M18 6L6 18M6 6l12 12" />
        </IconButton>
      </div>

      {/* Bottom row: Snipaste-style inline settings */}
      <div class="relative mt-1 flex w-full items-center gap-1.5 border-t border-white/10 pt-1">
        <div class="relative">
          <button
            class="flex h-7 items-center gap-1 rounded px-1.5 text-white/85 hover:bg-white/10"
            onMouseDown={(e) => {
              e.stopPropagation()
              toggleMore()
            }}
            aria-label="more"
          >
            <span class="h-2 w-2 rounded-full" style={{ background: props.style.stroke, opacity: props.style.opacity ?? 1 }} />
            <span class="text-xs">…</span>
          </button>

          <Show when={openMenu() === 'more'}>
            <MiniPopover>
              <div class="flex items-center gap-3">
                <div class="w-12 text-xs text-white/70">透明度</div>
                <input
                  class="w-40"
                  type="range"
                  min="5"
                  max="100"
                  value={Math.round((props.style.opacity ?? 1) * 100)}
                  onInput={(e) => {
                    const raw = Number((e.currentTarget as HTMLInputElement).value)
                    apply({ opacity: raw / 100 })
                  }}
                />
                <div class="w-10 text-right text-xs text-white/80">{Math.round((props.style.opacity ?? 1) * 100)}%</div>
              </div>
            </MiniPopover>
          </Show>
        </div>

        <Show when={showFillRow()}>
          <button
            class={
              `flex h-7 w-7 items-center justify-center rounded ` +
              (props.style.fillEnabled ? 'bg-white/15 text-white' : 'text-white/80 hover:bg-white/10')
            }
            onMouseDown={(e) => {
              e.stopPropagation()
              apply({ fillEnabled: !props.style.fillEnabled })
            }}
            aria-label="fill"
          >
            <span
              class="h-4 w-4 border border-white/30"
              style={{ background: props.style.fillEnabled ? props.style.fill : 'transparent' }}
            />
          </button>

          <button
            class={
              `flex h-7 w-7 items-center justify-center rounded ` +
              (props.tool === 'rect' ? 'bg-white/15 text-white' : 'text-white/80 hover:bg-white/10')
            }
            onMouseDown={(e) => {
              e.stopPropagation()
              props.onToolChange('rect')
            }}
            aria-label="rect"
          >
            <Icon path="M6 7h12v10H6z" />
          </button>

          <button
            class={
              `flex h-7 w-7 items-center justify-center rounded ` +
              (props.tool === 'ellipse' ? 'bg-white/15 text-white' : 'text-white/80 hover:bg-white/10')
            }
            onMouseDown={(e) => {
              e.stopPropagation()
              props.onToolChange('ellipse')
            }}
            aria-label="ellipse"
          >
            <Icon path="M12 6c4 0 7 2.5 7 6s-3 6-7 6-7-2.5-7-6 3-6 7-6z" />
          </button>

          <div class="h-5 w-px bg-white/10" />
        </Show>

        <Show when={showArrowRow()}>
          <div class="relative">
            <button
              class="flex h-7 items-center gap-1 rounded px-2 text-xs text-white/85 hover:bg-white/10"
              onMouseDown={(e) => {
                e.stopPropagation()
                toggleArrow()
              }}
              aria-label="arrow style"
            >
              <span class="text-white/90">
                {props.style.arrowMode === 'both' ? '↔' : props.style.arrowMode === 'start' ? '←' : '→'}
                {props.style.arrowHeadStyle === 'outline' ? '空' : '实'}
              </span>
              <svg viewBox="0 0 24 24" class="h-3 w-3" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M6 9l6 6 6-6" />
              </svg>
            </button>

            <Show when={openMenu() === 'arrow'}>
              <MiniPopover>
                <div class="flex flex-col gap-1">
                  <For each={arrowStyleOptions()}>
                    {(opt) => (
                      <button
                        class={
                          `rounded px-2 py-1 text-left text-xs ` +
                          (props.style.arrowMode === opt.patch.arrowMode && props.style.arrowHeadStyle === opt.patch.arrowHeadStyle
                            ? 'bg-white/15 text-white'
                            : 'text-white/80 hover:bg-white/10')
                        }
                        onMouseDown={(e) => {
                          e.stopPropagation()
                          apply(opt.patch as any)
                          setOpenMenu(null)
                        }}
                      >
                        {opt.label}
                      </button>
                    )}
                  </For>
                </div>
              </MiniPopover>
            </Show>
          </div>

          <div class="h-5 w-px bg-white/10" />
        </Show>

        <Show when={showTextRow()}>
          <button
            class={
              `flex h-7 w-7 items-center justify-center rounded text-sm ` +
              (props.style.textBold ? 'bg-white/15 text-white' : 'text-white/80 hover:bg-white/10')
            }
            onMouseDown={(e) => {
              e.stopPropagation()
              apply({ textBold: !props.style.textBold })
            }}
            aria-label="bold"
          >
            <span class="font-bold">B</span>
          </button>
          <button
            class={
              `flex h-7 w-7 items-center justify-center rounded text-sm ` +
              (props.style.textItalic ? 'bg-white/15 text-white' : 'text-white/80 hover:bg-white/10')
            }
            onMouseDown={(e) => {
              e.stopPropagation()
              apply({ textItalic: !props.style.textItalic })
            }}
            aria-label="italic"
          >
            <span class="italic">I</span>
          </button>
          <button
            class={
              `flex h-7 w-7 items-center justify-center rounded text-sm ` +
              (props.style.textBgEnabled ? 'bg-white/15 text-white' : 'text-white/80 hover:bg-white/10')
            }
            onMouseDown={(e) => {
              e.stopPropagation()
              apply({ textBgEnabled: !props.style.textBgEnabled })
            }}
            aria-label="text background"
          >
            <span class="font-semibold">A</span>
          </button>

          <div class="h-5 w-px bg-white/10" />

          <select
            class="h-7 rounded border border-white/15 bg-white/5 px-2 text-xs text-white/90 outline-none focus:border-white/30"
            value={props.style.fontFamily}
            onChange={(e) => apply({ fontFamily: (e.currentTarget as HTMLSelectElement).value })}
          >
            <For each={fontFamilies()}>
              {(f) => <option value={f}>{f}</option>}
            </For>
          </select>
          <input
            class="h-7 w-16 rounded border border-white/15 bg-white/5 px-2 text-xs text-white/90 outline-none focus:border-white/30"
            type="number"
            step="0.1"
            min="8"
            max="200"
            value={props.style.fontSize}
            onInput={(e) => {
              const n = Number((e.currentTarget as HTMLInputElement).value)
              if (!Number.isFinite(n)) return
              apply({ fontSize: n })
            }}
            aria-label="font size"
          />

          <div class="h-5 w-px bg-white/10" />
        </Show>

        {/* Color: current + compact palette (Snipaste style) */}
        <div class="flex items-center gap-1">
          <label class="relative h-5 w-5 cursor-pointer">
            <input
              class="absolute inset-0 h-full w-full opacity-0"
              type="color"
              value={props.style.stroke}
              onInput={(e) => {
                const v = (e.currentTarget as HTMLInputElement).value
                onPickStroke(v)
              }}
              aria-label="pick color"
            />
            <span class="absolute inset-0 border border-white/40 rounded-sm" style={{ background: props.style.stroke }} />
          </label>

          <div class="mx-0.5 h-4 w-px bg-white/10" />

          <div class="flex items-center gap-0.5 flex-wrap max-w-[200px]">
            <For each={palette()}>
              {(c) => (
                <SquareSwatch
                  color={c}
                  active={props.style.stroke === c}
                  onClick={() => onPickStroke(c)}
                  size="sm"
                />
              )}
            </For>
          </div>
        </div>
      </div>
    </div>
  )
}

export default CaptureToolbar
