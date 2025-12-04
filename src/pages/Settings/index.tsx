import { Component, createSignal, onMount } from 'solid-js'
import { invoke } from '@tauri-apps/api/core'
import { save, open } from '@tauri-apps/plugin-dialog'
import Layout, { SettingsTab } from './Layout'
import General from './General'
import Features from './Features'
import Appearance from './Appearance'
import Shortcuts from './Shortcuts'
import Clipboard from './Clipboard'
import Indexer from './Indexer'
import WebSearch from './WebSearch'
import Screenshot from './Screenshot'
import AISettings from './AISettings'
import { Plugins } from './Plugins'
import { PluginMarket } from './PluginMarket'
import Advanced from './Advanced'
import About from './About'

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
  const [activeTab, setActiveTab] = createSignal<SettingsTab>('general')
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

  const handleConfigChange = <K extends keyof AppConfig>(
    section: K,
    updates: Partial<AppConfig[K]>
  ) => {
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

  const handleExportConfig = async () => {
    try {
      const filePath = await save({
        filters: [{ name: 'JSON', extensions: ['json'] }],
        defaultPath: 'omnibox-config.json',
      })
      if (filePath) {
        const configJson = await invoke<string>('export_config')
        // Write to file using Tauri fs plugin
        await invoke('write_file', { path: filePath, content: configJson })
      }
    } catch (error) {
      console.error('Failed to export config:', error)
    }
  }

  const handleImportConfig = async () => {
    try {
      const filePath = await open({
        filters: [{ name: 'JSON', extensions: ['json'] }],
        multiple: false,
      })
      if (filePath) {
        await invoke('import_config', { path: filePath })
        await loadConfig()
      }
    } catch (error) {
      console.error('Failed to import config:', error)
    }
  }

  const handleResetConfig = async () => {
    if (confirm('Are you sure you want to reset all settings to defaults?')) {
      try {
        await invoke('reset_config')
        await loadConfig()
      } catch (error) {
        console.error('Failed to reset config:', error)
      }
    }
  }

  const handleClearData = async () => {
    if (confirm('Are you sure you want to clear all data? This cannot be undone.')) {
      try {
        await invoke('ai_clear_conversations')
        // Additional clear operations can be added here
        alert('Data cleared successfully')
      } catch (error) {
        console.error('Failed to clear data:', error)
      }
    }
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
      {activeTab() === 'shortcuts' && config() && (
        <Shortcuts
          config={config()!.shortcuts}
          onChange={(updates) => handleConfigChange('shortcuts', updates)}
        />
      )}
      {activeTab() === 'clipboard' && config() && (
        <Clipboard
          config={{
            enabled: config()!.clipboard.enabled,
            history_limit: config()!.clipboard.history_limit,
            filter_sensitive: config()!.clipboard.filter_sensitive,
            exclude_apps: config()!.clipboard.exclude_apps,
          }}
          onChange={(updates) => handleConfigChange('clipboard', updates)}
        />
      )}
      {activeTab() === 'indexer' && config() && (
        <Indexer
          config={{
            indexed_directories: config()!.indexer.index_paths,
            exclude_patterns: config()!.indexer.exclude_paths,
            include_extensions: config()!.indexer.file_types,
            max_file_size: config()!.indexer.max_file_size,
            enable_watcher: config()!.indexer.enabled,
          }}
          onChange={(updates) => {
            const mappedUpdates: Partial<AppConfig['indexer']> = {}
            if (updates.indexed_directories !== undefined) mappedUpdates.index_paths = updates.indexed_directories
            if (updates.exclude_patterns !== undefined) mappedUpdates.exclude_paths = updates.exclude_patterns
            if (updates.include_extensions !== undefined) mappedUpdates.file_types = updates.include_extensions
            if (updates.max_file_size !== undefined) mappedUpdates.max_file_size = updates.max_file_size
            if (updates.enable_watcher !== undefined) mappedUpdates.enabled = updates.enable_watcher
            handleConfigChange('indexer', mappedUpdates)
          }}
          onReindex={() => invoke('rebuild_index')}
        />
      )}
      {activeTab() === 'websearch' && config() && (
        <WebSearch
          config={{
            default_engine: config()!.web_search.default_engine,
            engines: config()!.web_search.engines.map(e => ({ ...e, enabled: true })),
          }}
          onChange={(updates) => {
            const mappedUpdates: Partial<AppConfig['web_search']> = {}
            if (updates.default_engine !== undefined) mappedUpdates.default_engine = updates.default_engine
            if (updates.engines !== undefined) mappedUpdates.engines = updates.engines.map(({ name, keyword, url }) => ({ name, keyword, url }))
            handleConfigChange('web_search', mappedUpdates)
          }}
        />
      )}
      {activeTab() === 'screenshot' && config() && (
        <Screenshot
          config={config()!.screenshot}
          onChange={(updates) => handleConfigChange('screenshot', updates)}
        />
      )}
      {activeTab() === 'ai' && config() && (
        <AISettings
          config={config()!.ai}
          onChange={(updates) => handleConfigChange('ai', updates)}
        />
      )}
      {activeTab() === 'plugins' && <Plugins />}
      {activeTab() === 'marketplace' && <PluginMarket />}
      {activeTab() === 'advanced' && (
        <Advanced
          onExportConfig={handleExportConfig}
          onImportConfig={handleImportConfig}
          onResetConfig={handleResetConfig}
          onClearData={handleClearData}
        />
      )}
      {activeTab() === 'about' && <About />}
    </Layout>
  )
}

export default SettingsPage
