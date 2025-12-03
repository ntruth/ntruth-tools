import { Component, JSX } from 'solid-js'
import { Settings, Palette, Zap } from 'lucide-solid'

interface LayoutProps {
  activeTab: 'general' | 'features' | 'appearance'
  onTabChange: (tab: 'general' | 'features' | 'appearance') => void
  saveStatus: 'idle' | 'saving' | 'saved'
  children: JSX.Element
}

const Layout: Component<LayoutProps> = (props) => {
  const tabs = [
    { id: 'general' as const, label: 'General', icon: Settings },
    { id: 'features' as const, label: 'Features', icon: Zap },
    { id: 'appearance' as const, label: 'Appearance', icon: Palette },
  ]

  return (
    <div class="flex h-full w-full bg-white dark:bg-gray-900">
      {/* Sidebar */}
      <div class="w-48 border-r border-gray-200 bg-gray-50 dark:border-gray-700 dark:bg-gray-800">
        <div class="p-4">
          <h1 class="text-lg font-semibold text-gray-900 dark:text-white">Settings</h1>
        </div>
        <nav class="space-y-1 px-2">
          {tabs.map((tab) => {
            const Icon = tab.icon
            return (
              <button
                onClick={() => props.onTabChange(tab.id)}
                class={`flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors ${
                  props.activeTab === tab.id
                    ? 'bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300'
                    : 'text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700'
                }`}
              >
                <Icon size={16} />
                <span>{tab.label}</span>
              </button>
            )
          })}
        </nav>

        {/* Save status indicator */}
        {props.saveStatus !== 'idle' && (
          <div class="absolute bottom-4 left-4 text-xs text-gray-600 dark:text-gray-400">
            {props.saveStatus === 'saving' ? '💾 Saving...' : '✓ Saved'}
          </div>
        )}
      </div>

      {/* Content */}
      <div class="flex-1 overflow-y-auto">
        <div class="p-6">{props.children}</div>
      </div>
    </div>
  )
}

export default Layout
