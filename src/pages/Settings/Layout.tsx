import { Component, JSX } from 'solid-js'
import { Settings, Palette, Zap, Keyboard, FolderSearch, Search, ClipboardList, Camera, Bot, Wrench, Info, Puzzle, Store } from 'lucide-solid'

export type SettingsTab = 'general' | 'features' | 'appearance' | 'shortcuts' | 'indexer' | 'websearch' | 'clipboard' | 'screenshot' | 'ai' | 'plugins' | 'marketplace' | 'advanced' | 'about'

interface LayoutProps {
  activeTab: SettingsTab
  onTabChange: (tab: SettingsTab) => void
  saveStatus: 'idle' | 'saving' | 'saved'
  children: JSX.Element
}

const Layout: Component<LayoutProps> = (props) => {
  const tabs = [
    { id: 'general' as const, label: 'General', icon: Settings },
    { id: 'features' as const, label: 'Features', icon: Zap },
    { id: 'appearance' as const, label: 'Appearance', icon: Palette },
    { id: 'shortcuts' as const, label: 'Shortcuts', icon: Keyboard },
    { id: 'clipboard' as const, label: 'Clipboard', icon: ClipboardList },
    { id: 'indexer' as const, label: 'File Indexer', icon: FolderSearch },
    { id: 'websearch' as const, label: 'Web Search', icon: Search },
    { id: 'screenshot' as const, label: 'Screenshot', icon: Camera },
    { id: 'ai' as const, label: 'AI', icon: Bot },
    { id: 'plugins' as const, label: 'Plugins', icon: Puzzle },
    { id: 'marketplace' as const, label: 'Marketplace', icon: Store },
    { id: 'advanced' as const, label: 'Advanced', icon: Wrench },
    { id: 'about' as const, label: 'About', icon: Info },
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
