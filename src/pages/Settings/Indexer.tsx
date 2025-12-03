import { Component, For, createSignal } from 'solid-js'
import { FolderSearch, Plus, Trash2, RotateCcw } from 'lucide-solid'

interface IndexerConfig {
  indexed_directories: string[]
  exclude_patterns: string[]
  include_extensions: string[]
  max_file_size: number
  enable_watcher: boolean
}

interface IndexerProps {
  config: IndexerConfig
  onChange: (updates: Partial<IndexerConfig>) => void
  onReindex?: () => void
}

const Indexer: Component<IndexerProps> = (props) => {
  const [newDirectory, setNewDirectory] = createSignal('')
  const [newPattern, setNewPattern] = createSignal('')
  const [newExtension, setNewExtension] = createSignal('')

  const addDirectory = () => {
    const dir = newDirectory().trim()
    if (dir && !props.config.indexed_directories.includes(dir)) {
      props.onChange({
        indexed_directories: [...props.config.indexed_directories, dir],
      })
      setNewDirectory('')
    }
  }

  const removeDirectory = (dir: string) => {
    props.onChange({
      indexed_directories: props.config.indexed_directories.filter(
        (d) => d !== dir
      ),
    })
  }

  const addPattern = () => {
    const pattern = newPattern().trim()
    if (pattern && !props.config.exclude_patterns.includes(pattern)) {
      props.onChange({
        exclude_patterns: [...props.config.exclude_patterns, pattern],
      })
      setNewPattern('')
    }
  }

  const removePattern = (pattern: string) => {
    props.onChange({
      exclude_patterns: props.config.exclude_patterns.filter(
        (p) => p !== pattern
      ),
    })
  }

  const addExtension = () => {
    let ext = newExtension().trim()
    if (!ext.startsWith('.')) ext = '.' + ext
    if (ext && !props.config.include_extensions.includes(ext)) {
      props.onChange({
        include_extensions: [...props.config.include_extensions, ext],
      })
      setNewExtension('')
    }
  }

  const removeExtension = (ext: string) => {
    props.onChange({
      include_extensions: props.config.include_extensions.filter(
        (e) => e !== ext
      ),
    })
  }

  return (
    <div class="space-y-6">
      <div>
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          File Indexer Settings
        </h2>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Configure which files and directories to index for search
        </p>
      </div>

      {/* Indexed Directories */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900">
              <FolderSearch size={20} class="text-blue-600 dark:text-blue-400" />
            </div>
            <div>
              <h3 class="text-sm font-medium text-gray-900 dark:text-white">
                Indexed Directories
              </h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">
                Directories to include in file search
              </p>
            </div>
          </div>
        </div>

        <div class="mt-4">
          <div class="flex gap-2">
            <input
              type="text"
              value={newDirectory()}
              onInput={(e) => setNewDirectory(e.currentTarget.value)}
              onKeyPress={(e) => e.key === 'Enter' && addDirectory()}
              placeholder="/path/to/directory"
              class="flex-1 rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800"
            />
            <button
              onClick={addDirectory}
              class="rounded-lg bg-blue-500 p-2 text-white hover:bg-blue-600"
            >
              <Plus size={20} />
            </button>
          </div>

          <div class="mt-3 space-y-2">
            <For each={props.config.indexed_directories}>
              {(dir) => (
                <div class="flex items-center justify-between rounded-lg bg-gray-100 px-3 py-2 dark:bg-gray-800">
                  <span class="text-sm text-gray-700 dark:text-gray-300">{dir}</span>
                  <button
                    onClick={() => removeDirectory(dir)}
                    class="text-gray-400 hover:text-red-500"
                  >
                    <Trash2 size={16} />
                  </button>
                </div>
              )}
            </For>
          </div>
        </div>
      </div>

      {/* Exclude Patterns */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <h3 class="text-sm font-medium text-gray-900 dark:text-white">
          Exclude Patterns
        </h3>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Glob patterns to exclude from indexing (e.g., node_modules, .git)
        </p>

        <div class="mt-4">
          <div class="flex gap-2">
            <input
              type="text"
              value={newPattern()}
              onInput={(e) => setNewPattern(e.currentTarget.value)}
              onKeyPress={(e) => e.key === 'Enter' && addPattern()}
              placeholder="e.g., **/node_modules/**"
              class="flex-1 rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800"
            />
            <button
              onClick={addPattern}
              class="rounded-lg bg-blue-500 p-2 text-white hover:bg-blue-600"
            >
              <Plus size={20} />
            </button>
          </div>

          <div class="mt-3 flex flex-wrap gap-2">
            <For each={props.config.exclude_patterns}>
              {(pattern) => (
                <div class="flex items-center gap-2 rounded-full bg-gray-100 px-3 py-1 dark:bg-gray-800">
                  <span class="text-sm text-gray-700 dark:text-gray-300">{pattern}</span>
                  <button
                    onClick={() => removePattern(pattern)}
                    class="text-gray-400 hover:text-red-500"
                  >
                    <Trash2 size={14} />
                  </button>
                </div>
              )}
            </For>
          </div>
        </div>
      </div>

      {/* File Extensions */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <h3 class="text-sm font-medium text-gray-900 dark:text-white">
          Include Extensions
        </h3>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Only index files with these extensions (leave empty for all)
        </p>

        <div class="mt-4">
          <div class="flex gap-2">
            <input
              type="text"
              value={newExtension()}
              onInput={(e) => setNewExtension(e.currentTarget.value)}
              onKeyPress={(e) => e.key === 'Enter' && addExtension()}
              placeholder="e.g., .txt, .md"
              class="flex-1 rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800"
            />
            <button
              onClick={addExtension}
              class="rounded-lg bg-blue-500 p-2 text-white hover:bg-blue-600"
            >
              <Plus size={20} />
            </button>
          </div>

          <div class="mt-3 flex flex-wrap gap-2">
            <For each={props.config.include_extensions}>
              {(ext) => (
                <div class="flex items-center gap-2 rounded-full bg-blue-100 px-3 py-1 dark:bg-blue-900">
                  <span class="text-sm text-blue-700 dark:text-blue-300">{ext}</span>
                  <button
                    onClick={() => removeExtension(ext)}
                    class="text-blue-400 hover:text-red-500"
                  >
                    <Trash2 size={14} />
                  </button>
                </div>
              )}
            </For>
          </div>
        </div>
      </div>

      {/* Max File Size */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-sm font-medium text-gray-900 dark:text-white">
              Max File Size
            </h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              Maximum file size to index (in MB)
            </p>
          </div>
          <div class="flex items-center gap-2">
            <input
              type="range"
              min="1"
              max="100"
              value={props.config.max_file_size}
              onInput={(e) =>
                props.onChange({ max_file_size: parseInt(e.currentTarget.value) })
              }
              class="w-32"
            />
            <span class="w-16 text-right text-sm text-gray-700 dark:text-gray-300">
              {props.config.max_file_size} MB
            </span>
          </div>
        </div>
      </div>

      {/* File Watcher Toggle */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-sm font-medium text-gray-900 dark:text-white">
              Real-time File Watching
            </h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              Automatically update index when files change
            </p>
          </div>
          <button
            onClick={() =>
              props.onChange({ enable_watcher: !props.config.enable_watcher })
            }
            class={`relative h-6 w-11 rounded-full transition-colors ${
              props.config.enable_watcher ? 'bg-blue-500' : 'bg-gray-300'
            }`}
          >
            <span
              class={`absolute top-1 h-4 w-4 rounded-full bg-white transition-transform ${
                props.config.enable_watcher ? 'left-6' : 'left-1'
              }`}
            />
          </button>
        </div>
      </div>

      {/* Reindex Button */}
      {props.onReindex && (
        <button
          onClick={props.onReindex}
          class="flex w-full items-center justify-center gap-2 rounded-lg border border-gray-300 py-3 text-gray-700 hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-800"
        >
          <RotateCcw size={18} />
          <span>Rebuild Index</span>
        </button>
      )}
    </div>
  )
}

export default Indexer
