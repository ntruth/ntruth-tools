import { Component, createSignal, onMount } from 'solid-js'
import { invoke } from '@tauri-apps/api/core'
import Layout from './Layout'
import General from './General'
import Features from './Features'
import Appearance from './Appearance'

export interface AppConfig {
  general: {
    language: string
    auto_start: boolean
    check_updates: boolean
  }
  features: {
    file_search: boolean
    app_search: boolean
    calculator: boolean
    web_search: boolean
    clipboard: boolean
    screenshot: boolean
    ai: boolean
  }
  appearance: {
    theme: string
    accent_color: string
    transparency: number
    window_radius: number
  }
  shortcuts: {
    main: string
    clipboard: string
    screenshot: string
    ai_chat: string
  }
  indexer: {
    enabled: boolean
    index_paths: string[]
    exclude_paths: string[]
    file_types: string[]
    max_file_size: number
    index_hidden: boolean
  }
  clipboard: {
    enabled: boolean
    history_limit: number
    filter_sensitive: boolean
    exclude_apps: string[]
  }
  screenshot: {
    format: string
    quality: number
    save_dir: string
    auto_save: boolean
  }
  ai: {
    provider: string
    api_key: string
    api_url: string
    model: string
    temperature: number
    max_tokens: number
  }
  web_search: {
    default_engine: string
    engines: Array<{
      name: string
      keyword: string
      url: string
      icon?: string
    }>
  }
}

const SettingsPage: Component = () => {
  const [config, setConfig] = createSignal<AppConfig | null>(null)
  const [activeTab, setActiveTab] = createSignal<'general' | 'features' | 'appearance'>('general')
  const [saveStatus, setSaveStatus] = createSignal<'idle' | 'saving' | 'saved'>('idle')

  onMount(async () => {
    await loadConfig()
  })

  const loadConfig = async () => {
    try {
      const cfg = await invoke<AppConfig>('get_config')
      setConfig(cfg)
    } catch (error) {
      console.error('Failed to load config:', error)
    }
  }

  const saveConfig = async (newConfig: AppConfig) => {
    try {
      setSaveStatus('saving')
      await invoke('update_config', { config: newConfig })
      setConfig(newConfig)
      setSaveStatus('saved')
      setTimeout(() => setSaveStatus('idle'), 2000)
    } catch (error) {
      console.error('Failed to save config:', error)
      setSaveStatus('idle')
    }
  }

  const handleConfigChange = (section: keyof AppConfig, updates: Partial<any>) => {
    const current = config()
    if (!current) return

    const updated = {
      ...current,
      [section]: {
        ...current[section],
        ...updates,
      },
    }
    saveConfig(updated)
  }

  return (
    <Layout
      activeTab={activeTab()}
      onTabChange={setActiveTab}
      saveStatus={saveStatus()}
    >
      {activeTab() === 'general' && config() && (
        <General
          config={config()!.general}
          onChange={(updates) => handleConfigChange('general', updates)}
        />
      )}
      {activeTab() === 'features' && config() && (
        <Features
          config={config()!.features}
          onChange={(updates) => handleConfigChange('features', updates)}
        />
      )}
      {activeTab() === 'appearance' && config() && (
        <Appearance
          config={config()!.appearance}
          onChange={(updates) => handleConfigChange('appearance', updates)}
        />
      )}
    </Layout>
  )
}

export default SettingsPage
