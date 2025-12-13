import { Component, createSignal, createMemo, createEffect, Show } from 'solid-js'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'
import type { SearchResult } from '../../types/search'
import { SearchInput } from '../../components/SearchBox'
import { ResultList } from '../../components/ResultList'
import { ActionBar } from '../../components/ActionBar'
import { AIChatView } from '../../components/AIChatView'
import { useKeyboard, useDebounce } from '../../hooks'

// Window size constants
const SEARCH_BAR_HEIGHT = 60
const RESULT_ITEM_HEIGHT = 48
const MAX_VISIBLE_RESULTS = 8
const WINDOW_WIDTH = 680
const PADDING = 16

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
  
  // AI Instant Query State
  const [aiMode, setAiMode] = createSignal(false)
  const [aiQueryId, setAiQueryId] = createSignal<string | null>(null)
  const [aiQuestion, setAiQuestion] = createSignal('')
  const [aiLoading, setAiLoading] = createSignal(false)

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
  
  // Check if current query is AI mode
  const isAiQuery = createMemo(() => query().trim().toLowerCase().startsWith('ai '))
  
  // Get the AI question without the prefix
  const getAiQuestionText = () => {
    const q = query().trim()
    if (q.toLowerCase().startsWith('ai ')) {
      return q.slice(3).trim()
    }
    return q
  }

  // Perform search when debounced query changes
  // Perform search when debounced query changes
  createEffect(async () => {
    const q = debouncedQuery()
    if (!q.trim()) {
      setResults([])
      setLoading(false)
      return
    }
    
    // Don't search if in AI mode
    if (q.trim().toLowerCase().startsWith('ai ') && aiMode()) {
      return
    }

    setLoading(true)
    try {
      // Check for built-in commands first
      const builtinResults = getBuiltinResults(q.toLowerCase())
      
      const searchResults = await invoke<SearchResult[]>('search', { query: q })
      // Prepend builtin results
      setResults([...builtinResults, ...searchResults])
      setSelectedIndex(0)
    } catch (error) {
      console.error('Search error:', error)
      setResults([])
    } finally {
      setLoading(false)
    }
  })
  
  // Execute AI instant query
  const executeAiQuery = async () => {
    const question = getAiQuestionText()
    if (!question) return
    
    setAiMode(true)
    setAiQuestion(question)
    setAiLoading(true)
    
    try {
      const queryId = await invoke<string>('ai_quick_query', { prompt: question })
      setAiQueryId(queryId)
    } catch (error) {
      console.error('AI query error:', error)
      setAiQueryId(null)
    } finally {
      setAiLoading(false)
    }
  }
  
  // Exit AI mode
  const exitAiMode = () => {
    setAiMode(false)
    setAiQueryId(null)
    setAiQuestion('')
    setQuery('')
  }

  // Get builtin command results
  const getBuiltinResults = (query: string): SearchResult[] => {
    const builtins: SearchResult[] = []
    
    // Settings/Preferences command
    if ('settings'.includes(query) || 'preferences'.includes(query) || '设置'.includes(query)) {
      builtins.push({
        id: 'builtin-settings',
        title: 'Preferences',
        subtitle: 'Open OmniBox Settings',
        icon: '⚙️',
        category: 'system',
        path: '',
        score: 100,
        action: { type: 'settings' },
      })
    }
    
    // Clipboard command
    if ('clipboard'.includes(query) || '剪贴板'.includes(query)) {
      builtins.push({
        id: 'builtin-clipboard',
        title: 'Clipboard History',
        subtitle: 'View clipboard history (⌘⇧V)',
        icon: '📋',
        category: 'system',
        path: '',
        score: 100,
        action: { type: 'clipboard' },
      })
    }
    
    // AI Chat command
    if ('ai'.includes(query) || 'chat'.includes(query)) {
      builtins.push({
        id: 'builtin-ai',
        title: 'AI Chat',
        subtitle: 'Start an AI conversation',
        icon: '🤖',
        category: 'system',
        path: '',
        score: 100,
        action: { type: 'ai-query' },
      })
    }
    
    return builtins
  }

  // Keyboard navigation
  useKeyboard({
    onArrowUp: () => {
      if (!aiMode()) {
        setSelectedIndex((prev) => Math.max(0, prev - 1))
      }
    },
    onArrowDown: () => {
      if (!aiMode()) {
        setSelectedIndex((prev) => Math.min(results().length - 1, prev + 1))
      }
    },
    onEnter: () => {
      // If in AI query mode (typing "ai xxx"), execute AI query
      if (isAiQuery() && !aiMode()) {
        executeAiQuery()
        return
      }
      
      // Normal search result execution
      const selected = results()[selectedIndex()]
      if (selected) {
        executeResult(selected)
      }
    },
    onEscape: () => {
      // If in AI mode, exit AI mode first
      if (aiMode()) {
        exitAiMode()
        return
      }
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
    console.log('executeResult called with:', result)
    console.log('action type:', result.action?.type)
    try {
      // Track if we need to hide the main window manually
      // (show_window for settings/ai/clipboard already handles hiding main window)
      let shouldHideManually = true
      
      const actionType = result.action?.type
      console.log('Switching on action type:', actionType)
      
      // Execute based on action type
      switch (actionType) {
        case 'open':
          console.log('Opening path:', result.path)
          // Hide window FIRST before opening app (faster user experience)
          await hideWindow()
          shouldHideManually = false
          // Then open the path
          invoke('open_path', { path: result.path }).catch(console.error)
          break
        case 'copy':
          // Copy to clipboard using Tauri clipboard API
          if (result.action.payload) {
            const { writeText } = await import('@tauri-apps/plugin-clipboard-manager')
            await writeText(result.action.payload)
          }
          break
        case 'execute':
          // Execute command (TODO: implement shell command execution)
          console.log('Execute command:', result.action.payload)
          break
        case 'web-search':
          // Hide window FIRST before opening URL (faster user experience)
          await hideWindow()
          shouldHideManually = false
          // Open web search URL
          if (result.action.payload) {
            invoke('open_path', { path: result.action.payload }).catch(console.error)
          }
          break
        case 'ai-query':
          // Open AI chat (backend handles hiding main window)
          await invoke('show_window', { label: 'ai' })
          shouldHideManually = false
          break
        case 'clipboard':
          // Open clipboard window (backend handles hiding main window)
          await invoke('show_window', { label: 'clipboard' })
          shouldHideManually = false
          break
        case 'settings':
          // Open settings window (backend handles hiding main window)
          await invoke('show_window', { label: 'settings' })
          shouldHideManually = false
          break
        default:
          console.log('Unknown action type, trying to open path:', result.path)
          // Fallback: try to open path if available
          if (result.path) {
            await hideWindow()
            shouldHideManually = false
            invoke('open_path', { path: result.path }).catch(console.error)
          }
          break
      }

      // Hide window after execution (only if not already handled)
      if (shouldHideManually) {
        hideWindow()
      }
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
    // Also reset AI mode
    setAiMode(false)
    setAiQueryId(null)
    setAiQuestion('')
  }

  // Clear search
  const handleClear = () => {
    setQuery('')
    setResults([])
    setSelectedIndex(0)
    // Also reset AI mode
    if (aiMode()) {
      exitAiMode()
    }
  }

  // Whether to show results (only when user has typed something)
  const showResults = createMemo(() => query().trim().length > 0 && !aiMode())
  
  // Dynamically resize window based on results
  createEffect(async () => {
    const resultCount = results().length
    const hasResults = showResults() && resultCount > 0
    const isAi = aiMode()
    
    let newHeight = SEARCH_BAR_HEIGHT + PADDING
    
    if (hasResults) {
      // Calculate height based on result count (max 8 visible)
      const visibleCount = Math.min(resultCount, MAX_VISIBLE_RESULTS)
      newHeight = SEARCH_BAR_HEIGHT + (visibleCount * RESULT_ITEM_HEIGHT) + PADDING + 40 // 40 for action bar
    } else if (isAi) {
      // AI mode needs more space
      newHeight = 450
    }
    
    try {
      await currentWindow.setSize(new LogicalSize(WINDOW_WIDTH, newHeight))
    } catch (e) {
      console.error('Failed to resize window:', e)
    }
  })

  return (
    <div class="flex h-full w-full flex-col bg-transparent p-2">
      {/* Search Input - already has its own styling */}
      <div class="search-box-wrapper">
        <SearchInput
          value={query()}
          onInput={setQuery}
          onClear={handleClear}
          placeholder="Search files, apps, or type a command..."
          inputType={inputType()}
          autofocus
        />
      </div>

      {/* AI Chat View - Show when in AI mode */}
      <Show when={aiMode()}>
        <div class="mt-2 max-h-96 overflow-y-auto rounded-xl bg-white/95 shadow-lg backdrop-blur-xl dark:bg-gray-900/95">
          <AIChatView
            queryId={aiQueryId()}
            question={aiQuestion()}
            isLoading={aiLoading()}
          />
        </div>
        <div class="mt-2 text-center text-xs text-gray-500">
          按 <kbd class="rounded bg-gray-200 px-1.5 py-0.5 dark:bg-gray-700">Esc</kbd> 退出 AI 模式
        </div>
      </Show>

      {/* Results List - Only show when there's a query and not in AI mode */}
      <Show when={showResults()}>
        <div class="mt-2 overflow-hidden rounded-xl bg-white/95 shadow-lg backdrop-blur-xl dark:bg-gray-900/95">
          <ResultList
            results={results()}
            selectedIndex={selectedIndex()}
            onSelect={setSelectedIndex}
            onExecute={executeResult}
            loading={loading()}
            emptyMessage="No results found"
          />

          {/* Action Bar */}
          <ActionBar visible={results().length > 0} />
        </div>
      </Show>
    </div>
  )
}
