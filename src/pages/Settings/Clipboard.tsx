import { Component, For, createSignal } from 'solid-js'
import { Plus, Trash2 } from 'lucide-solid'

interface ClipboardConfig {
  enabled: boolean
  history_limit: number
  filter_sensitive: boolean
  exclude_apps: string[]
}

interface ClipboardProps {
  config: ClipboardConfig
  onChange: (updates: Partial<ClipboardConfig>) => void
}

const Clipboard: Component<ClipboardProps> = (props) => {
  const [newApp, setNewApp] = createSignal('')

  const addExcludedApp = () => {
    const app = newApp().trim()
    if (app && !props.config.exclude_apps.includes(app)) {
      props.onChange({
        exclude_apps: [...props.config.exclude_apps, app],
      })
      setNewApp('')
    }
  }

  const removeExcludedApp = (app: string) => {
    props.onChange({
      exclude_apps: props.config.exclude_apps.filter((a) => a !== app),
    })
  }

  return (
    <div class="space-y-6">
      <div>
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Clipboard Settings
        </h2>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Configure clipboard history and filtering options
        </p>
      </div>

      {/* Enable Clipboard */}
      <div class="flex items-center justify-between rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div>
          <h3 class="text-sm font-medium text-gray-900 dark:text-white">
            Enable Clipboard History
          </h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            Track and store clipboard history
          </p>
        </div>
        <label class="relative inline-flex cursor-pointer items-center">
          <input
            type="checkbox"
            checked={props.config.enabled}
            onChange={(e) => props.onChange({ enabled: e.currentTarget.checked })}
            class="peer sr-only"
          />
          <div class="peer h-6 w-11 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-5 after:w-5 after:rounded-full after:border after:border-gray-300 after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-600 peer-checked:after:translate-x-full peer-checked:after:border-white dark:border-gray-600 dark:bg-gray-700"></div>
        </label>
      </div>

      {/* History Limit */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <h3 class="text-sm font-medium text-gray-900 dark:text-white">
          History Limit
        </h3>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Maximum number of items to keep in history
        </p>
        <div class="mt-3">
          <input
            type="number"
            min="10"
            max="10000"
            value={props.config.history_limit}
            onChange={(e) =>
              props.onChange({ history_limit: parseInt(e.currentTarget.value) || 1000 })
            }
            class="w-32 rounded-lg border border-gray-300 px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-800 dark:text-white"
          />
          <span class="ml-2 text-sm text-gray-500">items</span>
        </div>
      </div>

      {/* Filter Sensitive Content */}
      <div class="flex items-center justify-between rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div>
          <h3 class="text-sm font-medium text-gray-900 dark:text-white">
            Filter Sensitive Content
          </h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            Automatically detect and mark sensitive data (passwords, API keys, etc.)
          </p>
        </div>
        <label class="relative inline-flex cursor-pointer items-center">
          <input
            type="checkbox"
            checked={props.config.filter_sensitive}
            onChange={(e) =>
              props.onChange({ filter_sensitive: e.currentTarget.checked })
            }
            class="peer sr-only"
          />
          <div class="peer h-6 w-11 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-5 after:w-5 after:rounded-full after:border after:border-gray-300 after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-600 peer-checked:after:translate-x-full peer-checked:after:border-white dark:border-gray-600 dark:bg-gray-700"></div>
        </label>
      </div>

      {/* Excluded Apps */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <h3 class="text-sm font-medium text-gray-900 dark:text-white">
          Excluded Applications
        </h3>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Clipboard content from these apps will not be recorded
        </p>
        
        {/* Add new app */}
        <div class="mt-3 flex gap-2">
          <input
            type="text"
            placeholder="Application name (e.g., 1Password)"
            value={newApp()}
            onInput={(e) => setNewApp(e.currentTarget.value)}
            onKeyDown={(e) => e.key === 'Enter' && addExcludedApp()}
            class="flex-1 rounded-lg border border-gray-300 px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-800 dark:text-white"
          />
          <button
            onClick={addExcludedApp}
            class="flex items-center gap-1 rounded-lg bg-blue-600 px-3 py-2 text-sm text-white hover:bg-blue-700"
          >
            <Plus size={16} />
            Add
          </button>
        </div>

        {/* Excluded apps list */}
        <div class="mt-3 space-y-2">
          <For each={props.config.exclude_apps}>
            {(app) => (
              <div class="flex items-center justify-between rounded-lg bg-gray-100 px-3 py-2 dark:bg-gray-800">
                <span class="text-sm text-gray-700 dark:text-gray-300">{app}</span>
                <button
                  onClick={() => removeExcludedApp(app)}
                  class="rounded p-1 text-red-500 hover:bg-red-100 dark:hover:bg-red-900/30"
                >
                  <Trash2 size={14} />
                </button>
              </div>
            )}
          </For>
          {props.config.exclude_apps.length === 0 && (
            <p class="text-sm text-gray-400 italic">No excluded applications</p>
          )}
        </div>
      </div>
    </div>
  )
}

export default Clipboard
