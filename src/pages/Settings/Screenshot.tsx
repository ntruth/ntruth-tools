import { Component } from 'solid-js'
import { Camera, FolderOpen, Image } from 'lucide-solid'

interface ScreenshotConfig {
  format: string
  quality: number
  save_dir: string
  auto_save: boolean
}

interface ScreenshotProps {
  config: ScreenshotConfig
  onChange: (updates: Partial<ScreenshotConfig>) => void
  onSelectDirectory?: () => void
}

const Screenshot: Component<ScreenshotProps> = (props) => {
  const formats = [
    { value: 'png', label: 'PNG (Lossless)' },
    { value: 'jpg', label: 'JPEG (Compressed)' },
    { value: 'webp', label: 'WebP (Modern)' },
  ]

  return (
    <div class="space-y-6">
      <div>
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Screenshot Settings
        </h2>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Configure screenshot capture and save options
        </p>
      </div>

      {/* Image Format */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div class="flex items-center gap-3 mb-4">
          <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-purple-100 dark:bg-purple-900">
            <Image size={20} class="text-purple-600 dark:text-purple-400" />
          </div>
          <div>
            <h3 class="text-sm font-medium text-gray-900 dark:text-white">
              Image Format
            </h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              Choose the format for saved screenshots
            </p>
          </div>
        </div>

        <div class="grid grid-cols-3 gap-3">
          {formats.map((format) => (
            <button
              onClick={() => props.onChange({ format: format.value })}
              class={`rounded-lg border-2 p-3 text-center transition-colors ${
                props.config.format === format.value
                  ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
                  : 'border-gray-200 hover:border-gray-300 dark:border-gray-700'
              }`}
            >
              <div class="font-medium text-gray-900 dark:text-white">
                {format.value.toUpperCase()}
              </div>
              <div class="text-xs text-gray-500 dark:text-gray-400">
                {format.label.split('(')[1]?.replace(')', '')}
              </div>
            </button>
          ))}
        </div>
      </div>

      {/* Quality (for JPEG/WebP) */}
      {(props.config.format === 'jpg' || props.config.format === 'webp') && (
        <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
          <div class="flex items-center justify-between mb-4">
            <div>
              <h3 class="text-sm font-medium text-gray-900 dark:text-white">
                Image Quality
              </h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">
                Higher quality means larger file size
              </p>
            </div>
            <span class="text-lg font-semibold text-blue-600 dark:text-blue-400">
              {props.config.quality}%
            </span>
          </div>

          <input
            type="range"
            min="10"
            max="100"
            step="5"
            value={props.config.quality}
            onInput={(e) =>
              props.onChange({ quality: parseInt(e.currentTarget.value) })
            }
            class="w-full"
          />
          <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400">
            <span>Smallest</span>
            <span>Best Quality</span>
          </div>
        </div>
      )}

      {/* Save Directory */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div class="flex items-center gap-3 mb-4">
          <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-green-100 dark:bg-green-900">
            <FolderOpen size={20} class="text-green-600 dark:text-green-400" />
          </div>
          <div class="flex-1">
            <h3 class="text-sm font-medium text-gray-900 dark:text-white">
              Save Directory
            </h3>
            <p class="text-sm text-gray-500 dark:text-gray-400 truncate">
              {props.config.save_dir || 'Not set (will use default)'}
            </p>
          </div>
          <button
            onClick={props.onSelectDirectory}
            class="rounded-lg bg-gray-100 px-4 py-2 text-sm text-gray-700 hover:bg-gray-200 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600"
          >
            Browse
          </button>
        </div>
      </div>

      {/* Auto Save */}
      <div class="flex items-center justify-between rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div class="flex items-center gap-3">
          <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900">
            <Camera size={20} class="text-blue-600 dark:text-blue-400" />
          </div>
          <div>
            <h3 class="text-sm font-medium text-gray-900 dark:text-white">
              Auto Save
            </h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              Automatically save screenshots to the directory
            </p>
          </div>
        </div>
        <button
          onClick={() => props.onChange({ auto_save: !props.config.auto_save })}
          class={`relative h-6 w-11 rounded-full transition-colors ${
            props.config.auto_save ? 'bg-blue-500' : 'bg-gray-300'
          }`}
        >
          <span
            class={`absolute top-1 h-4 w-4 rounded-full bg-white transition-transform ${
              props.config.auto_save ? 'left-6' : 'left-1'
            }`}
          />
        </button>
      </div>

      {/* Coming Soon Notice */}
      <div class="rounded-lg bg-yellow-50 p-4 dark:bg-yellow-900/20">
        <h3 class="text-sm font-medium text-yellow-800 dark:text-yellow-200">
          Coming Soon
        </h3>
        <p class="mt-1 text-sm text-yellow-700 dark:text-yellow-300">
          Screenshot functionality is under development. Stay tuned for updates!
        </p>
      </div>
    </div>
  )
}

export default Screenshot
