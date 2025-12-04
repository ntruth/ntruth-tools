/**
 * Alfred-style Settings Page
 * 
 * Main entry point for the settings window using Alfred 5-inspired layout
 */

import { Component, createSignal, onMount, For, Show, createMemo, onCleanup } from 'solid-js'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import '../../styles/alfred-settings.css'
import { 
  Settings, 
  Zap, 
  Workflow, 
  Bot,
  Search,
  FileSearch,
  MousePointer,
  Globe,
  Bookmark,
  ClipboardList,
  FileText,
  Calculator,
  BookOpen,
  User,
  Terminal,
  Type,
  Eye,
  Wrench,
  Info,
  HelpCircle,
  Plus,
  Minus,
  FolderOpen,
  Store,
  Puzzle
} from 'lucide-solid'

// =============================================================================
// Types
// =============================================================================

type PrimaryTab = 'marketplace' | 'plugins' | 'general' | 'features' | 'workflows' | 'ai'

type FeatureSubTab = 
  | 'default-results' 
  | 'file-search' 
  | 'universal-actions' 
  | 'web-search' 
  | 'web-bookmarks'
  | 'clipboard-history'
  | 'snippets'
  | 'calculator'
  | 'dictionary'
  | 'contacts'
  | 'system'
  | 'terminal'
  | 'large-type'
  | 'previews'

interface AppConfig {
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
  indexer: {
    enabled: boolean
    index_paths: string[]
    exclude_paths: string[]
    file_types: string[]
    max_file_size: number
  }
  clipboard: {
    enabled: boolean
    history_limit: number
    filter_sensitive: boolean
    exclude_apps: string[]
  }
  ai: {
    provider: string
    api_key: string
    model: string
    temperature: number
    max_tokens: number
  }
  web_search: {
    default_engine: string
    engines: Array<{ name: string; keyword: string; url: string }>
  }
}

interface Conversation {
  id: string
  title: string
  created_at: string
}

interface WorkflowItem {
  id: string
  name: string
  author: string
}

// =============================================================================
// Navigation Data
// =============================================================================

const primaryNavItems = [
  { id: 'general' as PrimaryTab, label: 'General', icon: Settings },
  { id: 'features' as PrimaryTab, label: 'Features', icon: Zap },
  { id: 'workflows' as PrimaryTab, label: 'Workflows', icon: Workflow },
  { id: 'ai' as PrimaryTab, label: 'AI', icon: Bot },
  { id: 'marketplace' as PrimaryTab, label: '插件市场', icon: Store },
  { id: 'plugins' as PrimaryTab, label: '插件中心', icon: Puzzle },
]

const bottomNavItems = [
  { id: 'advanced', label: 'Advanced', icon: Wrench },
  { id: 'about', label: 'About', icon: Info },
]

const featureSubItems = [
  { id: 'default-results' as FeatureSubTab, label: 'Default Results', subtitle: 'Scope, Types, Fallbacks', icon: Search },
  { id: 'file-search' as FeatureSubTab, label: 'File Search', subtitle: 'Search, Navigation, Buffer', icon: FileSearch },
  { id: 'universal-actions' as FeatureSubTab, label: 'Universal Actions', subtitle: 'Files, Text, URLs', icon: MousePointer },
  { id: 'web-search' as FeatureSubTab, label: 'Web Search', subtitle: 'Custom, URLs, History', icon: Globe },
  { id: 'web-bookmarks' as FeatureSubTab, label: 'Web Bookmarks', subtitle: 'Safari, Chrome', icon: Bookmark },
  { id: 'clipboard-history' as FeatureSubTab, label: 'Clipboard History', subtitle: 'History, Merging', icon: ClipboardList },
  { id: 'snippets' as FeatureSubTab, label: 'Snippets', subtitle: 'Snippets, Clippings', icon: FileText },
  { id: 'calculator' as FeatureSubTab, label: 'Calculator', subtitle: 'Standard, Advanced', icon: Calculator },
  { id: 'dictionary' as FeatureSubTab, label: 'Dictionary', subtitle: 'Spelling, Definitions', icon: BookOpen },
  { id: 'contacts' as FeatureSubTab, label: 'Contacts', subtitle: 'Viewer, Emailing', icon: User },
  { id: 'system' as FeatureSubTab, label: 'System', subtitle: 'Commands, Quitting, Ejecting', icon: Settings },
  { id: 'terminal' as FeatureSubTab, label: 'Terminal', subtitle: 'Prefix, Custom Integration', icon: Terminal },
  { id: 'large-type' as FeatureSubTab, label: 'Large Type', subtitle: 'Display, Font', icon: Type },
  { id: 'previews' as FeatureSubTab, label: 'Previews', subtitle: 'Quick Look, Preview panels', icon: Eye },
]

// =============================================================================
// Main Component
// =============================================================================

