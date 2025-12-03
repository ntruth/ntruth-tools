import { Component, For, createSignal } from 'solid-js'
import { Search, Plus, Trash2, GripVertical } from 'lucide-solid'

interface SearchEngine {
  name: string
  keyword: string
  url: string
  enabled: boolean
}

interface WebSearchConfig {
  default_engine: string
  engines: SearchEngine[]
}

interface WebSearchProps {
  config: WebSearchConfig
  onChange: (updates: Partial<WebSearchConfig>) => void
}

const DEFAULT_ENGINES: SearchEngine[] = [
  {
    name: 'Google',
    keyword: 'g',
    url: 'https://www.google.com/search?q={query}',
    enabled: true,
  },
  {
    name: 'Bing',
    keyword: 'b',
    url: 'https://www.bing.com/search?q={query}',
    enabled: true,
  },
  {
    name: 'DuckDuckGo',
    keyword: 'ddg',
    url: 'https://duckduckgo.com/?q={query}',
    enabled: true,
  },
  {
    name: 'GitHub',
    keyword: 'gh',
    url: 'https://github.com/search?q={query}',
    enabled: true,
  },
  {
    name: 'Stack Overflow',
    keyword: 'so',
    url: 'https://stackoverflow.com/search?q={query}',
    enabled: true,
  },
  {
    name: 'YouTube',
    keyword: 'yt',
    url: 'https://www.youtube.com/results?search_query={query}',
    enabled: true,
  },
]

