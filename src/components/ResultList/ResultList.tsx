import { Component, For, Show, createEffect, createMemo } from 'solid-js'
import type { SearchResult } from '../../types/search'
import { ResultItem } from './ResultItem'

interface ResultListProps {
  results: SearchResult[]
  selectedIndex: number
  onSelect: (index: number) => void
  onExecute: (result: SearchResult) => void
  loading?: boolean
  emptyMessage?: string
  query?: string
}

// Group header component
const GroupHeader: Component<{ title: string }> = (props) => (
  <div class="px-3 py-1.5 text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
    {props.title}
  </div>
)

/**
 * Result list component with category grouping and keyboard navigation
 * NOTE: Parent component MUST wrap this in <Show when={results.length > 0}> to prevent ghost containers
 */
export const ResultList: Component<ResultListProps> = (props) => {
  let scrollContainerRef: HTMLDivElement | undefined

  // Group results by category
  const groupedResults = createMemo(() => {
    const apps = props.results.filter(r => r.category === 'Application')
    const files = props.results.filter(r => r.category === 'File')
    const others = props.results.filter(r => 
      r.category !== 'Application' && r.category !== 'File'
    )
    return { apps, files, others }
  })

  // Scroll to selected item when selection changes
  createEffect(() => {
    if (!scrollContainerRef) return

    const selectedIndex = props.selectedIndex
    const items = scrollContainerRef.querySelectorAll('[data-result-index]')
    const selectedItem = Array.from(items).find(
      el => el.getAttribute('data-result-index') === String(selectedIndex)
    ) as HTMLElement | undefined
    
    if (selectedItem) {
      const containerRect = scrollContainerRef.getBoundingClientRect()
      const itemRect = selectedItem.getBoundingClientRect()

      if (itemRect.bottom > containerRect.bottom - 8) {
        selectedItem.scrollIntoView({ block: 'end', behavior: 'smooth' })
      } else if (itemRect.top < containerRect.top + 8) {
        selectedItem.scrollIntoView({ block: 'start', behavior: 'smooth' })
      }
    }
  })

  return (
    <div class="flex flex-col overflow-hidden">
      {/* Scrollable list area - flex-1 + min-h-0 is critical for proper overflow */}
      <div
        ref={scrollContainerRef}
        class="max-h-[400px] min-h-0 flex-1 overflow-y-auto"
        style={{
          'scrollbar-width': 'thin',
          'scrollbar-color': 'rgb(156, 163, 175) transparent',
        }}
      >
        {/* Applications Section */}
        <Show when={groupedResults().apps.length > 0}>
          <GroupHeader title="APPLICATIONS" />
          <div class="px-2">
            <For each={groupedResults().apps}>
              {(result, index) => {
                const globalIndex = index()
                return (
                  <div data-result-index={globalIndex}>
                    <ResultItem
                      result={result}
                      isSelected={globalIndex === props.selectedIndex}
                      index={globalIndex}
                      onClick={() => props.onExecute(result)}
                      onDoubleClick={() => props.onExecute(result)}
                      onMouseEnter={() => props.onSelect(globalIndex)}
                    />
                  </div>
                )
              }}
            </For>
          </div>
        </Show>

        {/* Files Section */}
        <Show when={groupedResults().files.length > 0}>
          <GroupHeader title="FILES" />
          <div class="px-2">
            <For each={groupedResults().files}>
              {(result, index) => {
                const globalIndex = groupedResults().apps.length + index()
                return (
                  <div data-result-index={globalIndex}>
                    <ResultItem
                      result={result}
                      isSelected={globalIndex === props.selectedIndex}
                      index={globalIndex}
                      onClick={() => props.onExecute(result)}
                      onDoubleClick={() => props.onExecute(result)}
                      onMouseEnter={() => props.onSelect(globalIndex)}
                    />
                  </div>
                )
              }}
            </For>
          </div>
        </Show>

        {/* Other Results Section */}
        <Show when={groupedResults().others.length > 0}>
          <GroupHeader title="OTHER" />
          <div class="px-2">
            <For each={groupedResults().others}>
              {(result, index) => {
                const globalIndex = groupedResults().apps.length + groupedResults().files.length + index()
                return (
                  <div data-result-index={globalIndex}>
                    <ResultItem
                      result={result}
                      isSelected={globalIndex === props.selectedIndex}
                      index={globalIndex}
                      onClick={() => props.onExecute(result)}
                      onDoubleClick={() => props.onExecute(result)}
                      onMouseEnter={() => props.onSelect(globalIndex)}
                    />
                  </div>
                )
              }}
            </For>
          </div>
        </Show>

        {/* Bottom spacer to prevent clipping */}
        <div class="h-3 flex-shrink-0" />
      </div>

      {/* Fixed footer - Action Bar */}
      <div class="flex flex-shrink-0 items-center justify-between border-t border-gray-200/50 px-3 py-1.5 text-xs text-gray-500 dark:border-gray-700/50 dark:text-gray-400">
        <div>
          <Show when={groupedResults().apps.length > 0}>
            <span class="mr-3">{groupedResults().apps.length} app{groupedResults().apps.length !== 1 ? 's' : ''}</span>
          </Show>
          <Show when={groupedResults().files.length > 0}>
            <span>{groupedResults().files.length} file{groupedResults().files.length !== 1 ? 's' : ''}</span>
          </Show>
        </div>
        <div class="flex items-center gap-3">
          <span><kbd class="rounded bg-gray-100 px-1.5 py-0.5 text-[10px] font-medium dark:bg-gray-700">↑↓</kbd> Navigate</span>
          <span><kbd class="rounded bg-gray-100 px-1.5 py-0.5 text-[10px] font-medium dark:bg-gray-700">↵</kbd> Open</span>
          <span><kbd class="rounded bg-gray-100 px-1.5 py-0.5 text-[10px] font-medium dark:bg-gray-700">Esc</kbd> Close</span>
        </div>
      </div>
    </div>
  )
}
