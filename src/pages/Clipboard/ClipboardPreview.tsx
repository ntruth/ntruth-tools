import { Component } from 'solid-js'
import type { ClipboardItemData } from './index'

interface ClipboardPreviewProps {
  item: ClipboardItemData
}

const ClipboardPreview: Component<ClipboardPreviewProps> = (props) => {
  return (
    <div class="border-t border-gray-200 bg-gray-50 p-4 dark:border-gray-700 dark:bg-gray-800">
      <div class="mb-2 text-xs font-semibold uppercase text-gray-600 dark:text-gray-400">
        Preview
      </div>
      <div class="max-h-32 overflow-y-auto rounded-lg bg-white p-3 text-sm text-gray-900 dark:bg-gray-900 dark:text-gray-100">
        <pre class="whitespace-pre-wrap font-mono text-xs">{props.item.content}</pre>
      </div>
    </div>
  )
}

export default ClipboardPreview
