/**
 * Alfred-style Settings Layout
 * 
 * Component Architecture:
 * ┌────────────────────────────────────────────────────────────────────┐
 * │ SettingsWindow                                                      │
 * │ ┌──────────┬─────────────────────────────────────────────────────┐ │
 * │ │          │                                                      │ │
 * │ │ Primary  │  ContentArea                                         │ │
 * │ │ Sidebar  │  ┌─────────────┬───────────────────────────────────┐ │ │
 * │ │          │  │             │                                    │ │ │
 * │ │ General  │  │  Secondary  │   Detail Panel                     │ │ │
 * │ │ Features │  │  Sidebar    │   (Form content)                   │ │ │
 * │ │ Workflows│  │  (SubMenu)  │                                    │ │ │
 * │ │ AI       │  │             │                                    │ │ │
 * │ │          │  │             │                                    │ │ │
 * │ │          │  └─────────────┴───────────────────────────────────┘ │ │
 * │ └──────────┴─────────────────────────────────────────────────────┘ │
 * └────────────────────────────────────────────────────────────────────┘
 * 
 * Layout Rules:
 * - General: Single column (no sub-sidebar)
 * - Features: Three columns (primary + sub-sidebar + detail)
 * - Workflows: Three columns (primary + workflow list + editor)
 * - AI: Three columns (primary + history + chat)
 */

import { Component, JSX, For, Show, createSignal } from 'solid-js'
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
  Music,
  Key,
  Terminal,
  Type,
  Eye
} from 'lucide-solid'

// =============================================================================
// Types
// =============================================================================

export type PrimaryTab = 'general' | 'features' | 'workflows' | 'ai'

export type FeatureSubTab = 
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
  | 'music'
  | '1password'
  | 'system'
  | 'terminal'
  | 'large-type'
  | 'previews'

interface SettingsLayoutProps {
  children?: JSX.Element
}

// =============================================================================
// Primary Navigation Items
// =============================================================================

const primaryNavItems = [
  { id: 'general' as PrimaryTab, label: 'General', icon: Settings },
  { id: 'features' as PrimaryTab, label: 'Features', icon: Zap },
  { id: 'workflows' as PrimaryTab, label: 'Workflows', icon: Workflow },
  { id: 'ai' as PrimaryTab, label: 'AI', icon: Bot },
]

// =============================================================================
// Features Sub-Navigation Items (Alfred-style)
// =============================================================================

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
  { id: 'music' as FeatureSubTab, label: 'Music', subtitle: 'Mini Player', icon: Music },
  { id: '1password' as FeatureSubTab, label: '1Password', subtitle: '1Click Bookmarks', icon: Key },
  { id: 'system' as FeatureSubTab, label: 'System', subtitle: 'Commands, Quitting, Ejecting', icon: Settings },
  { id: 'terminal' as FeatureSubTab, label: 'Terminal', subtitle: 'Prefix, Custom Integration', icon: Terminal },
  { id: 'large-type' as FeatureSubTab, label: 'Large Type', subtitle: 'Display, Font', icon: Type },
  { id: 'previews' as FeatureSubTab, label: 'Previews', subtitle: 'Quick Look, Preview panels', icon: Eye },
]

// =============================================================================
// Primary Sidebar Component (Alfred-style Icon + Label)
// =============================================================================

interface PrimarySidebarProps {
  activeTab: PrimaryTab
  onTabChange: (tab: PrimaryTab) => void
}