const WebSearch: Component<WebSearchProps> = (props) => {
  const [showAddForm, setShowAddForm] = createSignal(false)
  const [newEngine, setNewEngine] = createSignal<SearchEngine>({
    name: '',
    keyword: '',
    url: '',
    enabled: true,
  })

  const engines = () =>
    props.config.engines.length > 0 ? props.config.engines : DEFAULT_ENGINES

  const addEngine = () => {
    const engine = newEngine()
    if (engine.name && engine.keyword && engine.url) {
      props.onChange({
        engines: [...engines(), engine],
      })
      setNewEngine({ name: '', keyword: '', url: '', enabled: true })
      setShowAddForm(false)
    }
  }

  const removeEngine = (keyword: string) => {
    props.onChange({
      engines: engines().filter((e) => e.keyword !== keyword),
    })
  }

  const toggleEngine = (keyword: string) => {
    props.onChange({
      engines: engines().map((e) =>
        e.keyword === keyword ? { ...e, enabled: !e.enabled } : e
      ),
    })
  }

  const setDefaultEngine = (keyword: string) => {
    props.onChange({ default_engine: keyword })
  }

  return (
    <div class="space-y-6">
      <div>
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Web Search Settings
        </h2>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Configure search engines for web search queries
        </p>
      </div>

      {/* Search Engines List */}
      <div class="space-y-3">
        <div class="flex items-center justify-between">
          <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300">
            Search Engines
          </h3>
          <button
            onClick={() => setShowAddForm(true)}
            class="flex items-center gap-1 rounded-lg bg-blue-500 px-3 py-1.5 text-sm text-white hover:bg-blue-600"
          >
            <Plus size={16} />
            Add Engine
          </button>
        </div>

        <For each={engines()}>
          {(engine) => (
            <div
              class={`flex items-center gap-3 rounded-lg border p-3 ${
                engine.enabled
                  ? 'border-gray-200 dark:border-gray-700'
                  : 'border-gray-100 bg-gray-50 opacity-60 dark:border-gray-800 dark:bg-gray-800/50'
              }`}
            >
              <div class="cursor-move text-gray-400">
                <GripVertical size={16} />
              </div>

              <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900">
                <Search size={18} class="text-blue-600 dark:text-blue-400" />
              </div>

              <div class="flex-1">
                <div class="flex items-center gap-2">
                  <span class="font-medium text-gray-900 dark:text-white">
                    {engine.name}
                  </span>
                  <span class="rounded bg-gray-200 px-1.5 py-0.5 text-xs font-mono text-gray-600 dark:bg-gray-700 dark:text-gray-400">
                    {engine.keyword}
                  </span>
                  {props.config.default_engine === engine.keyword && (
                    <span class="rounded bg-blue-100 px-1.5 py-0.5 text-xs text-blue-700 dark:bg-blue-900 dark:text-blue-300">
                      Default
                    </span>
                  )}
                </div>
                <p class="mt-0.5 text-xs text-gray-500 dark:text-gray-400 truncate max-w-md">
                  {engine.url}
                </p>
              </div>

              <div class="flex items-center gap-2">
                {props.config.default_engine !== engine.keyword && (
                  <button
                    onClick={() => setDefaultEngine(engine.keyword)}
                    class="rounded px-2 py-1 text-xs text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-700"
                  >
                    Set Default
                  </button>
                )}

                <button
                  onClick={() => toggleEngine(engine.keyword)}
                  class={`relative h-5 w-9 rounded-full transition-colors ${
                    engine.enabled ? 'bg-blue-500' : 'bg-gray-300'
                  }`}
                >
                  <span
                    class={`absolute top-0.5 h-4 w-4 rounded-full bg-white transition-transform ${
                      engine.enabled ? 'left-4' : 'left-0.5'
                    }`}
                  />
                </button>

                <button
                  onClick={() => removeEngine(engine.keyword)}
                  class="p-1 text-gray-400 hover:text-red-500"
                >
                  <Trash2 size={16} />
                </button>
              </div>
            </div>
          )}
        </For>
      </div>

      {/* Add Engine Form */}
      {showAddForm() && (
        <div class="rounded-lg border border-blue-200 bg-blue-50 p-4 dark:border-blue-800 dark:bg-blue-900/20">
          <h4 class="mb-4 font-medium text-gray-900 dark:text-white">
            Add Search Engine
          </h4>
          <div class="space-y-3">
            <div>
              <label class="block text-sm text-gray-600 dark:text-gray-400">
                Name
              </label>
              <input
                type="text"
                value={newEngine().name}
                onInput={(e) =>
                  setNewEngine({ ...newEngine(), name: e.currentTarget.value })
                }
                placeholder="Google"
                class="mt-1 w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800"
              />
            </div>
            <div>
              <label class="block text-sm text-gray-600 dark:text-gray-400">
                Keyword (trigger)
              </label>
              <input
                type="text"
                value={newEngine().keyword}
                onInput={(e) =>
                  setNewEngine({ ...newEngine(), keyword: e.currentTarget.value })
                }
                placeholder="g"
                class="mt-1 w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800"
              />
            </div>
            <div>
              <label class="block text-sm text-gray-600 dark:text-gray-400">
                URL (use {'{query}'} as placeholder)
              </label>
              <input
                type="text"
                value={newEngine().url}
                onInput={(e) =>
                  setNewEngine({ ...newEngine(), url: e.currentTarget.value })
                }
                placeholder="https://www.google.com/search?q={query}"
                class="mt-1 w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800"
              />
            </div>
            <div class="flex justify-end gap-2">
              <button
                onClick={() => setShowAddForm(false)}
                class="rounded-lg px-4 py-2 text-sm text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-700"
              >
                Cancel
              </button>
              <button
                onClick={addEngine}
                class="rounded-lg bg-blue-500 px-4 py-2 text-sm text-white hover:bg-blue-600"
              >
                Add
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Usage Guide */}
      <div class="rounded-lg bg-gray-50 p-4 dark:bg-gray-800/50">
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300">
          How to use
        </h3>
        <ul class="mt-2 space-y-1 text-sm text-gray-600 dark:text-gray-400">
          <li>
            • Type a keyword followed by your search query (e.g., <code class="bg-gray-200 px-1 rounded dark:bg-gray-700">g react hooks</code>)
          </li>
          <li>
            • The default engine is used when no keyword is specified
          </li>
          <li>
            • Press Enter to search with the selected engine
          </li>
        </ul>
      </div>
    </div>
  )
}

export default WebSearch
