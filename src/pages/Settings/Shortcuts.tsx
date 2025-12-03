import { Component } from 'solid-js'
import { Keyboard } from 'lucide-solid'

interface ShortcutsConfig {
  main: string
  clipboard: string
  screenshot: string
  ai_chat: string
}

interface ShortcutsProps {
  config: ShortcutsConfig
  onChange: (updates: Partial<ShortcutsConfig>) => void
}

const Shortcuts: Component<ShortcutsProps> = (props) => {
  const isMac = navigator.platform.toLowerCase().includes('mac')
  const modKey = isMac ? '⌘' : 'Ctrl'

  const shortcuts = [
    {
      id: 'main' as const,
      label: 'Main Window',
      description: 'Open/close the main search window',
      value: props.config.main,
    },
    {
      id: 'clipboard' as const,
      label: 'Clipboard History',
      description: 'Open/close clipboard history',
      value: props.config.clipboard,
    },
    {
      id: 'screenshot' as const,
      label: 'Screenshot',
      description: 'Take a screenshot',
      value: props.config.screenshot,
    },
    {
      id: 'ai_chat' as const,
      label: 'AI Chat',
      description: 'Open AI chat window',
      value: props.config.ai_chat,
    },
  ]

  const formatShortcut = (shortcut: string): string => {
    return shortcut
      .replace('CommandOrControl', modKey)
      .replace('Shift', '⇧')
      .replace('Alt', isMac ? '⌥' : 'Alt')
      .replace('+', ' ')
  }

  return (
    <div class="space-y-6">
      <div>
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Keyboard Shortcuts
        </h2>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Configure global keyboard shortcuts for quick access
        </p>
      </div>

      <div class="space-y-3">
        {shortcuts.map((shortcut) => (
          <div class="flex items-center justify-between rounded-lg border border-gray-200 p-4 dark:border-gray-700">
            <div class="flex items-center gap-3">
              <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-gray-100 dark:bg-gray-800">
                <Keyboard size={20} class="text-gray-600 dark:text-gray-400" />
              </div>
              <div>
                <h3 class="text-sm font-medium text-gray-900 dark:text-white">
                  {shortcut.label}
                </h3>
                <p class="text-sm text-gray-500 dark:text-gray-400">
                  {shortcut.description}
                </p>
              </div>
            </div>
            <div class="flex items-center gap-2">
              <div class="flex items-center gap-1 rounded-lg bg-gray-100 px-3 py-2 font-mono text-sm dark:bg-gray-800">
                {formatShortcut(shortcut.value).split(' ').map((key) => (
                  <kbd class="rounded bg-white px-2 py-1 shadow-sm dark:bg-gray-700">
                    {key}
                  </kbd>
                ))}
              </div>
            </div>
          </div>
        ))}
      </div>

      <div class="rounded-lg bg-blue-50 p-4 dark:bg-blue-900/20">
        <h3 class="text-sm font-medium text-blue-800 dark:text-blue-200">
          Note
        </h3>
        <p class="mt-1 text-sm text-blue-700 dark:text-blue-300">
          Shortcut editing will be available in a future update. Currently using default shortcuts.
        </p>
      </div>
    </div>
  )
}

export default Shortcuts
