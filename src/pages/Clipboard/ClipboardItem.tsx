import { Component } from 'solid-js'
import { Star, Copy, Trash2 } from 'lucide-solid'
import type { ClipboardItemData } from './index'

interface ClipboardItemProps {
  item: ClipboardItemData
  selected: boolean
  onClick: () => void
  onPaste: () => void
  onToggleFavorite: () => void
  onDelete: () => void
}

const ClipboardItem: Component<ClipboardItemProps> = (props) => {
  const formatTime = (timestamp: number) => {
    const date = new Date(timestamp * 1000)
    const now = new Date()
    const diff = Math.floor((now.getTime() - date.getTime()) / 1000)

    if (diff < 60) return 'Just now'
    if (diff < 3600) return `${Math.floor(diff / 60)}m ago`
    if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`
    if (diff < 604800) return `${Math.floor(diff / 86400)}d ago`
    
    return date.toLocaleDateString()
  }

  const getTypeIcon = (type: string) => {
    switch (type) {
      case 'text':
        return '📄'
      case 'image':
        return '🖼️'
      case 'files':
        return '📁'
      case 'html':
        return '🌐'
      default:
        return '📋'
    }
  }

  const truncateContent = (content: string, maxLength: number = 100) => {
    if (content.length <= maxLength) return content
    return content.substring(0, maxLength) + '...'
  }

  return (
    <div
      class={`cursor-pointer border-b border-gray-200 p-3 transition-colors hover:bg-gray-50 dark:border-gray-700 dark:hover:bg-gray-800 ${
        props.selected ? 'bg-blue-50 dark:bg-blue-900/20' : ''
      }`}
      onClick={props.onClick}
      onDblClick={props.onPaste}
    >
      <div class="flex items-start justify-between">
        <div class="flex-1">
          <div class="flex items-center gap-2">
            <span class="text-lg">{getTypeIcon(props.item.type)}</span>
            <div class="flex-1">
              <div class="text-sm text-gray-900 dark:text-gray-100">
                {truncateContent(props.item.content)}
              </div>
              <div class="mt-1 text-xs text-gray-500 dark:text-gray-400">
                {formatTime(props.item.timestamp)}
              </div>
            </div>
          </div>
        </div>

        <div class="flex items-center gap-2">
          <button
            onClick={(e) => {
              e.stopPropagation()
              props.onToggleFavorite()
            }}
            class="rounded p-1 hover:bg-gray-200 dark:hover:bg-gray-700"
            title="Toggle favorite"
          >
            <Star
              size={16}
              class={props.item.favorite ? 'fill-yellow-500 text-yellow-500' : 'text-gray-400'}
            />
          </button>
          <button
            onClick={(e) => {
              e.stopPropagation()
              props.onPaste()
            }}
            class="rounded p-1 hover:bg-gray-200 dark:hover:bg-gray-700"
            title="Paste"
          >
            <Copy size={16} class="text-gray-600 dark:text-gray-400" />
          </button>
          <button
            onClick={(e) => {
              e.stopPropagation()
              props.onDelete()
            }}
            class="rounded p-1 hover:bg-gray-200 dark:hover:bg-gray-700"
            title="Delete"
          >
            <Trash2 size={16} class="text-red-500" />
          </button>
        </div>
      </div>
    </div>
  )
}

export default ClipboardItem
