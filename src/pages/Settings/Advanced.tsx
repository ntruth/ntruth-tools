import { Component } from 'solid-js'
import { Database, Trash2, Download, Upload, RotateCcw, Shield, Zap } from 'lucide-solid'

interface AdvancedProps {
  onExportConfig: () => void
  onImportConfig: () => void
  onResetConfig: () => void
  onClearData: () => void
}

const Advanced: Component<AdvancedProps> = (props) => {
  return (
    <div class="space-y-6">
      <div>
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Advanced Settings
        </h2>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Advanced configuration and data management options
        </p>
      </div>

      {/* Configuration Management */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <h3 class="flex items-center gap-2 text-sm font-medium text-gray-900 dark:text-white mb-4">
          <Database size={18} />
          Configuration Management
        </h3>
        
        <div class="space-y-3">
          <button
            onClick={props.onExportConfig}
            class="flex w-full items-center gap-3 rounded-lg border border-gray-200 p-3 text-left hover:bg-gray-50 dark:border-gray-700 dark:hover:bg-gray-800"
          >
            <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900">
              <Download size={20} class="text-blue-600 dark:text-blue-400" />
            </div>
            <div>
              <div class="font-medium text-gray-900 dark:text-white">
                Export Configuration
              </div>
              <div class="text-sm text-gray-500 dark:text-gray-400">
                Save your settings to a file
              </div>
            </div>
          </button>

          <button
            onClick={props.onImportConfig}
            class="flex w-full items-center gap-3 rounded-lg border border-gray-200 p-3 text-left hover:bg-gray-50 dark:border-gray-700 dark:hover:bg-gray-800"
          >
            <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-green-100 dark:bg-green-900">
              <Upload size={20} class="text-green-600 dark:text-green-400" />
            </div>
            <div>
              <div class="font-medium text-gray-900 dark:text-white">
                Import Configuration
              </div>
              <div class="text-sm text-gray-500 dark:text-gray-400">
                Load settings from a file
              </div>
            </div>
          </button>

          <button
            onClick={props.onResetConfig}
            class="flex w-full items-center gap-3 rounded-lg border border-gray-200 p-3 text-left hover:bg-gray-50 dark:border-gray-700 dark:hover:bg-gray-800"
          >
            <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-yellow-100 dark:bg-yellow-900">
              <RotateCcw size={20} class="text-yellow-600 dark:text-yellow-400" />
            </div>
            <div>
              <div class="font-medium text-gray-900 dark:text-white">
                Reset to Defaults
              </div>
              <div class="text-sm text-gray-500 dark:text-gray-400">
                Restore all settings to default values
              </div>
            </div>
          </button>
        </div>
      </div>

      {/* Data Management */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <h3 class="flex items-center gap-2 text-sm font-medium text-gray-900 dark:text-white mb-4">
          <Shield size={18} />
          Data Management
        </h3>
        
        <div class="space-y-3">
          <button
            onClick={props.onClearData}
            class="flex w-full items-center gap-3 rounded-lg border border-red-200 p-3 text-left hover:bg-red-50 dark:border-red-800 dark:hover:bg-red-900/20"
          >
            <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-red-100 dark:bg-red-900">
              <Trash2 size={20} class="text-red-600 dark:text-red-400" />
            </div>
            <div>
              <div class="font-medium text-red-700 dark:text-red-400">
                Clear All Data
              </div>
              <div class="text-sm text-red-500 dark:text-red-400">
                Delete clipboard history, search history, and cached data
              </div>
            </div>
          </button>
        </div>
      </div>

      {/* Performance */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <h3 class="flex items-center gap-2 text-sm font-medium text-gray-900 dark:text-white mb-4">
          <Zap size={18} />
          Performance
        </h3>
        
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div>
              <div class="text-sm font-medium text-gray-900 dark:text-white">
                Hardware Acceleration
              </div>
              <div class="text-sm text-gray-500 dark:text-gray-400">
                Use GPU for rendering (requires restart)
              </div>
            </div>
            <button class="relative h-6 w-11 rounded-full bg-blue-500 transition-colors">
              <span class="absolute top-1 left-6 h-4 w-4 rounded-full bg-white transition-transform" />
            </button>
          </div>

          <div class="flex items-center justify-between">
            <div>
              <div class="text-sm font-medium text-gray-900 dark:text-white">
                Reduce Motion
              </div>
              <div class="text-sm text-gray-500 dark:text-gray-400">
                Minimize animations and transitions
              </div>
            </div>
            <button class="relative h-6 w-11 rounded-full bg-gray-300 transition-colors">
              <span class="absolute top-1 left-1 h-4 w-4 rounded-full bg-white transition-transform" />
            </button>
          </div>
        </div>
      </div>

      {/* Developer Options */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <h3 class="text-sm font-medium text-gray-900 dark:text-white mb-4">
          Developer Options
        </h3>
        
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div>
              <div class="text-sm font-medium text-gray-900 dark:text-white">
                Debug Mode
              </div>
              <div class="text-sm text-gray-500 dark:text-gray-400">
                Enable verbose logging and dev tools
              </div>
            </div>
            <button class="relative h-6 w-11 rounded-full bg-gray-300 transition-colors">
              <span class="absolute top-1 left-1 h-4 w-4 rounded-full bg-white transition-transform" />
            </button>
          </div>

          <div class="flex items-center justify-between">
            <div>
              <div class="text-sm font-medium text-gray-900 dark:text-white">
                Send Analytics
              </div>
              <div class="text-sm text-gray-500 dark:text-gray-400">
                Help improve OmniBox by sending usage data
              </div>
            </div>
            <button class="relative h-6 w-11 rounded-full bg-gray-300 transition-colors">
              <span class="absolute top-1 left-1 h-4 w-4 rounded-full bg-white transition-transform" />
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}

export default Advanced
