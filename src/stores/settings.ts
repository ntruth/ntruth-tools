import { createStore } from 'solid-js/store'
import type { AppConfig } from '@/types'

const defaultConfig: AppConfig = {
  general: {
    language: 'en',
    autoStart: false,
    checkUpdates: true,
  },
  features: {
    fileSearch: true,
    appSearch: true,
    calculator: true,
    webSearch: true,
    clipboard: true,
    screenshot: true,
    ai: false,
  },
  appearance: {
    theme: 'auto',
    accentColor: '#007AFF',
    transparency: 0.95,
    windowRadius: 8,
  },
  shortcuts: {
    main: 'CommandOrControl+Space',
    clipboard: 'CommandOrControl+Shift+V',
    screenshot: 'CommandOrControl+Shift+S',
    aiChat: 'CommandOrControl+Shift+A',
  },
  indexer: {
    enabled: true,
    indexPaths: [],
    excludePaths: [],
    fileTypes: [],
    maxFileSize: 100 * 1024 * 1024, // 100MB
    indexHidden: false,
  },
  clipboard: {
    enabled: true,
    historyLimit: 1000,
    filterSensitive: true,
    excludeApps: [],
  },
  screenshot: {
    format: 'png',
    quality: 90,
    saveDir: '',
    autoSave: false,
  },
  ai: {
    provider: 'openai',
    apiKey: '',
    apiUrl: '',
    model: 'gpt-4',
    temperature: 0.7,
    maxTokens: 2000,
  },
  webSearch: {
    defaultEngine: 'google',
    engines: [
      { name: 'Google', keyword: 'gg', url: 'https://www.google.com/search?q={query}' },
      { name: 'Baidu', keyword: 'bd', url: 'https://www.baidu.com/s?wd={query}' },
      { name: 'GitHub', keyword: 'gh', url: 'https://github.com/search?q={query}' },
    ],
  },
}

const [settingsStore, setSettingsStore] = createStore<AppConfig>(defaultConfig)

export const useSettingsStore = () => {
  const updateConfig = (config: Partial<AppConfig>) => {
    setSettingsStore(config as AppConfig)
  }

  return {
    store: settingsStore,
    updateConfig,
  }
}

export { settingsStore }
