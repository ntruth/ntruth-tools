import { Component } from 'solid-js'
import { Search, Calculator, Globe, Clipboard, Camera, Sparkles, Layout } from 'lucide-solid'

interface FeaturesConfig {
  file_search: boolean
  app_search: boolean
  calculator: boolean
  web_search: boolean
  clipboard: boolean
  screenshot: boolean
  ai: boolean
}

interface FeaturesProps {
  config: FeaturesConfig
  onChange: (updates: Partial<FeaturesConfig>) => void
}

const Features: Component<FeaturesProps> = (props) => {
  const features = [
    {
      key: 'file_search' as const,
      icon: Search,
      title: 'File Search',
      description: 'Search files and folders on your computer',
    },
    {
      key: 'app_search' as const,
      icon: Layout,
      title: 'Application Search',
      description: 'Find and launch installed applications',
    },
    {
      key: 'calculator' as const,
      icon: Calculator,
      title: 'Calculator',
      description: 'Evaluate mathematical expressions inline',
    },
    {
      key: 'web_search' as const,
      icon: Globe,
      title: 'Web Search',
      description: 'Search the web with custom keywords',
    },
    {
      key: 'clipboard' as const,
      icon: Clipboard,
      title: 'Clipboard Manager',
      description: 'Track and manage clipboard history',
    },
    {
      key: 'screenshot' as const,
      icon: Camera,
      title: 'Screenshot',
      description: 'Capture and annotate screenshots',
    },
    {
      key: 'ai' as const,
      icon: Sparkles,
      title: 'AI Assistant',
      description: 'Chat with AI and get intelligent answers',
    },
  ]

  return (
    <div class="space-y-6">
      <div>
        <h2 class="mb-4 text-xl font-semibold text-gray-900 dark:text-white">Features</h2>
        <p class="text-sm text-gray-600 dark:text-gray-400">
          Enable or disable specific features to customize your experience
        </p>
      </div>

      <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
        {features.map((feature) => {
          const Icon = feature.icon
          return (
            <div class="rounded-lg border border-gray-200 p-4 transition-all hover:border-blue-300 hover:shadow-sm dark:border-gray-700 dark:hover:border-blue-600">
              <label class="flex cursor-pointer items-start gap-3">
                <input
                  type="checkbox"
                  checked={props.config[feature.key]}
                  onChange={(e) =>
                    props.onChange({ [feature.key]: e.currentTarget.checked })
                  }
                  class="mt-1 h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-2 focus:ring-blue-500"
                />
                <div class="flex-1">
                  <div class="flex items-center gap-2">
                    <Icon size={16} class="text-gray-600 dark:text-gray-400" />
                    <div class="text-sm font-medium text-gray-900 dark:text-white">
                      {feature.title}
                    </div>
                  </div>
                  <div class="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    {feature.description}
                  </div>
                </div>
              </label>
            </div>
          )
        })}
      </div>
    </div>
  )
}

export default Features
