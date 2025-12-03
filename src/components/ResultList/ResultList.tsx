import { Component, For, Show, createEffect } from 'solid-js'
import type { SearchResult } from '../../types/search'
import { ResultItem } from './ResultItem'

interface ResultListProps {
  results: SearchResult[]
  selectedIndex: number
  onSelect: (index: number) => void
  onExecute: (result: SearchResult) => void
  loading?: boolean
  emptyMessage?: string
}

/**
 * Result list component with virtual scrolling support and keyboard navigation
 */
export const ResultList: Component<ResultListProps> = (props) => {
  let containerRef: HTMLDivElement | undefined

  // Scroll to selected item
  createEffect(() => {
    if (!containerRef) return

    const selectedIndex = props.selectedIndex
    const item = containerRef.children[selectedIndex] as HTMLElement
    if (item) {
      // Check if item is in view
      const containerRect = containerRef.getBoundingClientRect()
      const itemRect = item.getBoundingClientRect()

      if (itemRect.bottom > containerRect.bottom) {
        // Scroll down
        item.scrollIntoView({ block: 'end', behavior: 'smooth' })
      } else if (itemRect.top < containerRect.top) {
        // Scroll up
        item.scrollIntoView({ block: 'start', behavior: 'smooth' })
      }
    }
  })

  return (
    <div class="mt-2 overflow-hidden rounded-xl bg-white/90 shadow-lg backdrop-blur-sm dark:bg-gray-800/90">
      <Show
        when={!props.loading && props.results.length > 0}
        fallback={
          <div class="p-8 text-center">
            <Show
              when={!props.loading}
              fallback={
                <div class="flex items-center justify-center gap-2 text-gray-500 dark:text-gray-400">
                  <div class="h-4 w-4 animate-spin rounded-full border-2 border-gray-300 border-t-blue-500" />
                  <span>Searching...</span>
                </div>
              }
            >
              <div class="text-gray-500 dark:text-gray-400">
                {props.emptyMessage || 'No results found'}
              </div>
            </Show>
          </div>
        }
      >
        <div
          ref={containerRef}
          class="max-h-[480px] overflow-y-auto p-2"
          style={{
            'scrollbar-width': 'thin',
            'scrollbar-color': 'rgb(156, 163, 175) transparent',
          }}
        >
          <For each={props.results}>
            {(result, index) => (
              <ResultItem
                result={result}
                isSelected={index() === props.selectedIndex}
                index={index()}
                onClick={() => props.onExecute(result)}
                onDoubleClick={() => props.onExecute(result)}
                onMouseEnter={() => props.onSelect(index())}
              />
            )}
          </For>
        </div>

        {/* Result Count */}
        <div class="border-t border-gray-200 px-3 py-2 text-xs text-gray-500 dark:border-gray-700 dark:text-gray-400">
          {props.results.length} result{props.results.length !== 1 ? 's' : ''}
        </div>
      </Show>
    </div>
  )
}
