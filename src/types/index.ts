// Core types
export interface AppConfig {
  general: GeneralConfig
  features: FeaturesConfig
  appearance: AppearanceConfig
  shortcuts: ShortcutsConfig
  indexer: IndexerConfig
  clipboard: ClipboardConfig
  screenshot: ScreenshotConfig
  ai: AIConfig
  webSearch: WebSearchConfig
}

export interface GeneralConfig {
  language: 'en' | 'zh-CN'
  autoStart: boolean
  checkUpdates: boolean
}

export interface FeaturesConfig {
  fileSearch: boolean
  appSearch: boolean
  calculator: boolean
  webSearch: boolean
  clipboard: boolean
  screenshot: boolean
  ai: boolean
}

export interface AppearanceConfig {
  theme: 'light' | 'dark' | 'auto'
  accentColor: string
  transparency: number
  windowRadius: number
}

export interface ShortcutsConfig {
  main: string
  clipboard: string
  screenshot: string
  aiChat: string
}

export interface IndexerConfig {
  enabled: boolean
  indexPaths: string[]
  excludePaths: string[]
  fileTypes: string[]
  maxFileSize: number
  indexHidden: boolean
}

export interface ClipboardConfig {
  enabled: boolean
  historyLimit: number
  filterSensitive: boolean
  excludeApps: string[]
}

export interface ScreenshotConfig {
  format: 'png' | 'jpg'
  quality: number
  saveDir: string
  autoSave: boolean
}

export interface AIConfig {
  provider: 'openai' | 'anthropic' | 'ollama' | 'custom'
  apiKey: string
  apiUrl: string
  model: string
  temperature: number
  maxTokens: number
}

export interface WebSearchConfig {
  defaultEngine: string
  engines: SearchEngine[]
}

export interface SearchEngine {
  name: string
  keyword: string
  url: string
  icon?: string
}

// Window types
export type WindowLabel = 'main' | 'clipboard' | 'settings' | 'ai-chat' | `pin-${string}` | 'screenshot-overlay' | 'screenshot-editor'