const AlfredSettings: Component = () => {
  // State - Default to 'general' tab
  const [activeTab, setActiveTab] = createSignal<PrimaryTab>('general')
  const [activeFeature, setActiveFeature] = createSignal<FeatureSubTab>('default-results')
  const [activeWorkflow, setActiveWorkflow] = createSignal<string | null>(null)
  const [activeConversation, setActiveConversation] = createSignal<string | null>(null)
  const [config, setConfig] = createSignal<AppConfig | null>(null)
  const [conversations, setConversations] = createSignal<Conversation[]>([])
  const [workflows, setWorkflows] = createSignal<WorkflowItem[]>([])
  const [aiMessage, setAiMessage] = createSignal('')
  const [selectedModel, setSelectedModel] = createSignal('gpt-4')
  const [searchQuery, setSearchQuery] = createSignal('')

  // Load data on mount and reset to general tab
  onMount(async () => {
    // Always reset to general tab when settings window opens
    setActiveTab('general')
    setActiveFeature('default-results')
    
    // Listen for window focus to reset state
    const currentWindow = getCurrentWindow()
    const unlisten = await currentWindow.onFocusChanged(({ payload: focused }) => {
      if (focused) {
        // Reset to general tab when window gains focus
        setActiveTab('general')
      }
    })
    
    // Cleanup listener on unmount
    onCleanup(() => {
      unlisten()
    })
    
    try {
      const cfg = await invoke<AppConfig>('get_config')
      setConfig(cfg)
    } catch (e) {
      console.error('Failed to load config:', e)
    }

    try {
      const convs = await invoke<Conversation[]>('get_ai_conversations')
      setConversations(convs)
    } catch (e) {
      console.error('Failed to load conversations:', e)
    }

    // Mock workflows for now
    setWorkflows([
      { id: '1', name: 'Quick Notes', author: 'User' },
      { id: '2', name: 'Clipboard Formatter', author: 'Unknown' },
    ])
  })

  // Get current feature item
  const currentFeature = createMemo(() => {
    return featureSubItems.find(item => item.id === activeFeature()) || featureSubItems[0]
  })

  // AI models
  const aiModels = [
    { id: 'gpt-4o', name: 'GPT-4o' },
    { id: 'gpt-4', name: 'GPT-4' },
    { id: 'gpt-3.5-turbo', name: 'GPT-3.5 Turbo' },
    { id: 'claude-3.5-sonnet', name: 'Claude 3.5 Sonnet' },
    { id: 'claude-3-opus', name: 'Claude 3 Opus' },
    { id: 'llama3.2', name: 'Llama 3.2 (Local)' },
  ]

  return (
    <div class="alfred-settings-window">
      {/* ================================================================
          Primary Sidebar (Level 1)
          ================================================================ */}
      <div class="alfred-primary-sidebar">
        <nav class="alfred-primary-nav">
          <For each={primaryNavItems}>
            {(item) => (
              <button
                onClick={() => setActiveTab(item.id)}
                class={`alfred-primary-nav-item ${activeTab() === item.id ? 'active' : ''}`}
                title={item.label}
              >
                <div class="alfred-nav-icon">
                  <item.icon size={22} />
                </div>
                <span class="alfred-nav-label">{item.label}</span>
              </button>
            )}
          </For>
        </nav>
        
        <div class="alfred-primary-bottom">
          <div class="alfred-divider" />
          <For each={bottomNavItems}>
            {(item) => (
              <button
                class="alfred-primary-nav-item"
                title={item.label}
              >
                <div class="alfred-nav-icon">
                  <item.icon size={22} />
                </div>
                <span class="alfred-nav-label">{item.label}</span>
              </button>
            )}
          </For>
        </div>
      </div>

      {/* ================================================================
          Content Area
          ================================================================ */}
      <div class="alfred-content-area">
        
        {/* ============================================================
            Plugin Marketplace Page - Single Column
            ============================================================ */}
        <Show when={activeTab() === 'marketplace'}>
          <div class="alfred-single-column">
            <div class="alfred-detail-header">
              <div class="alfred-detail-header-icon">
                <Store size={28} />
              </div>
              <div class="alfred-detail-header-text">
                <h2 class="alfred-detail-title">插件市场</h2>
                <p class="alfred-detail-subtitle">发现和安装插件来增强 OmniBox 功能</p>
              </div>
              <button class="alfred-help-btn" title="Help">
                <HelpCircle size={14} />
              </button>
            </div>
            
            <div class="alfred-detail-content">
              <div class="alfred-settings-section">
                <div class="marketplace-search">
                  <input 
                    type="text" 
                    class="alfred-text-input" 
                    placeholder="搜索插件..."
                    style={{ width: '100%', 'margin-bottom': '16px' }}
                  />
                </div>
                
                <div class="marketplace-categories" style={{ display: 'flex', gap: '8px', 'margin-bottom': '16px', 'flex-wrap': 'wrap' }}>
                  <button class="alfred-checkbox-item" style={{ padding: '6px 12px', 'border-radius': '6px', background: 'var(--alfred-accent)', color: 'white' }}>全部</button>
                  <button class="alfred-checkbox-item" style={{ padding: '6px 12px', 'border-radius': '6px' }}>搜索增强</button>
                  <button class="alfred-checkbox-item" style={{ padding: '6px 12px', 'border-radius': '6px' }}>效率工具</button>
                  <button class="alfred-checkbox-item" style={{ padding: '6px 12px', 'border-radius': '6px' }}>开发工具</button>
                  <button class="alfred-checkbox-item" style={{ padding: '6px 12px', 'border-radius': '6px' }}>AI 助手</button>
                </div>

                <div class="marketplace-grid" style={{ display: 'grid', 'grid-template-columns': 'repeat(auto-fill, minmax(280px, 1fr))', gap: '16px' }}>
                  <div class="plugin-card" style={{ padding: '16px', border: '1px solid var(--alfred-border)', 'border-radius': '12px', background: 'var(--alfred-bg)' }}>
                    <div style={{ display: 'flex', 'align-items': 'center', gap: '12px', 'margin-bottom': '12px' }}>
                      <div style={{ width: '48px', height: '48px', 'border-radius': '10px', background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)', display: 'flex', 'align-items': 'center', 'justify-content': 'center', 'font-size': '24px' }}>🔍</div>
                      <div>
                        <h4 style={{ margin: '0', 'font-weight': '600' }}>快速翻译</h4>
                        <p style={{ margin: '0', 'font-size': '12px', color: 'var(--alfred-text-secondary)' }}>v1.2.0 · 1.2k 下载</p>
                      </div>
                    </div>
                    <p style={{ margin: '0 0 12px', 'font-size': '13px', color: 'var(--alfred-text-secondary)' }}>快速翻译选中的文本，支持多语言</p>
                    <button style={{ width: '100%', padding: '8px', 'border-radius': '6px', background: 'var(--alfred-accent)', color: 'white', border: 'none', cursor: 'pointer' }}>安装</button>
                  </div>
                  
                  <div class="plugin-card" style={{ padding: '16px', border: '1px solid var(--alfred-border)', 'border-radius': '12px', background: 'var(--alfred-bg)' }}>
                    <div style={{ display: 'flex', 'align-items': 'center', gap: '12px', 'margin-bottom': '12px' }}>
                      <div style={{ width: '48px', height: '48px', 'border-radius': '10px', background: 'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)', display: 'flex', 'align-items': 'center', 'justify-content': 'center', 'font-size': '24px' }}>📝</div>
                      <div>
                        <h4 style={{ margin: '0', 'font-weight': '600' }}>Markdown 预览</h4>
                        <p style={{ margin: '0', 'font-size': '12px', color: 'var(--alfred-text-secondary)' }}>v2.0.1 · 890 下载</p>
                      </div>
                    </div>
                    <p style={{ margin: '0 0 12px', 'font-size': '13px', color: 'var(--alfred-text-secondary)' }}>实时预览 Markdown 文件内容</p>
                    <button style={{ width: '100%', padding: '8px', 'border-radius': '6px', background: 'var(--alfred-accent)', color: 'white', border: 'none', cursor: 'pointer' }}>安装</button>
                  </div>
                  
                  <div class="plugin-card" style={{ padding: '16px', border: '1px solid var(--alfred-border)', 'border-radius': '12px', background: 'var(--alfred-bg)' }}>
                    <div style={{ display: 'flex', 'align-items': 'center', gap: '12px', 'margin-bottom': '12px' }}>
                      <div style={{ width: '48px', height: '48px', 'border-radius': '10px', background: 'linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)', display: 'flex', 'align-items': 'center', 'justify-content': 'center', 'font-size': '24px' }}>🤖</div>
                      <div>
                        <h4 style={{ margin: '0', 'font-weight': '600' }}>AI 代码助手</h4>
                        <p style={{ margin: '0', 'font-size': '12px', color: 'var(--alfred-text-secondary)' }}>v1.5.0 · 2.3k 下载</p>
                      </div>
                    </div>
                    <p style={{ margin: '0 0 12px', 'font-size': '13px', color: 'var(--alfred-text-secondary)' }}>智能代码补全和解释</p>
                    <button style={{ width: '100%', padding: '8px', 'border-radius': '6px', background: 'var(--alfred-accent)', color: 'white', border: 'none', cursor: 'pointer' }}>安装</button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </Show>
        
        {/* ============================================================
            Plugin Center Page - Single Column
            ============================================================ */}
        <Show when={activeTab() === 'plugins'}>
          <div class="alfred-single-column">
            <div class="alfred-detail-header">
              <div class="alfred-detail-header-icon">
                <Puzzle size={28} />
              </div>
              <div class="alfred-detail-header-text">
                <h2 class="alfred-detail-title">插件中心</h2>
                <p class="alfred-detail-subtitle">管理已安装的插件</p>
              </div>
              <button class="alfred-help-btn" title="Help">
                <HelpCircle size={14} />
              </button>
            </div>
            
            <div class="alfred-detail-content">
              <div class="alfred-settings-section">
                <h3 class="alfred-section-title">已安装插件</h3>
                
                <div class="installed-plugins" style={{ display: 'flex', 'flex-direction': 'column', gap: '12px' }}>
                  <div class="plugin-item" style={{ display: 'flex', 'align-items': 'center', 'justify-content': 'space-between', padding: '12px 16px', border: '1px solid var(--alfred-border)', 'border-radius': '10px' }}>
                    <div style={{ display: 'flex', 'align-items': 'center', gap: '12px' }}>
                      <div style={{ width: '40px', height: '40px', 'border-radius': '8px', background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)', display: 'flex', 'align-items': 'center', 'justify-content': 'center', 'font-size': '20px' }}>🔍</div>
                      <div>
                        <h4 style={{ margin: '0', 'font-weight': '500' }}>快速翻译</h4>
                        <p style={{ margin: '0', 'font-size': '12px', color: 'var(--alfred-text-secondary)' }}>v1.2.0 · 已启用</p>
                      </div>
                    </div>
                    <div style={{ display: 'flex', gap: '8px' }}>
                      <button style={{ padding: '6px 12px', 'border-radius': '6px', border: '1px solid var(--alfred-border)', background: 'transparent', cursor: 'pointer' }}>设置</button>
                      <button style={{ padding: '6px 12px', 'border-radius': '6px', border: '1px solid #ef4444', color: '#ef4444', background: 'transparent', cursor: 'pointer' }}>卸载</button>
                    </div>
                  </div>
                  
                  <div class="plugin-item" style={{ display: 'flex', 'align-items': 'center', 'justify-content': 'space-between', padding: '12px 16px', border: '1px solid var(--alfred-border)', 'border-radius': '10px', opacity: '0.6' }}>
                    <div style={{ display: 'flex', 'align-items': 'center', gap: '12px' }}>
                      <div style={{ width: '40px', height: '40px', 'border-radius': '8px', background: 'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)', display: 'flex', 'align-items': 'center', 'justify-content': 'center', 'font-size': '20px' }}>📝</div>
                      <div>
                        <h4 style={{ margin: '0', 'font-weight': '500' }}>Markdown 预览</h4>
                        <p style={{ margin: '0', 'font-size': '12px', color: 'var(--alfred-text-secondary)' }}>v2.0.1 · 已禁用</p>
                      </div>
                    </div>
                    <div style={{ display: 'flex', gap: '8px' }}>
                      <button style={{ padding: '6px 12px', 'border-radius': '6px', border: '1px solid var(--alfred-border)', background: 'transparent', cursor: 'pointer' }}>启用</button>
                      <button style={{ padding: '6px 12px', 'border-radius': '6px', border: '1px solid #ef4444', color: '#ef4444', background: 'transparent', cursor: 'pointer' }}>卸载</button>
                    </div>
                  </div>
                </div>
              </div>
              
              <div class="alfred-settings-section" style={{ 'margin-top': '24px' }}>
                <h3 class="alfred-section-title">插件更新</h3>
                <p style={{ color: 'var(--alfred-text-secondary)', 'font-size': '13px' }}>所有插件都是最新版本 ✓</p>
              </div>
            </div>
          </div>
        </Show>
        
        {/* ============================================================
            General Page - Single Column
            ============================================================ */}
        <Show when={activeTab() === 'general'}>
          <div class="alfred-single-column">
            <div class="alfred-detail-header">
              <div class="alfred-detail-header-icon">
                <Settings size={28} />
              </div>
              <div class="alfred-detail-header-text">
                <h2 class="alfred-detail-title">General</h2>
                <p class="alfred-detail-subtitle">Basic application settings and preferences</p>
              </div>
              <button class="alfred-help-btn" title="Help">
                <HelpCircle size={14} />
              </button>
            </div>
            
            <div class="alfred-detail-content">
              <div class="alfred-settings-section">
                <h3 class="alfred-section-title">Startup</h3>
                <label class="alfred-checkbox-item">
                  <input 
                    type="checkbox" 
                    class="alfred-checkbox" 
                    checked={config()?.general.auto_start}
                    onChange={(e) => {
                      const cfg = config()
                      if (cfg) {
                        setConfig({
                          ...cfg,
                          general: { ...cfg.general, auto_start: e.currentTarget.checked }
                        })
                      }
                    }}
                  />
                  <span>Launch OmniBox at login</span>
                </label>
              </div>
              
              <div class="alfred-settings-section">
                <h3 class="alfred-section-title">Updates</h3>
                <label class="alfred-checkbox-item">
                  <input 
                    type="checkbox" 
                    class="alfred-checkbox" 
                    checked={config()?.general.check_updates}
                    onChange={(e) => {
                      const cfg = config()
                      if (cfg) {
                        setConfig({
                          ...cfg,
                          general: { ...cfg.general, check_updates: e.currentTarget.checked }
                        })
                      }
                    }}
                  />
                  <span>Automatically check for updates</span>
                </label>
              </div>
              
              <div class="alfred-settings-section">
                <h3 class="alfred-section-title">Language</h3>
                <select 
                  class="alfred-select"
                  value={config()?.general.language || 'en'}
                >
                  <option value="en">English</option>
                  <option value="zh-CN">简体中文</option>
                  <option value="zh-TW">繁體中文</option>
                  <option value="ja">日本語</option>
                </select>
              </div>

              <div class="alfred-settings-section">
                <h3 class="alfred-section-title">Permissions</h3>
                <p class="alfred-section-desc">
                  OmniBox needs Accessibility permissions to register global shortcuts.
                </p>
                <button class="alfred-btn alfred-btn-secondary">
                  Request Permissions
                </button>
              </div>
            </div>
          </div>
        </Show>

        {/* ============================================================
            Features Page - Three Columns
            ============================================================ */}
        <Show when={activeTab() === 'features'}>
          {/* Sub-Sidebar */}
          <div class="alfred-sub-sidebar">
            <div class="alfred-sub-sidebar-scroll">
              <For each={featureSubItems}>
                {(item) => (
                  <button
                    onClick={() => setActiveFeature(item.id)}
                    class={`alfred-sub-nav-item ${activeFeature() === item.id ? 'active' : ''}`}
                  >
                    <div class="alfred-sub-nav-icon">
                      <item.icon size={18} />
                    </div>
                    <div class="alfred-sub-nav-text">
                      <span class="alfred-sub-nav-label">{item.label}</span>
                      <span class="alfred-sub-nav-subtitle">{item.subtitle}</span>
                    </div>
                  </button>
                )}
              </For>
            </div>
          </div>
          
          {/* Detail Panel */}
          <div class="alfred-detail-panel">
            <div class="alfred-detail-header">
              <div class="alfred-detail-header-icon">
                {(() => {
                  const Icon = currentFeature().icon
                  return <Icon size={28} />
                })()}
              </div>
              <div class="alfred-detail-header-text">
                <h2 class="alfred-detail-title">{currentFeature().label}</h2>
                <p class="alfred-detail-subtitle">
                  Configure {currentFeature().label.toLowerCase()} settings and behavior.
                </p>
              </div>
              <button class="alfred-help-btn" title="Help">
                <HelpCircle size={14} />
              </button>
            </div>
            
            <div class="alfred-detail-content">
              <FeatureContent 
                feature={activeFeature()} 
                config={config()} 
                onConfigChange={setConfig}
              />
            </div>
          </div>
        </Show>

        {/* ============================================================
            Workflows Page - Three Columns
            ============================================================ */}
        <Show when={activeTab() === 'workflows'}>
          {/* Workflow List Sidebar */}
          <div class="alfred-sub-sidebar">
            <div class="alfred-sub-sidebar-header">
              <div class="alfred-search-box">
                <Search size={14} class="alfred-search-icon" />
                <input 
                  type="text" 
                  placeholder="Filter" 
                  class="alfred-search-input"
                  value={searchQuery()}
                  onInput={(e) => setSearchQuery(e.currentTarget.value)}
                />
              </div>
            </div>
            
            <div class="alfred-sub-sidebar-scroll">
              <For each={workflows()}>
                {(workflow) => (
                  <button
                    onClick={() => setActiveWorkflow(workflow.id)}
                    class={`alfred-workflow-item ${activeWorkflow() === workflow.id ? 'active' : ''}`}
                  >
                    <div class="alfred-workflow-icon">
                      <Workflow size={16} />
                    </div>
                    <div class="alfred-workflow-info">
                      <span class="alfred-workflow-name">{workflow.name}</span>
                      <span class="alfred-workflow-author">by {workflow.author}</span>
                    </div>
                  </button>
                )}
              </For>
            </div>
          </div>
          
          {/* Workflow Editor/Empty State */}
          <div class="alfred-detail-panel alfred-workflow-panel">
            <Show when={activeWorkflow()}>
              <div class="alfred-workflow-content">
                <div class="alfred-workflow-canvas">
                  <div class="alfred-workflow-canvas-empty">
                    <p>Workflow editor coming soon</p>
                  </div>
                </div>
              </div>
            </Show>
            <Show when={!activeWorkflow()}>
              <WorkflowEmptyState />
            </Show>
          </div>
        </Show>

        {/* ============================================================
            AI Page - Three Columns
            ============================================================ */}
        <Show when={activeTab() === 'ai'}>
          {/* AI History Sidebar */}
          <div class="alfred-sub-sidebar">
            <div class="alfred-sub-sidebar-header">
              <button 
                class="alfred-new-chat-btn" 
                onClick={() => setActiveConversation(null)}
              >
                <Plus size={14} />
                <span>New Chat</span>
              </button>
            </div>
            
            <div class="alfred-search-box-wrapper">
              <div class="alfred-search-box">
                <Search size={14} class="alfred-search-icon" />
                <input 
                  type="text" 
                  placeholder="Search conversations..." 
                  class="alfred-search-input"
                />
              </div>
            </div>
            
            <div class="alfred-sub-sidebar-scroll">
              <For each={conversations()}>
                {(conv) => (
                  <button
                    onClick={() => setActiveConversation(conv.id)}
                    class={`alfred-conversation-item ${activeConversation() === conv.id ? 'active' : ''}`}
                  >
                    <div class="alfred-conversation-icon">
                      <Bot size={16} />
                    </div>
                    <div class="alfred-conversation-info">
                      <span class="alfred-conversation-title">{conv.title}</span>
                      <span class="alfred-conversation-date">{conv.created_at}</span>
                    </div>
                  </button>
                )}
              </For>
              
              <Show when={conversations().length === 0}>
                <div class="alfred-placeholder" style="padding: 20px; text-align: center;">
                  <p>No conversations yet</p>
                </div>
              </Show>
            </div>
          </div>
          
          {/* AI Chat Panel */}
          <div class="alfred-detail-panel alfred-ai-panel">
            <div class="alfred-ai-content">
              {/* Chat Header */}
              <div class="alfred-ai-header">
                <h3 class="alfred-ai-title">
                  {activeConversation() ? 'Conversation' : 'New Conversation'}
                </h3>
              </div>

              {/* Chat Messages */}
              <div class="alfred-ai-messages">
                <Show when={!activeConversation()}>
                  <div class="alfred-ai-welcome">
                    <div class="alfred-ai-welcome-icon">
                      <Bot size={48} />
                    </div>
                    <h4>How can I help you today?</h4>
                    <p>Start a conversation by typing below.</p>
                  </div>
                </Show>
              </div>

              {/* Input Area */}
              <div class="alfred-ai-input-area">
                <textarea 
                  class="alfred-ai-textarea"
                  placeholder="Type your message..."
                  value={aiMessage()}
                  onInput={(e) => setAiMessage(e.currentTarget.value)}
                  rows={3}
                />
                {/* Model Selector - below textarea */}
                <div class="alfred-ai-input-footer">
                  <div class="alfred-ai-model-selector">
                    <label class="alfred-ai-model-label">Model:</label>
                    <select 
                      class="alfred-select alfred-select-sm"
                      value={selectedModel()}
                      onChange={(e) => setSelectedModel(e.currentTarget.value)}
                    >
                      <For each={aiModels}>
                        {(model) => (
                          <option value={model.id}>{model.name}</option>
                        )}
                      </For>
                    </select>
                  </div>
                  <button 
                    class="alfred-btn alfred-btn-primary"
                    disabled={!aiMessage().trim()}
                  >
                    Send
                  </button>
                </div>
              </div>
            </div>
          </div>
        </Show>
      </div>
    </div>
  )
}

// =============================================================================
// Feature Content Component
// =============================================================================

interface FeatureContentProps {
  feature: FeatureSubTab
  config: AppConfig | null
  onConfigChange: (config: AppConfig) => void
}

const FeatureContent: Component<FeatureContentProps> = (props) => {
  return (
    <div class="alfred-feature-content">
      {/* Default Results */}
      <Show when={props.feature === 'default-results'}>
        <div class="alfred-form-row">
          <label class="alfred-form-label">Applications:</label>
          <button class="alfred-btn alfred-btn-secondary">Options...</button>
        </div>

        <div class="alfred-form-section">
          <label class="alfred-form-label">Essentials:</label>
          <div class="alfred-checkbox-grid">
            <label class="alfred-checkbox-item">
              <input type="checkbox" class="alfred-checkbox" checked />
              <span>Preferences</span>
            </label>
            <label class="alfred-checkbox-item">
              <input type="checkbox" class="alfred-checkbox" checked />
              <span>Contacts</span>
            </label>
          </div>
        </div>

        <div class="alfred-form-section">
          <label class="alfred-form-label">Extras:</label>
          <div class="alfred-checkbox-grid">
            <label class="alfred-checkbox-item">
              <input type="checkbox" class="alfred-checkbox" checked />
              <span>Folders</span>
            </label>
            <label class="alfred-checkbox-item">
              <input type="checkbox" class="alfred-checkbox" />
              <span>Documents</span>
            </label>
            <label class="alfred-checkbox-item">
              <input type="checkbox" class="alfred-checkbox" />
              <span>Text Files</span>
            </label>
            <label class="alfred-checkbox-item">
              <input type="checkbox" class="alfred-checkbox" />
              <span>Images</span>
            </label>
            <label class="alfred-checkbox-item">
              <input type="checkbox" class="alfred-checkbox" />
              <span>Archives</span>
            </label>
            <label class="alfred-checkbox-item">
              <input type="checkbox" class="alfred-checkbox" />
              <span>AppleScripts</span>
            </label>
          </div>
          <button class="alfred-btn alfred-btn-secondary alfred-btn-sm">Advanced...</button>
        </div>

        <div class="alfred-note">
          <p>Note: OmniBox works most efficiently if you only have 'Essential' items ticked and use the 'open' keyword to find files.</p>
          <p>You can also press [spacebar] immediately after activating OmniBox to quickly enter the file search mode.</p>
        </div>

        <div class="alfred-form-section">
          <label class="alfred-form-label">Search Scope:</label>
          <div class="alfred-checkbox-inline">
            <label class="alfred-checkbox-item">
              <input type="checkbox" class="alfred-checkbox" checked />
              <span>macOS Applications folder</span>
            </label>
            <label class="alfred-checkbox-item">
              <input type="checkbox" class="alfred-checkbox" checked />
              <span>Folders in Home</span>
              <span class="alfred-note-inline">- Excluding ~/Library</span>
            </label>
          </div>
        </div>

        <div class="alfred-paths-list">
          <div class="alfred-path-item">
            <FolderOpen size={14} class="alfred-path-icon" />
            <span>/Applications</span>
          </div>
          <div class="alfred-path-item">
            <FolderOpen size={14} class="alfred-path-icon" />
            <span>/System/Library/CoreServices/Applications</span>
          </div>
          <div class="alfred-path-item">
            <FolderOpen size={14} class="alfred-path-icon" />
            <span>~/Documents</span>
          </div>
          <div class="alfred-path-item">
            <FolderOpen size={14} class="alfred-path-icon" />
            <span>~/Desktop</span>
          </div>
        </div>

        <div class="alfred-paths-actions">
          <button class="alfred-btn alfred-btn-icon"><Plus size={14} /></button>
          <button class="alfred-btn alfred-btn-icon"><Minus size={14} /></button>
          <button class="alfred-btn alfred-btn-secondary alfred-btn-sm">Reset...</button>
        </div>

        <div class="alfred-form-row alfred-form-row-spaced">
          <label class="alfred-form-label">Fallbacks:</label>
          <div class="alfred-select-row">
            <select class="alfred-select">
              <option>Only show fallbacks when there are no results</option>
              <option>Always show fallbacks</option>
              <option>Never show fallbacks</option>
            </select>
            <button class="alfred-btn alfred-btn-secondary">Setup fallback results</button>
          </div>
        </div>
      </Show>

      {/* Clipboard History */}
      <Show when={props.feature === 'clipboard-history'}>
        <div class="alfred-form-section">
          <label class="alfred-checkbox-item">
            <input 
              type="checkbox" 
              class="alfred-checkbox" 
              checked={props.config?.clipboard.enabled}
            />
            <span>Keep clipboard history</span>
          </label>
        </div>
        
        <div class="alfred-form-section">
          <label class="alfred-form-label">Viewer Hotkey:</label>
          <div class="alfred-hotkey-input">
            <input type="text" value="⌘⇧V" class="alfred-input alfred-input-sm" readonly />
          </div>
        </div>
        
        <div class="alfred-form-section">
          <label class="alfred-form-label">History Limit:</label>
          <select 
            class="alfred-select alfred-select-sm"
            value={props.config?.clipboard.history_limit || 1000}
          >
            <option value="100">100 items</option>
            <option value="500">500 items</option>
            <option value="1000">1000 items</option>
            <option value="5000">5000 items</option>
          </select>
        </div>
        
        <div class="alfred-form-section">
          <label class="alfred-checkbox-item">
            <input 
              type="checkbox" 
              class="alfred-checkbox" 
              checked={props.config?.clipboard.filter_sensitive}
            />
            <span>Filter sensitive content (passwords, credit cards)</span>
          </label>
        </div>
        
        <div class="alfred-form-section">
          <label class="alfred-form-label">Ignored Apps:</label>
          <p class="alfred-section-desc">
            Clipboard content from these apps won't be saved.
          </p>
          <div class="alfred-paths-list">
            <For each={props.config?.clipboard.exclude_apps || []}>
              {(app) => (
                <div class="alfred-path-item">
                  <span>{app}</span>
                </div>
              )}
            </For>
          </div>
          <div class="alfred-paths-actions">
            <button class="alfred-btn alfred-btn-icon"><Plus size={14} /></button>
            <button class="alfred-btn alfred-btn-icon"><Minus size={14} /></button>
          </div>
        </div>
      </Show>

      {/* Web Search */}
      <Show when={props.feature === 'web-search'}>
        <div class="alfred-form-section">
          <label class="alfred-form-label">Default Search Engine:</label>
          <select 
            class="alfred-select"
            value={props.config?.web_search.default_engine || 'google'}
          >
            <option value="google">Google</option>
            <option value="baidu">Baidu</option>
            <option value="bing">Bing</option>
            <option value="duckduckgo">DuckDuckGo</option>
          </select>
        </div>
        
        <div class="alfred-form-section">
          <label class="alfred-form-label">Custom Searches:</label>
          <div class="alfred-paths-list">
            <For each={props.config?.web_search.engines || []}>
              {(engine) => (
                <div class="alfred-path-item">
                  <span><strong>{engine.keyword}</strong> - {engine.name}</span>
                </div>
              )}
            </For>
          </div>
          <div class="alfred-paths-actions">
            <button class="alfred-btn alfred-btn-icon"><Plus size={14} /></button>
            <button class="alfred-btn alfred-btn-icon"><Minus size={14} /></button>
          </div>
        </div>
      </Show>

      {/* Calculator */}
      <Show when={props.feature === 'calculator'}>
        <div class="alfred-form-section">
          <label class="alfred-checkbox-item">
            <input 
              type="checkbox" 
              class="alfred-checkbox" 
              checked={props.config?.features.calculator}
            />
            <span>Enable calculator</span>
          </label>
        </div>
        
        <div class="alfred-note">
          <p>The calculator supports basic arithmetic, percentages, and common math functions.</p>
          <p>Examples: 2+2, 15% of 200, sqrt(16), sin(45)</p>
        </div>
      </Show>

      {/* File Search */}
      <Show when={props.feature === 'file-search'}>
        <div class="alfred-form-section">
          <label class="alfred-checkbox-item">
            <input 
              type="checkbox" 
              class="alfred-checkbox" 
              checked={props.config?.features.file_search}
            />
            <span>Enable file search</span>
          </label>
        </div>
        
        <div class="alfred-form-section">
          <label class="alfred-form-label">Search Keywords:</label>
          <div class="alfred-note">
            <p><strong>open</strong> - Search for files to open</p>
            <p><strong>find</strong> - Search for files in Finder</p>
            <p><strong>in</strong> - Search inside file contents</p>
          </div>
        </div>
        
        <div class="alfred-form-section">
          <label class="alfred-form-label">Indexed Paths:</label>
          <div class="alfred-paths-list">
            <For each={props.config?.indexer.index_paths || []}>
              {(path) => (
                <div class="alfred-path-item">
                  <FolderOpen size={14} />
                  <span>{path}</span>
                </div>
              )}
            </For>
          </div>
          <div class="alfred-paths-actions">
            <button class="alfred-btn alfred-btn-icon"><Plus size={14} /></button>
            <button class="alfred-btn alfred-btn-icon"><Minus size={14} /></button>
            <button class="alfred-btn alfred-btn-secondary alfred-btn-sm">Rebuild Index</button>
          </div>
        </div>
      </Show>

      {/* Default placeholder for other features */}
      <Show when={
        !['default-results', 'clipboard-history', 'web-search', 'calculator', 'file-search'].includes(props.feature)
      }>
        <div class="alfred-placeholder">
          <p>Configuration for {props.feature} will be available soon.</p>
        </div>
      </Show>
    </div>
  )
}

// =============================================================================
// Workflow Empty State
// =============================================================================

const WorkflowEmptyState: Component = () => {
  const items = [
    { icon: '⭐', title: 'Workflow Gallery', desc: 'Browse gallery and install amazing Workflows.' },
    { icon: '🚀', title: 'Getting Started Guide', desc: 'Check out our tutorials to learn the basics.' },
    { icon: '📝', title: 'Create a new Workflow', desc: 'Jump in and create a blank Workflow.' },
    { icon: '📋', title: 'Create from Template', desc: 'New to workflows? Try one of our templates.' },
    { icon: '📚', title: 'View Documentation', desc: 'Learn how to configure workflow objects.' },
  ]

  return (
    <div class="alfred-workflow-empty">
      <div class="alfred-empty-state">
        <For each={items}>
          {(item) => (
            <div class="alfred-empty-item">
              <div class="alfred-empty-icon">{item.icon}</div>
              <div class="alfred-empty-text">
                <h4>{item.title}</h4>
                <p>{item.desc}</p>
              </div>
            </div>
          )}
        </For>
      </div>
    </div>
  )
}

export default AlfredSettings
