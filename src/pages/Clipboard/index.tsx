import { Component, For, createSignal, onMount, Show } from 'solid-js'
import { invoke } from '@tauri-apps/api/core'
import ClipboardItem from './ClipboardItem'
import ClipboardPreview from './ClipboardPreview'

export interface ClipboardItemData {
  id: string
  type: string
  content: string
  timestamp: number
  favorite: boolean
}

const ClipboardPage: Component = () => {
  const [items, setItems] = createSignal<ClipboardItemData[]>([])
  const [selectedItem, setSelectedItem] = createSignal<ClipboardItemData | null>(null)
  const [searchQuery, setSearchQuery] = createSignal('')
  const [loading, setLoading] = createSignal(true)

  // Load clipboard history on mount
  onMount(async () => {
    await loadHistory()
  })

  const loadHistory = async () => {
    try {
      setLoading(true)
      const history = await invoke<ClipboardItemData[]>('get_clipboard_history')
      setItems(history)
    } catch (error) {
      console.error('Failed to load clipboard history:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleItemClick = (item: ClipboardItemData) => {
    setSelectedItem(item)
  }

  const handlePaste = async (item: ClipboardItemData) => {
    try {
      await invoke('paste_clipboard_item', { id: item.id })
      // Close window after paste
      await invoke('hide_clipboard_window')
    } catch (error) {
      console.error('Failed to paste item:', error)
    }
  }

  const handleToggleFavorite = async (item: ClipboardItemData) => {
    try {
      await invoke('toggle_clipboard_favorite', { id: item.id })
      await loadHistory()
    } catch (error) {
      console.error('Failed to toggle favorite:', error)
    }
  }

  const handleDelete = async (item: ClipboardItemData) => {
    try {
      await invoke('delete_clipboard_item', { id: item.id })
      await loadHistory()
      if (selectedItem()?.id === item.id) {
        setSelectedItem(null)
      }
    } catch (error) {
      console.error('Failed to delete item:', error)
    }
  }

  const filteredItems = () => {
    const query = searchQuery().toLowerCase()
    if (!query) return items()
    return items().filter((item) =>
      item.content.toLowerCase().includes(query)
    )
  }

  return (
    <div class="flex h-full w-full flex-col bg-white dark:bg-gray-900">
      {/* Header with search */}
      <div class="border-b border-gray-200 p-4 dark:border-gray-700">
        <input
          type="text"
          placeholder="Search clipboard..."
          value={searchQuery()}
          onInput={(e) => setSearchQuery(e.currentTarget.value)}
          class="w-full rounded-lg border border-gray-300 px-4 py-2 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
        />
      </div>

      {/* Clipboard items list */}
      <div class="flex-1 overflow-y-auto">
        <Show
          when={!loading()}
          fallback={
            <div class="flex h-full items-center justify-center text-gray-500">
              Loading...
            </div>
          }
        >
          <Show
            when={filteredItems().length > 0}
            fallback={
              <div class="flex h-full items-center justify-center text-gray-500">
                No clipboard items
              </div>
            }
          >
            <For each={filteredItems()}>
              {(item) => (
                <ClipboardItem
                  item={item}
                  selected={selectedItem()?.id === item.id}
                  onClick={() => handleItemClick(item)}
                  onPaste={() => handlePaste(item)}
                  onToggleFavorite={() => handleToggleFavorite(item)}
                  onDelete={() => handleDelete(item)}
                />
              )}
            </For>
          </Show>
        </Show>
      </div>

      {/* Preview panel */}
      <Show when={selectedItem()}>
        {(item) => <ClipboardPreview item={item()} />}
      </Show>

      {/* Keyboard shortcuts hint */}
      <div class="border-t border-gray-200 bg-gray-50 px-4 py-2 text-xs text-gray-600 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400">
        <span class="mr-4">
          <kbd class="rounded bg-white px-1.5 py-0.5 dark:bg-gray-700">↑↓</kbd> Navigate
        </span>
        <span class="mr-4">
          <kbd class="rounded bg-white px-1.5 py-0.5 dark:bg-gray-700">Enter</kbd> Paste
        </span>
        <span class="mr-4">
          <kbd class="rounded bg-white px-1.5 py-0.5 dark:bg-gray-700">⌘D</kbd> Delete
        </span>
        <span>
          <kbd class="rounded bg-white px-1.5 py-0.5 dark:bg-gray-700">Esc</kbd> Close
        </span>
      </div>
    </div>
  )
}

export default ClipboardPage
