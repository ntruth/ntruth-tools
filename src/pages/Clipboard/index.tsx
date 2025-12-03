import { Component, For, createSignal, onMount, Show, createEffect, onCleanup } from 'solid-js'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'
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
  const [selectedIndex, setSelectedIndex] = createSignal(0)
  const [searchQuery, setSearchQuery] = createSignal('')
  const [loading, setLoading] = createSignal(true)
  const [showPreview, setShowPreview] = createSignal(true)
  let searchInputRef: HTMLInputElement | undefined

  // Computed selected item
  const selectedItem = () => {
    const filtered = filteredItems()
    return filtered[selectedIndex()] || null
  }

  // Load clipboard history on mount
  onMount(async () => {
    await loadHistory()
    
    // Focus search input
    searchInputRef?.focus()
    
    // Listen for clipboard changes
    const unlisten = await listen('clipboard-changed', async () => {
      await loadHistory()
    })
    
    onCleanup(() => {
      unlisten()
    })
  })

  // Keyboard navigation
  createEffect(() => {
    const handleKeyDown = async (e: KeyboardEvent) => {
      const filtered = filteredItems()
      
      switch (e.key) {
        case 'ArrowUp':
          e.preventDefault()
          setSelectedIndex(prev => Math.max(0, prev - 1))
          break
        case 'ArrowDown':
          e.preventDefault()
          setSelectedIndex(prev => Math.min(filtered.length - 1, prev + 1))
          break
        case 'Enter':
          e.preventDefault()
          const item = selectedItem()
          if (item) {
            await handlePaste(item)
          }
          break
        case 'Escape':
          e.preventDefault()
          await hideWindow()
          break
        case 'Delete':
        case 'Backspace':
          if (e.metaKey || e.ctrlKey) {
            e.preventDefault()
            const item = selectedItem()
            if (item) {
              await handleDelete(item)
            }
          }
          break
        case 'f':
          if (e.metaKey || e.ctrlKey) {
            e.preventDefault()
            const item = selectedItem()
            if (item) {
              await handleToggleFavorite(item)
            }
          }
          break
        case 'p':
          if (e.metaKey || e.ctrlKey) {
            e.preventDefault()
            setShowPreview(prev => !prev)
          }
          break
        default:
          // Number shortcuts (1-9) for quick paste
          if (e.key >= '1' && e.key <= '9' && (e.metaKey || e.ctrlKey)) {
            e.preventDefault()
            const index = parseInt(e.key) - 1
            if (index < filtered.length) {
              await handlePaste(filtered[index])
            }
          }
      }
    }
    
    document.addEventListener('keydown', handleKeyDown)
    onCleanup(() => document.removeEventListener('keydown', handleKeyDown))
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

  const hideWindow = async () => {
    const window = getCurrentWindow()
    await window.hide()
  }

  const handleItemClick = (_item: ClipboardItemData, index: number) => {
    setSelectedIndex(index)
  }

  const handlePaste = async (item: ClipboardItemData) => {
    try {
      await invoke('paste_clipboard_item', { id: item.id })
      // Close window after paste
      await hideWindow()
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
      // Reset selection if the deleted item was selected
      if (selectedItem()?.id === item.id) {
        setSelectedIndex(0)
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

  // Reset selected index when search changes
  createEffect(() => {
    searchQuery()
    setSelectedIndex(0)
  })

  return (
    <div class="flex h-full w-full flex-col bg-white dark:bg-gray-900">
      {/* Header with search */}
      <div class="border-b border-gray-200 p-3 dark:border-gray-700" data-tauri-drag-region>
        <input
          ref={searchInputRef}
          type="text"
          placeholder="Search clipboard... (Type to filter)"
          value={searchQuery()}
          onInput={(e) => setSearchQuery(e.currentTarget.value)}
          class="w-full rounded-lg border border-gray-300 px-4 py-2 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
        />
      </div>

      {/* Main content area */}
      <div class="flex flex-1 overflow-hidden">
        {/* Clipboard items list */}
        <div class={`flex-1 overflow-y-auto ${showPreview() ? 'w-1/2' : 'w-full'}`}>
          <Show
            when={!loading()}
            fallback={
              <div class="flex h-full items-center justify-center text-gray-500">
                <div class="animate-pulse">Loading...</div>
              </div>
            }
          >
            <Show
              when={filteredItems().length > 0}
              fallback={
                <div class="flex h-full flex-col items-center justify-center text-gray-500">
                  <div class="text-4xl mb-2">📋</div>
                  <div>No clipboard items</div>
                  <div class="text-xs mt-1">Copy something to get started</div>
                </div>
              }
            >
              <For each={filteredItems()}>
                {(item, index) => (
                  <ClipboardItem
                    item={item}
                    index={index()}
                    selected={selectedIndex() === index()}
                    onClick={() => handleItemClick(item, index())}
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
        <Show when={showPreview() && selectedItem()}>
          {(item) => (
            <div class="w-1/2 border-l border-gray-200 dark:border-gray-700">
              <ClipboardPreview item={item()} />
            </div>
          )}
        </Show>
      </div>

      {/* Keyboard shortcuts hint */}
      <div class="border-t border-gray-200 bg-gray-50 px-4 py-2 text-xs text-gray-600 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 flex items-center justify-between">
        <div>
          <span class="mr-3">
            <kbd class="rounded bg-white px-1.5 py-0.5 shadow-sm dark:bg-gray-700">↑↓</kbd> Navigate
          </span>
          <span class="mr-3">
            <kbd class="rounded bg-white px-1.5 py-0.5 shadow-sm dark:bg-gray-700">Enter</kbd> Paste
          </span>
          <span class="mr-3">
            <kbd class="rounded bg-white px-1.5 py-0.5 shadow-sm dark:bg-gray-700">⌘1-9</kbd> Quick
          </span>
          <span class="mr-3">
            <kbd class="rounded bg-white px-1.5 py-0.5 shadow-sm dark:bg-gray-700">⌘F</kbd> Favorite
          </span>
          <span>
            <kbd class="rounded bg-white px-1.5 py-0.5 shadow-sm dark:bg-gray-700">Esc</kbd> Close
          </span>
        </div>
        <div class="text-gray-400">
          {filteredItems().length} items
        </div>
      </div>
    </div>
  )
}

export default ClipboardPage
