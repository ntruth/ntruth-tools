import { Component, createSignal, createMemo, createEffect } from 'solid-js'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import type { SearchResult } from '../../types/search'
import { SearchInput } from '../../components/SearchBox'
import { ResultList } from '../../components/ResultList'
import { ActionBar } from '../../components/ActionBar'
import { useKeyboard, useDebounce } from '../../hooks'

/**
 * Main search window component
 */
export const MainPage: Component = () => {
  const currentWindow = getCurrentWindow()

  // State
  const [query, setQuery] = createSignal('')
  const [results, setResults] = createSignal<SearchResult[]>([])
  const [selectedIndex, setSelectedIndex] = createSignal(0)
  const [loading, setLoading] = createSignal(false)

  // Debounced query for search
  const debouncedQuery = useDebounce(query, 150)

  // Determine input type based on query
  const inputType = createMemo(() => {
    const q = query().trim()
    if (!q) return undefined

    // Check for web search prefixes
    const webSearchPrefixes: Record<string, string> = {
      'gg': 'Google Search',
      'bd': 'Baidu Search',
      'bi': 'Bing Search',
      'ddg': 'DuckDuckGo',
      'gh': 'GitHub',
      'so': 'Stack Overflow',
      'yt': 'YouTube',
      'tw': 'Twitter',
      'npm': 'NPM',
      'crate': 'Crates.io',
    }

    for (const [prefix, name] of Object.entries(webSearchPrefixes)) {
      if (q.startsWith(prefix + ' ')) {
        return name
      }
    }

    // Check for other prefixes
    if (q.startsWith('ai ')) return 'AI Query'
    if (q.startsWith('cb ')) return 'Clipboard Search'
    if (q.startsWith('bm ')) return 'Bookmark Search'
    if (q.startsWith('> ')) return 'System Command'
    if (q.startsWith('=') || /^[\d+\-*/().^%\s]+$/.test(q)) return 'Calculator'

    // Check if URL
    if (/^https?:\/\//.test(q) || /^www\./.test(q) || /\.[a-z]{2,}/.test(q)) {
      return 'Open URL'
    }

    return undefined
  })

  // Perform search when debounced query changes
  createEffect(async () => {
    const q = debouncedQuery()
    if (!q.trim()) {
      setResults([])
      setLoading(false)
      return
    }

    setLoading(true)
    try {
      const searchResults = await invoke<SearchResult[]>('search', { query: q })
      setResults(searchResults)
      setSelectedIndex(0)
    } catch (error) {
      console.error('Search error:', error)
      setResults([])
    } finally {
      setLoading(false)
    }
  })

  // Keyboard navigation
  useKeyboard({
    onArrowUp: () => {
      setSelectedIndex((prev) => Math.max(0, prev - 1))
    },
    onArrowDown: () => {
      setSelectedIndex((prev) => Math.min(results().length - 1, prev + 1))
    },
    onEnter: () => {
      const selected = results()[selectedIndex()]
      if (selected) {
        executeResult(selected)
      }
    },
    onEscape: () => {
      hideWindow()
    },
    onCommand1: () => executeAtIndex(0),
    onCommand2: () => executeAtIndex(1),
    onCommand3: () => executeAtIndex(2),
    onCommand4: () => executeAtIndex(3),
    onCommand5: () => executeAtIndex(4),
    onCommand6: () => executeAtIndex(5),
    onCommand7: () => executeAtIndex(6),
    onCommand8: () => executeAtIndex(7),
    onCommand9: () => executeAtIndex(8),
  })

  // Execute result at specific index
  const executeAtIndex = (index: number) => {
    const result = results()[index]
    if (result) {
      executeResult(result)
    }
  }

  // Execute selected result
  const executeResult = async (result: SearchResult) => {
    try {
      // Execute based on action type
      switch (result.action.type) {
        case 'open':
          await invoke('open_path', { path: result.path })
          break
        case 'copy':
          // Copy to clipboard
          // TODO: Implement clipboard copy
          break
        case 'execute':
          // Execute command
          // TODO: Implement command execution
          break
        case 'web-search':
          // Open web search
          await invoke('open_path', { path: result.action.payload })
          break
        case 'ai-query':
          // Open AI chat
          await invoke('show_window', { label: 'ai-chat' })
          break
      }

      // Hide window after execution
      hideWindow()
    } catch (error) {
      console.error('Execute error:', error)
    }
  }

  // Hide window
  const hideWindow = async () => {
    await currentWindow.hide()
    // Reset state
    setQuery('')
    setResults([])
    setSelectedIndex(0)
  }

  // Clear search
  const handleClear = () => {
    setQuery('')
    setResults([])
    setSelectedIndex(0)
  }

  return (
    <div 
      class="flex h-full w-full items-start justify-center pt-32"
      data-tauri-drag-region
    >
      <div class="w-full max-w-2xl px-4">
        {/* Search Input */}
        <SearchInput
          value={query()}
          onInput={setQuery}
          onClear={handleClear}
          placeholder="Search files, apps, or type a command..."
          inputType={inputType()}
          autofocus
        />

        {/* Results List */}
        <ResultList
          results={results()}
          selectedIndex={selectedIndex()}
          onSelect={setSelectedIndex}
          onExecute={executeResult}
          loading={loading()}
          emptyMessage={query() ? 'No results found' : 'Start typing to search...'}
        />

        {/* Action Bar */}
        <ActionBar visible={results().length > 0} />
      </div>
    </div>
  )
}
