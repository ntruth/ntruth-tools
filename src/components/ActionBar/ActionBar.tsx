import { Component } from 'solid-js'

interface ActionBarProps {
  /** Show available actions and shortcuts */
  visible?: boolean
}

/**
 * Action bar component showing available actions and keyboard shortcuts
 */
export const ActionBar: Component<ActionBarProps> = (props) => {
  const actions = [
    { key: '↵', label: 'Open' },
    { key: '⌘C / Ctrl+C', label: 'Copy' },
    { key: 'Esc', label: 'Close' },
  ]

  if (!props.visible) return null

  return (
    <div class="mt-2 flex items-center justify-center gap-4 px-4 py-2">
      {actions.map((action) => (
        <div class="flex items-center gap-1.5 text-xs text-gray-500 dark:text-gray-400">
          <kbd class="rounded bg-gray-200 px-1.5 py-0.5 font-mono font-semibold dark:bg-gray-700">
            {action.key}
          </kbd>
          <span>{action.label}</span>
        </div>
      ))}
    </div>
  )
}