const PrimarySidebar: Component<PrimarySidebarProps> = (props) => {
  return (
    <div class="alfred-primary-sidebar">
      <nav class="alfred-primary-nav">
        <For each={primaryNavItems}>
          {(item) => (
            <button
              onClick={() => props.onTabChange(item.id)}
              class={`alfred-primary-nav-item ${props.activeTab === item.id ? 'active' : ''}`}
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
      
      {/* Bottom section - like Alfred's Powerpack, Usage, etc. */}
      <div class="alfred-primary-bottom">
        <div class="alfred-divider" />
        <button class="alfred-primary-nav-item" title="Advanced">
          <div class="alfred-nav-icon">
            <Settings size={22} />
          </div>
          <span class="alfred-nav-label">Advanced</span>
        </button>
      </div>
    </div>
  )
}

// =============================================================================
// Features Sub-Sidebar (Alfred-style List)
// =============================================================================

interface FeaturesSidebarProps {
  activeSubTab: FeatureSubTab
  onSubTabChange: (tab: FeatureSubTab) => void
}

const FeaturesSidebar: Component<FeaturesSidebarProps> = (props) => {
  return (
    <div class="alfred-sub-sidebar">
      <div class="alfred-sub-sidebar-scroll">
        <For each={featureSubItems}>
          {(item) => (
            <button
              onClick={() => props.onSubTabChange(item.id)}
              class={`alfred-sub-nav-item ${props.activeSubTab === item.id ? 'active' : ''}`}
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
  )
}

// =============================================================================
// Workflows Sidebar (List of workflows)
// =============================================================================

interface WorkflowsSidebarProps {
  workflows: Array<{ id: string; name: string; author: string }>
  activeWorkflow: string | null
  onWorkflowSelect: (id: string) => void
  onSearch: (query: string) => void
}

const WorkflowsSidebar: Component<WorkflowsSidebarProps> = (props) => {
  return (
    <div class="alfred-sub-sidebar">
      {/* Search/Filter */}
      <div class="alfred-sub-sidebar-header">
        <div class="alfred-search-box">
          <Search size={14} class="alfred-search-icon" />
          <input 
            type="text" 
            placeholder="Filter" 
            class="alfred-search-input"
            onInput={(e) => props.onSearch(e.currentTarget.value)}
          />
        </div>
      </div>
      
      {/* Workflow List */}
      <div class="alfred-sub-sidebar-scroll">
        <For each={props.workflows}>
          {(workflow) => (
            <button
              onClick={() => props.onWorkflowSelect(workflow.id)}
              class={`alfred-workflow-item ${props.activeWorkflow === workflow.id ? 'active' : ''}`}
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
  )
}

// =============================================================================
// AI History Sidebar
// =============================================================================

interface AIHistorySidebarProps {
  conversations: Array<{ id: string; title: string; date: string }>
  activeConversation: string | null
  onConversationSelect: (id: string) => void
  onSearch: (query: string) => void
  onNewChat: () => void
}

const AIHistorySidebar: Component<AIHistorySidebarProps> = (props) => {
  return (
    <div class="alfred-sub-sidebar">
      {/* Header with New Chat button */}
      <div class="alfred-sub-sidebar-header">
        <button class="alfred-new-chat-btn" onClick={props.onNewChat}>
          <span>+ New Chat</span>
        </button>
      </div>
      
      {/* Search */}
      <div class="alfred-search-box-wrapper">
        <div class="alfred-search-box">
          <Search size={14} class="alfred-search-icon" />
          <input 
            type="text" 
            placeholder="Search conversations..." 
            class="alfred-search-input"
            onInput={(e) => props.onSearch(e.currentTarget.value)}
          />
        </div>
      </div>
      
      {/* Conversation List */}
      <div class="alfred-sub-sidebar-scroll">
        <For each={props.conversations}>
          {(conv) => (
            <button
              onClick={() => props.onConversationSelect(conv.id)}
              class={`alfred-conversation-item ${props.activeConversation === conv.id ? 'active' : ''}`}
            >
              <div class="alfred-conversation-icon">
                <Bot size={16} />
              </div>
              <div class="alfred-conversation-info">
                <span class="alfred-conversation-title">{conv.title}</span>
                <span class="alfred-conversation-date">{conv.date}</span>
              </div>
            </button>
          )}
        </For>
      </div>
    </div>
  )
}

// =============================================================================
// Detail Panel Header (Alfred-style with icon)
// =============================================================================

interface DetailHeaderProps {
  icon: Component<{ size: number }>
  title: string
  subtitle?: string
}

const DetailHeader: Component<DetailHeaderProps> = (props) => {
  return (
    <div class="alfred-detail-header">
      <div class="alfred-detail-header-icon">
        <props.icon size={32} />
      </div>
      <div class="alfred-detail-header-text">
        <h2 class="alfred-detail-title">{props.title}</h2>
        <Show when={props.subtitle}>
          <p class="alfred-detail-subtitle">{props.subtitle}</p>
        </Show>
      </div>
      <button class="alfred-help-btn" title="Help">
        ?
      </button>
    </div>
  )
}

// =============================================================================
// Main Layout Component
// =============================================================================

const AlfredSettingsLayout: Component<SettingsLayoutProps> = (_props) => {
  const [activeTab, setActiveTab] = createSignal<PrimaryTab>('features')
  const [activeFeature, setActiveFeature] = createSignal<FeatureSubTab>('default-results')
  const [activeWorkflow, setActiveWorkflow] = createSignal<string | null>('123123')
  const [activeConversation, setActiveConversation] = createSignal<string | null>(null)

  // Mock data
  const workflows = [
    { id: '123123', name: '123123', author: 'Unknown' },
  ]
  
  const conversations = [
    { id: '1', title: 'Code Review Help', date: 'Today' },
    { id: '2', title: 'API Design Discussion', date: 'Yesterday' },
  ]

  const getCurrentFeatureItem = () => {
    return featureSubItems.find(item => item.id === activeFeature()) || featureSubItems[0]
  }

  return (
    <div class="alfred-settings-window">
      {/* Primary Sidebar (Level 1) */}
      <PrimarySidebar 
        activeTab={activeTab()} 
        onTabChange={setActiveTab} 
      />

      {/* Content Area - varies based on active tab */}
      <div class="alfred-content-area">
        {/* General Page - Single column */}
        <Show when={activeTab() === 'general'}>
          <div class="alfred-single-column">
            <DetailHeader 
              icon={Settings} 
              title="General" 
              subtitle="Basic application settings and preferences"
            />
            <div class="alfred-detail-content">
              {/* General settings content */}
              <div class="alfred-settings-section">
                <h3 class="alfred-section-title">Startup</h3>
                <label class="alfred-checkbox-item">
                  <input type="checkbox" class="alfred-checkbox" />
                  <span>Launch OmniBox at login</span>
                </label>
              </div>
              
              <div class="alfred-settings-section">
                <h3 class="alfred-section-title">Updates</h3>
                <label class="alfred-checkbox-item">
                  <input type="checkbox" class="alfred-checkbox" checked />
                  <span>Automatically check for updates</span>
                </label>
              </div>
              
              <div class="alfred-settings-section">
                <h3 class="alfred-section-title">Permissions</h3>
                <p class="alfred-section-desc">
                  OmniBox needs Accessibility permissions to work properly.
                </p>
                <button class="alfred-btn alfred-btn-secondary">
                  Request Permissions
                </button>
              </div>
            </div>
          </div>
        </Show>

        {/* Features Page - Three columns */}
        <Show when={activeTab() === 'features'}>
          <FeaturesSidebar 
            activeSubTab={activeFeature()} 
            onSubTabChange={setActiveFeature} 
          />
          <div class="alfred-detail-panel">
            <DetailHeader 
              icon={getCurrentFeatureItem().icon} 
              title={getCurrentFeatureItem().label}
              subtitle={`These are the main results OmniBox presents by default, and where he looks for them.`}
            />
            <div class="alfred-detail-content">
              {/* Feature-specific content would go here */}
              <FeatureDetailContent activeFeature={activeFeature()} />
            </div>
          </div>
        </Show>

        {/* Workflows Page - Three columns */}
        <Show when={activeTab() === 'workflows'}>
          <WorkflowsSidebar
            workflows={workflows}
            activeWorkflow={activeWorkflow()}
            onWorkflowSelect={setActiveWorkflow}
            onSearch={() => {}}
          />
          <div class="alfred-detail-panel alfred-workflow-panel">
            <Show when={activeWorkflow()}>
              <WorkflowDetailContent />
            </Show>
            <Show when={!activeWorkflow()}>
              <WorkflowEmptyState />
            </Show>
          </div>
        </Show>

        {/* AI Page - Three columns */}
        <Show when={activeTab() === 'ai'}>
          <AIHistorySidebar
            conversations={conversations}
            activeConversation={activeConversation()}
            onConversationSelect={setActiveConversation}
            onSearch={() => {}}
            onNewChat={() => setActiveConversation(null)}
          />
          <div class="alfred-detail-panel alfred-ai-panel">
            <AIChatContent activeConversation={activeConversation()} />
          </div>
        </Show>
      </div>
    </div>
  )
}

// =============================================================================
// Feature Detail Content (Default Results example)
// =============================================================================

const FeatureDetailContent: Component<{ activeFeature: FeatureSubTab }> = (props) => {
  return (
    <div class="alfred-feature-content">
      <Show when={props.activeFeature === 'default-results'}>
        {/* Applications section */}
        <div class="alfred-form-row">
          <label class="alfred-form-label">Applications:</label>
          <button class="alfred-btn alfred-btn-secondary">Options...</button>
        </div>

        {/* Essentials */}
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

        {/* Extras */}
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

        {/* Note */}
        <div class="alfred-note">
          <p>Note: OmniBox works most efficiently if you only have 'Essential' items ticked and use the 'open' keyword to find files.</p>
          <p>You can also press [spacebar] immediately after activating OmniBox to quickly enter the file search mode.</p>
        </div>

        {/* Search Scope */}
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

        {/* Search paths list */}
        <div class="alfred-paths-list">
          <div class="alfred-path-item">
            <span class="alfred-path-icon">📁</span>
            <span>/Applications/Xcode.app/Contents/Applications</span>
          </div>
          <div class="alfred-path-item">
            <span class="alfred-path-icon">📁</span>
            <span>/Developer/Applications</span>
          </div>
          <div class="alfred-path-item">
            <span class="alfred-path-icon">📁</span>
            <span>/Library/PreferencePanes</span>
          </div>
          <div class="alfred-path-item">
            <span class="alfred-path-icon">📁</span>
            <span>/opt/homebrew/Cellar</span>
          </div>
          <div class="alfred-path-item">
            <span class="alfred-path-icon">📁</span>
            <span>/System/Library/CoreServices/Applications</span>
          </div>
        </div>

        {/* Path management buttons */}
        <div class="alfred-paths-actions">
          <button class="alfred-btn alfred-btn-icon">+</button>
          <button class="alfred-btn alfred-btn-icon">−</button>
          <button class="alfred-btn alfred-btn-secondary alfred-btn-sm">Reset...</button>
        </div>

        {/* Fallbacks */}
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

      <Show when={props.activeFeature === 'clipboard-history'}>
        <div class="alfred-form-section">
          <label class="alfred-form-label">History:</label>
          <label class="alfred-checkbox-item">
            <input type="checkbox" class="alfred-checkbox" checked />
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
          <select class="alfred-select alfred-select-sm">
            <option>3 months</option>
            <option>1 month</option>
            <option>1 week</option>
            <option>24 hours</option>
          </select>
        </div>
      </Show>

      <Show when={props.activeFeature !== 'default-results' && props.activeFeature !== 'clipboard-history'}>
        <div class="alfred-placeholder">
          <p>Configuration for {props.activeFeature} will be shown here.</p>
        </div>
      </Show>
    </div>
  )
}

// =============================================================================
// Workflow Detail Content
// =============================================================================

const WorkflowDetailContent: Component = () => {
  return (
    <div class="alfred-workflow-content">
      {/* Workflow canvas would go here */}
      <div class="alfred-workflow-canvas">
        <div class="alfred-workflow-canvas-empty">
          <p>Drag and drop workflow nodes here</p>
        </div>
      </div>
    </div>
  )
}

const WorkflowEmptyState: Component = () => {
  return (
    <div class="alfred-workflow-empty">
      <div class="alfred-empty-state">
        <div class="alfred-empty-item">
          <div class="alfred-empty-icon">⭐</div>
          <div class="alfred-empty-text">
            <h4>Alfred Gallery</h4>
            <p>Browse Alfred Gallery and install amazing Workflows.</p>
          </div>
        </div>
        <div class="alfred-empty-item">
          <div class="alfred-empty-icon">🚀</div>
          <div class="alfred-empty-text">
            <h4>Getting Started Guide</h4>
            <p>Check out our tutorials to learn the basics.</p>
          </div>
        </div>
        <div class="alfred-empty-item">
          <div class="alfred-empty-icon">📝</div>
          <div class="alfred-empty-text">
            <h4>Create a new Workflow</h4>
            <p>Jump in and create a blank Workflow.</p>
          </div>
        </div>
        <div class="alfred-empty-item">
          <div class="alfred-empty-icon">📋</div>
          <div class="alfred-empty-text">
            <h4>Create a Workflow from a Template</h4>
            <p>New to workflows? Try one of our Getting Started templates.</p>
          </div>
        </div>
        <div class="alfred-empty-item">
          <div class="alfred-empty-icon">📚</div>
          <div class="alfred-empty-text">
            <h4>View Documentation</h4>
            <p>Find out more about how to configure and use workflow objects.</p>
          </div>
        </div>
      </div>
    </div>
  )
}

// =============================================================================
// AI Chat Content
// =============================================================================

interface AIChatContentProps {
  activeConversation: string | null
}

const AIChatContent: Component<AIChatContentProps> = (props) => {
  const [message, setMessage] = createSignal('')
  const [selectedModel, setSelectedModel] = createSignal('gpt-4')

  const models = [
    { id: 'gpt-4', name: 'GPT-4' },
    { id: 'gpt-4o', name: 'GPT-4o' },
    { id: 'claude-3.5', name: 'Claude 3.5 Sonnet' },
    { id: 'local-llama', name: 'Local-Llama' },
  ]

  return (
    <div class="alfred-ai-content">
      {/* Chat Header */}
      <div class="alfred-ai-header">
        <h3 class="alfred-ai-title">
          {props.activeConversation ? 'Code Review Help' : 'New Conversation'}
        </h3>
      </div>

      {/* Chat Messages */}
      <div class="alfred-ai-messages">
        <Show when={props.activeConversation}>
          <div class="alfred-ai-message alfred-ai-message-user">
            <div class="alfred-ai-message-content">
              Can you help me review this code?
            </div>
          </div>
          <div class="alfred-ai-message alfred-ai-message-assistant">
            <div class="alfred-ai-message-avatar">
              <Bot size={16} />
            </div>
            <div class="alfred-ai-message-content">
              Of course! Please share the code you'd like me to review, and I'll provide feedback on:
              <ul>
                <li>Code quality and best practices</li>
                <li>Potential bugs or issues</li>
                <li>Performance considerations</li>
                <li>Suggestions for improvement</li>
              </ul>
            </div>
          </div>
        </Show>
        <Show when={!props.activeConversation}>
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
          value={message()}
          onInput={(e) => setMessage(e.currentTarget.value)}
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
              <For each={models}>
                {(model) => (
                  <option value={model.id}>{model.name}</option>
                )}
              </For>
            </select>
          </div>
          <button 
            class="alfred-btn alfred-btn-primary"
            disabled={!message().trim()}
          >
            Send
          </button>
        </div>
      </div>
    </div>
  )
}

export default AlfredSettingsLayout
