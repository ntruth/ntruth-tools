import { Component } from 'solid-js'

interface GeneralConfig {
  language: string
  auto_start: boolean
  check_updates: boolean
}

interface GeneralProps {
  config: GeneralConfig
  onChange: (updates: Partial<GeneralConfig>) => void
}

const General: Component<GeneralProps> = (props) => {
  return (
    <div class="space-y-6">
      <div>
        <h2 class="mb-4 text-xl font-semibold text-gray-900 dark:text-white">General Settings</h2>
      </div>

      {/* Startup Settings */}
      <section class="space-y-4">
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300">Startup</h3>
        
        <label class="flex items-center justify-between">
          <div>
            <div class="text-sm font-medium text-gray-900 dark:text-white">
              Launch at startup
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-400">
              Automatically start OmniBox when you log in
            </div>
          </div>
          <input
            type="checkbox"
            checked={props.config.auto_start}
            onChange={(e) => props.onChange({ auto_start: e.currentTarget.checked })}
            class="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-2 focus:ring-blue-500"
          />
        </label>
      </section>

      {/* Language Settings */}
      <section class="space-y-4">
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300">Language</h3>
        
        <label class="block">
          <div class="mb-2 text-sm font-medium text-gray-900 dark:text-white">
            Interface Language
          </div>
          <select
            value={props.config.language}
            onChange={(e) => props.onChange({ language: e.currentTarget.value })}
            class="w-full rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
          >
            <option value="system">System Default</option>
            <option value="en">English</option>
            <option value="zh">中文 (Chinese)</option>
            <option value="ja">日本語 (Japanese)</option>
            <option value="ko">한국어 (Korean)</option>
          </select>
        </label>
      </section>

      {/* Update Settings */}
      <section class="space-y-4">
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300">Updates</h3>
        
        <label class="flex items-center justify-between">
          <div>
            <div class="text-sm font-medium text-gray-900 dark:text-white">
              Check for updates automatically
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-400">
              Get notified when a new version is available
            </div>
          </div>
          <input
            type="checkbox"
            checked={props.config.check_updates}
            onChange={(e) => props.onChange({ check_updates: e.currentTarget.checked })}
            class="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-2 focus:ring-blue-500"
          />
        </label>
      </section>
    </div>
  )
}

export default General
