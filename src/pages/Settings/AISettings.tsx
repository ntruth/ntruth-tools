import { Component, For, createSignal, Show } from 'solid-js'
import { invoke } from '@tauri-apps/api/core'
import { Bot, Key, Server, Thermometer, Hash, RefreshCw } from 'lucide-solid'

interface AIConfig {
  provider: string
  api_key: string
  api_url: string
  model: string
  temperature: number
  max_tokens: number
}

interface AISettingsProps {
  config: AIConfig
  onChange: (updates: Partial<AIConfig>) => void
}

const AISettings: Component<AISettingsProps> = (props) => {
  const [models, setModels] = createSignal<string[]>([])
  const [loadingModels, setLoadingModels] = createSignal(false)
  const [showApiKey, setShowApiKey] = createSignal(false)

  const providers = [
    {
      id: 'openai',
      name: 'OpenAI',
      description: 'GPT-4, GPT-3.5 Turbo',
      defaultUrl: 'https://api.openai.com/v1',
    },
    {
      id: 'anthropic',
      name: 'Anthropic',
      description: 'Claude 3.5, Claude 3',
      defaultUrl: 'https://api.anthropic.com/v1',
    },
    {
      id: 'ollama',
      name: 'Ollama',
      description: 'Local LLMs (Llama, Mistral)',
      defaultUrl: 'http://localhost:11434',
    },
  ]

  const loadModels = async () => {
    setLoadingModels(true)
    try {
      const modelList = await invoke<string[]>('ai_get_models', {
        provider: props.config.provider,
      })
      setModels(modelList)
    } catch (error) {
      console.error('Failed to load models:', error)
      // Set default models based on provider
      if (props.config.provider === 'openai') {
        setModels(['gpt-4o', 'gpt-4o-mini', 'gpt-4-turbo', 'gpt-4', 'gpt-3.5-turbo'])
      } else if (props.config.provider === 'anthropic') {
        setModels([
          'claude-3-5-sonnet-20241022',
          'claude-3-5-haiku-20241022',
          'claude-3-opus-20240229',
        ])
      } else if (props.config.provider === 'ollama') {
        setModels(['llama3.2', 'llama3.1', 'mistral', 'codellama'])
      }
    } finally {
      setLoadingModels(false)
    }
  }

  const handleProviderChange = (provider: string) => {
    const providerInfo = providers.find((p) => p.id === provider)
    props.onChange({
      provider,
      api_url: providerInfo?.defaultUrl || '',
      model: '', // Reset model when provider changes
    })
    loadModels()
  }

  return (
    <div class="space-y-6">
      <div>
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          AI Settings
        </h2>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Configure AI providers and models for chat functionality
        </p>
      </div>

      {/* Provider Selection */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <h3 class="text-sm font-medium text-gray-900 dark:text-white mb-4">
          AI Provider
        </h3>
        <div class="grid grid-cols-3 gap-3">
          <For each={providers}>
            {(provider) => (
              <button
                onClick={() => handleProviderChange(provider.id)}
                class={`rounded-lg border-2 p-4 text-left transition-colors ${
                  props.config.provider === provider.id
                    ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
                    : 'border-gray-200 hover:border-gray-300 dark:border-gray-700'
                }`}
              >
                <div class="flex items-center gap-2 mb-1">
                  <Bot size={16} class="text-gray-600 dark:text-gray-400" />
                  <span class="font-medium text-gray-900 dark:text-white">
                    {provider.name}
                  </span>
                </div>
                <p class="text-xs text-gray-500 dark:text-gray-400">
                  {provider.description}
                </p>
              </button>
            )}
          </For>
        </div>
      </div>

      {/* API Key (not for Ollama) */}
      <Show when={props.config.provider !== 'ollama'}>
        <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
          <div class="flex items-center gap-3 mb-4">
            <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-yellow-100 dark:bg-yellow-900">
              <Key size={20} class="text-yellow-600 dark:text-yellow-400" />
            </div>
            <div>
              <h3 class="text-sm font-medium text-gray-900 dark:text-white">
                API Key
              </h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">
                Your {props.config.provider === 'openai' ? 'OpenAI' : 'Anthropic'} API key
              </p>
            </div>
          </div>
          <div class="flex gap-2">
            <input
              type={showApiKey() ? 'text' : 'password'}
              value={props.config.api_key}
              onInput={(e) => props.onChange({ api_key: e.currentTarget.value })}
              placeholder="sk-..."
              class="flex-1 rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
            />
            <button
              onClick={() => setShowApiKey(!showApiKey())}
              class="rounded-lg bg-gray-100 px-4 py-2 text-sm text-gray-700 hover:bg-gray-200 dark:bg-gray-700 dark:text-gray-300"
            >
              {showApiKey() ? 'Hide' : 'Show'}
            </button>
          </div>
        </div>
      </Show>

      {/* API URL */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div class="flex items-center gap-3 mb-4">
          <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-green-100 dark:bg-green-900">
            <Server size={20} class="text-green-600 dark:text-green-400" />
          </div>
          <div>
            <h3 class="text-sm font-medium text-gray-900 dark:text-white">
              API URL
            </h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {props.config.provider === 'ollama'
                ? 'Local Ollama server URL'
                : 'Custom API endpoint (optional)'}
            </p>
          </div>
        </div>
        <input
          type="text"
          value={props.config.api_url}
          onInput={(e) => props.onChange({ api_url: e.currentTarget.value })}
          placeholder={providers.find((p) => p.id === props.config.provider)?.defaultUrl}
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
        />
      </div>

      {/* Model Selection */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div class="flex items-center justify-between mb-4">
          <div class="flex items-center gap-3">
            <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-purple-100 dark:bg-purple-900">
              <Bot size={20} class="text-purple-600 dark:text-purple-400" />
            </div>
            <div>
              <h3 class="text-sm font-medium text-gray-900 dark:text-white">
                Model
              </h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">
                Select the AI model to use
              </p>
            </div>
          </div>
          <button
            onClick={loadModels}
            disabled={loadingModels()}
            class="flex items-center gap-1 rounded-lg bg-gray-100 px-3 py-1.5 text-sm text-gray-700 hover:bg-gray-200 dark:bg-gray-700 dark:text-gray-300"
          >
            <RefreshCw size={14} class={loadingModels() ? 'animate-spin' : ''} />
            Refresh
          </button>
        </div>
        <select
          value={props.config.model}
          onChange={(e) => props.onChange({ model: e.currentTarget.value })}
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
        >
          <option value="">Select a model</option>
          <For each={models()}>
            {(model) => <option value={model}>{model}</option>}
          </For>
        </select>
      </div>

      {/* Temperature */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div class="flex items-center justify-between mb-4">
          <div class="flex items-center gap-3">
            <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-orange-100 dark:bg-orange-900">
              <Thermometer size={20} class="text-orange-600 dark:text-orange-400" />
            </div>
            <div>
              <h3 class="text-sm font-medium text-gray-900 dark:text-white">
                Temperature
              </h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">
                Controls randomness (0 = focused, 1 = creative)
              </p>
            </div>
          </div>
          <span class="text-lg font-semibold text-orange-600 dark:text-orange-400">
            {props.config.temperature.toFixed(1)}
          </span>
        </div>
        <input
          type="range"
          min="0"
          max="1"
          step="0.1"
          value={props.config.temperature}
          onInput={(e) =>
            props.onChange({ temperature: parseFloat(e.currentTarget.value) })
          }
          class="w-full"
        />
        <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400">
          <span>Precise</span>
          <span>Creative</span>
        </div>
      </div>

      {/* Max Tokens */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div class="flex items-center justify-between mb-4">
          <div class="flex items-center gap-3">
            <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900">
              <Hash size={20} class="text-blue-600 dark:text-blue-400" />
            </div>
            <div>
              <h3 class="text-sm font-medium text-gray-900 dark:text-white">
                Max Tokens
              </h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">
                Maximum response length
              </p>
            </div>
          </div>
        </div>
        <input
          type="number"
          min="100"
          max="8000"
          step="100"
          value={props.config.max_tokens}
          onInput={(e) =>
            props.onChange({ max_tokens: parseInt(e.currentTarget.value) || 2000 })
          }
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
        />
      </div>
    </div>
  )
}

export default AISettings
