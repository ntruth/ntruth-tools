import { Component, For } from 'solid-js'
import { X, Sparkles, MessageSquare } from 'lucide-solid'

interface PresetPrompt {
  id: string
  name: string
  prompt: string
  description?: string
  category?: string
}

interface PresetSelectorProps {
  presets: PresetPrompt[]
  onSelect: (prompt: string) => void
  onClose: () => void
  onNewChat: () => void
}

const PresetSelector: Component<PresetSelectorProps> = (props) => {
  // Group presets by category
  const groupedPresets = () => {
    const groups: Record<string, PresetPrompt[]> = {}
    props.presets.forEach((preset) => {
      const category = preset.category || 'General'
      if (!groups[category]) {
        groups[category] = []
      }
      groups[category].push(preset)
    })
    return groups
  }

  return (
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div class="w-full max-w-lg rounded-xl bg-white p-6 shadow-xl dark:bg-gray-800">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
            Start New Chat
          </h2>
          <button
            onClick={props.onClose}
            class="p-1 text-gray-500 hover:text-gray-700 dark:hover:text-gray-300"
          >
            <X size={20} />
          </button>
        </div>

        {/* New Empty Chat */}
        <button
          onClick={props.onNewChat}
          class="flex w-full items-center gap-3 rounded-lg border-2 border-dashed border-gray-300 p-4 mb-4 hover:border-blue-500 hover:bg-blue-50 dark:border-gray-600 dark:hover:border-blue-400 dark:hover:bg-blue-900/20"
        >
          <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900">
            <MessageSquare size={20} class="text-blue-600 dark:text-blue-400" />
          </div>
          <div class="text-left">
            <div class="font-medium text-gray-900 dark:text-white">
              New Empty Chat
            </div>
            <div class="text-sm text-gray-500 dark:text-gray-400">
              Start a conversation without a preset
            </div>
          </div>
        </button>

        {/* Preset Prompts */}
        <div class="space-y-4 max-h-96 overflow-y-auto">
          <For each={Object.entries(groupedPresets())}>
            {([category, presets]) => (
              <div>
                <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-2">
                  {category}
                </h3>
                <div class="space-y-2">
                  <For each={presets}>
                    {(preset) => (
                      <button
                        onClick={() => props.onSelect(preset.prompt)}
                        class="flex w-full items-center gap-3 rounded-lg border border-gray-200 p-3 text-left hover:bg-gray-50 dark:border-gray-700 dark:hover:bg-gray-700"
                      >
                        <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-purple-100 dark:bg-purple-900">
                          <Sparkles size={16} class="text-purple-600 dark:text-purple-400" />
                        </div>
                        <div class="flex-1">
                          <div class="font-medium text-gray-900 dark:text-white">
                            {preset.name}
                          </div>
                          {preset.description && (
                            <div class="text-sm text-gray-500 dark:text-gray-400">
                              {preset.description}
                            </div>
                          )}
                        </div>
                      </button>
                    )}
                  </For>
                </div>
              </div>
            )}
          </For>
        </div>
      </div>
    </div>
  )
}

export default PresetSelector
