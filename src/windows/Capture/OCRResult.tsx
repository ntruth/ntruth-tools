import { Component, Show } from 'solid-js'

export interface OCRResultProps {
  open: boolean
  loading: boolean
  previewSrc: string
  text: string
  onTextChange: (next: string) => void
  onCopy: () => void
  onTranslate: () => void
  onClose: () => void
}

const OCRResult: Component<OCRResultProps> = (props) => {
  return (
    <Show when={props.open}>
      <div
        class="fixed inset-0 z-[60] flex items-center justify-center"
        onMouseDown={(e) => e.stopPropagation()}
      >
        {/* Backdrop */}
        <div
          class="absolute inset-0 bg-black/60"
          onMouseDown={(e) => {
            e.stopPropagation()
            props.onClose()
          }}
        />

        {/* Modal */}
        <div
          class="relative mx-4 w-[min(980px,100%)] rounded-lg border border-white/10 bg-gray-900/95 shadow-2xl backdrop-blur-sm"
          onMouseDown={(e) => e.stopPropagation()}
        >
          <div class="flex items-center justify-between border-b border-white/10 px-4 py-3">
            <div class="text-sm font-medium text-white/90">OCR</div>
            <button
              class="rounded px-2 py-1 text-xs text-white/80 hover:bg-white/10 hover:text-white"
              onMouseDown={(e) => {
                e.stopPropagation()
                props.onClose()
              }}
            >
              关闭
            </button>
          </div>

          <div class="flex flex-col gap-3 p-4 md:flex-row">
            {/* Preview */}
            <div class="w-full md:w-[44%]">
              <div class="mb-2 text-xs text-white/60">预览</div>
              <div class="overflow-hidden rounded border border-white/10 bg-black/20">
                <img src={props.previewSrc} alt="ocr preview" class="block h-auto w-full" />
              </div>
            </div>

            {/* Text */}
            <div class="w-full md:w-[56%]">
              <div class="mb-2 flex items-center justify-between">
                <div class="text-xs text-white/60">文本</div>
                <Show when={props.loading}>
                  <div class="text-xs text-white/60">识别中...</div>
                </Show>
              </div>

              <textarea
                class="h-64 w-full resize-none rounded border border-white/10 bg-white/5 p-3 text-sm text-white/90 outline-none focus:border-white/20"
                value={props.text}
                placeholder={props.loading ? '识别中...' : '未识别到文字'}
                disabled={props.loading}
                onInput={(e) => props.onTextChange(e.currentTarget.value)}
              />

              <div class="mt-3 flex items-center justify-end gap-2">
                <button
                  class="rounded bg-white/10 px-3 py-1.5 text-xs text-white/85 hover:bg-white/15 disabled:opacity-40"
                  disabled={props.loading}
                  onMouseDown={(e) => {
                    e.stopPropagation()
                    props.onCopy()
                  }}
                >
                  复制文本
                </button>
                <button
                  class="rounded bg-white/10 px-3 py-1.5 text-xs text-white/85 hover:bg-white/15 disabled:opacity-40"
                  disabled
                  title="预留接口"
                  onMouseDown={(e) => {
                    e.stopPropagation()
                    props.onTranslate()
                  }}
                >
                  翻译(预留)
                </button>
                <button
                  class="rounded bg-blue-500 px-3 py-1.5 text-xs text-white hover:bg-blue-600"
                  onMouseDown={(e) => {
                    e.stopPropagation()
                    props.onClose()
                  }}
                >
                  关闭
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Show>
  )
}

export default OCRResult
