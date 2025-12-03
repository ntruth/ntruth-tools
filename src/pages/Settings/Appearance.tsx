import { Component } from 'solid-js'
import { Sun, Moon, Monitor } from 'lucide-solid'

interface AppearanceConfig {
  theme: string
  accent_color: string
  transparency: number
  window_radius: number
}

interface AppearanceProps {
  config: AppearanceConfig
  onChange: (updates: Partial<AppearanceConfig>) => void
}

const Appearance: Component<AppearanceProps> = (props) => {
  const themes = [
    { value: 'light', label: 'Light', icon: Sun },
    { value: 'dark', label: 'Dark', icon: Moon },
    { value: 'auto', label: 'System', icon: Monitor },
  ]

  const accentColors = [
    { name: 'Blue', value: '#007AFF' },
    { name: 'Purple', value: '#AF52DE' },
    { name: 'Pink', value: '#FF2D55' },
    { name: 'Red', value: '#FF3B30' },
    { name: 'Orange', value: '#FF9500' },
    { name: 'Yellow', value: '#FFCC00' },
    { name: 'Green', value: '#34C759' },
    { name: 'Teal', value: '#5AC8FA' },
  ]

  return (
    <div class="space-y-6">
      <div>
        <h2 class="mb-4 text-xl font-semibold text-gray-900 dark:text-white">Appearance</h2>
        <p class="text-sm text-gray-600 dark:text-gray-400">
          Customize the look and feel of OmniBox
        </p>
      </div>

      {/* Theme Selection */}
      <section class="space-y-4">
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300">Theme</h3>
        <div class="grid grid-cols-3 gap-4">
          {themes.map((theme) => {
            const Icon = theme.icon
            return (
              <button
                onClick={() => props.onChange({ theme: theme.value })}
                class={`flex flex-col items-center gap-2 rounded-lg border-2 p-4 transition-all ${
                  props.config.theme === theme.value
                    ? 'border-blue-500 bg-blue-50 dark:border-blue-600 dark:bg-blue-900/20'
                    : 'border-gray-200 hover:border-gray-300 dark:border-gray-700 dark:hover:border-gray-600'
                }`}
              >
                <Icon size={24} class="text-gray-600 dark:text-gray-400" />
                <span class="text-sm font-medium text-gray-900 dark:text-white">
                  {theme.label}
                </span>
              </button>
            )
          })}
        </div>
      </section>

      {/* Accent Color */}
      <section class="space-y-4">
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300">Accent Color</h3>
        <div class="grid grid-cols-8 gap-3">
          {accentColors.map((color) => (
            <button
              onClick={() => props.onChange({ accent_color: color.value })}
              class={`h-10 w-10 rounded-full transition-transform hover:scale-110 ${
                props.config.accent_color === color.value
                  ? 'ring-2 ring-gray-900 ring-offset-2 dark:ring-white'
                  : ''
              }`}
              style={{ 'background-color': color.value }}
              title={color.name}
            />
          ))}
        </div>
      </section>

      {/* Window Settings */}
      <section class="space-y-4">
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300">Window</h3>
        
        <div class="space-y-4">
          <label class="block">
            <div class="mb-2 flex items-center justify-between">
              <span class="text-sm font-medium text-gray-900 dark:text-white">
                Transparency
              </span>
              <span class="text-sm text-gray-500 dark:text-gray-400">
                {Math.round(props.config.transparency * 100)}%
              </span>
            </div>
            <input
              type="range"
              min="0.5"
              max="1"
              step="0.05"
              value={props.config.transparency}
              onInput={(e) =>
                props.onChange({ transparency: parseFloat(e.currentTarget.value) })
              }
              class="w-full"
            />
          </label>

          <label class="block">
            <div class="mb-2 flex items-center justify-between">
              <span class="text-sm font-medium text-gray-900 dark:text-white">
                Corner Radius
              </span>
              <span class="text-sm text-gray-500 dark:text-gray-400">
                {props.config.window_radius}px
              </span>
            </div>
            <input
              type="range"
              min="0"
              max="24"
              step="2"
              value={props.config.window_radius}
              onInput={(e) =>
                props.onChange({ window_radius: parseInt(e.currentTarget.value) })
              }
              class="w-full"
            />
          </label>
        </div>
      </section>
    </div>
  )
}

export default Appearance
