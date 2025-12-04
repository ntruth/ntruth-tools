/**
 * Plugin System Types
 * 插件系统类型定义
 */

// 插件元数据
export interface PluginMetadata {
  id: string
  name: string
  version: string
  description: string
  author: string
  homepage?: string
  repository?: string
  license?: string
  icon?: string
  keywords?: string[]
  category: PluginCategory
  minAppVersion?: string
}

// 插件分类
export type PluginCategory = 
  | 'search'      // 搜索提供者
  | 'action'      // 动作处理器
  | 'workflow'    // 工作流节点
  | 'theme'       // 主题
  | 'integration' // 第三方集成
  | 'utility'     // 实用工具
  | 'other'       // 其他

// 插件权限
export type PluginPermission = 
  | 'clipboard:read'     // 读取剪贴板
  | 'clipboard:write'    // 写入剪贴板
  | 'fs:read'           // 读取文件
  | 'fs:write'          // 写入文件
  | 'network'           // 网络访问
  | 'shell'             // Shell 命令
  | 'notification'      // 系统通知
  | 'system'            // 系统信息

// 插件状态
export type PluginStatus = 
  | 'installed'   // 已安装
  | 'enabled'     // 已启用
  | 'disabled'    // 已禁用
  | 'error'       // 错误
  | 'updating'    // 更新中

// 已安装的插件
export interface InstalledPlugin {
  metadata: PluginMetadata
  status: PluginStatus
  permissions: PluginPermission[]
  grantedPermissions: PluginPermission[]
  installedAt: string
  updatedAt: string
  config?: Record<string, unknown>
  error?: string
}

// 插件市场项目
export interface MarketplacePlugin {
  metadata: PluginMetadata
  downloads: number
  rating: number
  ratingCount: number
  lastUpdated: string
  publishedAt: string
  screenshots?: string[]
  readme?: string
  changelog?: string
  installed?: boolean
  installedVersion?: string
  hasUpdate?: boolean
}

// 搜索提供者插件接口
export interface SearchProviderPlugin {
  type: 'search'
  // 搜索触发关键词
  trigger?: string
  // 是否支持全局搜索
  global: boolean
  // 搜索方法
  search: (query: string, limit: number) => Promise<SearchResult[]>
}

// 搜索结果
export interface SearchResult {
  id: string
  title: string
  subtitle?: string
  icon?: string
  action: PluginAction
  score?: number
}

// 动作处理器插件接口
export interface ActionHandlerPlugin {
  type: 'action'
  // 支持的动作类型
  actionTypes: string[]
  // 执行动作
  execute: (action: PluginAction) => Promise<void>
}

// 插件动作
export interface PluginAction {
  type: string
  payload?: unknown
}

// 工作流节点插件接口
export interface WorkflowNodePlugin {
  type: 'workflow'
  // 节点定义
  nodeDefinition: WorkflowNodeDefinition
  // 执行节点
  execute: (input: unknown, config: unknown) => Promise<unknown>
}

// 工作流节点定义
export interface WorkflowNodeDefinition {
  type: string
  name: string
  description: string
  icon?: string
  inputs: WorkflowPort[]
  outputs: WorkflowPort[]
  configSchema?: Record<string, unknown>
}

// 工作流端口
export interface WorkflowPort {
  name: string
  type: 'string' | 'number' | 'boolean' | 'object' | 'array' | 'any'
  required?: boolean
  description?: string
}

// 插件配置 Schema
export interface PluginConfigSchema {
  type: 'object'
  properties: Record<string, PluginConfigProperty>
  required?: string[]
}

export interface PluginConfigProperty {
  type: 'string' | 'number' | 'boolean' | 'select' | 'multiselect'
  title: string
  description?: string
  default?: unknown
  options?: { label: string; value: unknown }[]
  min?: number
  max?: number
  placeholder?: string
}

// 插件市场筛选
export interface MarketplaceFilter {
  category?: PluginCategory
  search?: string
  sort?: 'popular' | 'newest' | 'updated' | 'rating'
  page?: number
  pageSize?: number
}

// 插件市场响应
export interface MarketplaceResponse {
  plugins: MarketplacePlugin[]
  total: number
  page: number
  pageSize: number
}

// 插件安装选项
export interface PluginInstallOptions {
  pluginId: string
  version?: string
  grantPermissions?: PluginPermission[]
}

// 插件更新信息
export interface PluginUpdateInfo {
  pluginId: string
  currentVersion: string
  latestVersion: string
  changelog?: string
  breaking?: boolean
}
