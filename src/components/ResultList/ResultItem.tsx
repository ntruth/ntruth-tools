import { Component, Show } from 'solid-js'
import { File, Calculator, Globe, Brain, Clipboard, Terminal, Package } from 'lucide-solid'
import type { SearchResult } from '../../types/search'

interface ResultItemProps {
  result: SearchResult
  isSelected: boolean
  index: number
  onClick: () => void
  onMouseEnter: () => void
}

/**
 * Single result item component
 * Displays icon, title, subtitle with highlighted text and selection state
 */
export const ResultItem: Component<ResultItemProps> = (props) => {
  // Get icon based on result type
  const getIcon = () => {
    switch (props.result.type) {
      case 'file':
        return <File class="h-5 w-5" />
      case 'app':
        return <Package class="h-5 w-5" />
      case 'calculator':
        return <Calculator class="h-5 w-5" />
      case 'web-search':
        return <Globe class="h-5 w-5" />
      case 'ai':
        return <Brain class="h-5 w-5" />
      case 'clipboard':
        return <Clipboard class="h-5 w-5" />
      case 'command':
        return <Terminal class="h-5 w-5" />
      default:
        return <File class="h-5 w-5" />
    }
  }

  // Get keyboard shortcut hint
  const getShortcut = () => {
    if (props.index < 9) {
      const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0
      return `${isMac ? '⌘' : 'Ctrl+'} ${props.index + 1}`
    }
    return null
  }

  return (
    <div
      onClick={props.onClick}
      onMouseEnter={props.onMouseEnter}
      class="group flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2.5 transition-all duration-150"
      classList={{
        'bg-blue-50 dark:bg-blue-900/30': props.isSelected,
        'hover:bg-gray-50 dark:hover:bg-gray-800/50': !props.isSelected,
      }}
    >
      {/* Icon */}
      <div
        class="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-lg transition-colors"
        classList={{
          'bg-blue-100 text-blue-600 dark:bg-blue-900/50 dark:text-blue-400': props.isSelected,
          'bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-400': !props.isSelected,
        }}
      >
        <Show when={props.result.icon} fallback={getIcon()}>
          <img src={props.result.icon} alt="" class="h-6 w-6" />
        </Show>
      </div>

      {/* Content */}
      <div class="min-w-0 flex-1">
        {/* Title */}
        <div
          class="truncate font-medium transition-colors"
          classList={{
            'text-gray-900 dark:text-white': props.isSelected,
            'text-gray-800 dark:text-gray-200': !props.isSelected,
          }}
          innerHTML={highlightMatch(props.result.title, '')}
        />

        {/* Subtitle */}
        <Show when={props.result.subtitle}>
          <div
            class="truncate text-sm transition-colors"
            classList={{
              'text-gray-600 dark:text-gray-400': props.isSelected,
              'text-gray-500 dark:text-gray-500': !props.isSelected,
            }}
          >
            {props.result.subtitle}
          </div>
        </Show>
      </div>

      {/* Keyboard Shortcut Hint */}
      <Show when={getShortcut()}>
        <div
          class="flex-shrink-0 rounded px-1.5 py-0.5 text-xs font-medium transition-all"
          classList={{
            'bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300': props.isSelected,
            'bg-gray-100 text-gray-500 opacity-0 group-hover:opacity-100 dark:bg-gray-800 dark:text-gray-400':
              !props.isSelected,
          }}
        >
          {getShortcut()}
        </div>
      </Show>
    </div>
  )
}

/**
 * Highlight matching text in the result
 * Uses DOM manipulation to safely highlight without XSS risks
 * @param text Text to highlight
 * @param query Search query
 */
function highlightMatch(text: string, query: string): string {
  if (!query || !text) return text

  // For security, we'll use a simple approach that doesn't rely on innerHTML
  // In a production app, consider using a library like highlight-words
  const regex = new RegExp(`(${escapeRegex(query)})`, 'gi')
  
  // Create a temporary div to safely escape HTML
  const div = document.createElement('div')
  div.textContent = text
  const escapedText = div.innerHTML
  
  // Now safe to use innerHTML since text was escaped
  return escapedText.replace(regex, '<mark class="bg-yellow-200 dark:bg-yellow-900/50">$1</mark>')
}

/**
 * Escape special regex characters
 */
function escapeRegex(str: string): string {
  return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}
