import { Component, createSignal, createResource, For, Show } from 'solid-js'
import { invoke } from '@tauri-apps/api/core'
import { 
  Search, 
  Download, 
  Star, 
  RefreshCw, 
  ChevronLeft,
  ExternalLink,
  Check,
  Package
} from 'lucide-solid'
import type { 
  MarketplacePlugin, 
  MarketplaceFilter, 
  MarketplaceResponse,
  PluginCategory
} from '../../types/plugin'

/**
 * 插件市场页面
 */
export const PluginMarket: Component = () => {
  const [searchQuery, setSearchQuery] = createSignal('')
  const [selectedCategory, setSelectedCategory] = createSignal<PluginCategory | undefined>()
  const [sortBy, setSortBy] = createSignal<'popular' | 'newest' | 'updated' | 'rating'>('popular')
  const [selectedPlugin, setSelectedPlugin] = createSignal<MarketplacePlugin | null>(null)
  const [installing, setInstalling] = createSignal<string | null>(null)

  // 构建筛选条件
  const buildFilter = (): MarketplaceFilter => ({
    category: selectedCategory(),
    search: searchQuery() || undefined,
    sort: sortBy(),
    page: 1,
    pageSize: 20,
  })

  // 获取市场插件
  const [marketData, { refetch }] = createResource(
    () => [searchQuery(), selectedCategory(), sortBy()],
    async () => {
      try {
        return await invoke<MarketplaceResponse>('search_marketplace', { 
          filter: buildFilter() 
        })
      } catch (e) {
        console.error('Failed to search marketplace:', e)
        return { plugins: [], total: 0, page: 1, pageSize: 20 }
      }
    }
  )

  // 获取推荐插件
  const [featured] = createResource(async () => {
    try {
      return await invoke<MarketplacePlugin[]>('get_featured_plugins')
    } catch (e) {
      console.error('Failed to get featured plugins:', e)
      return []
    }
  })

  // 分类列表
  const categories: { id: PluginCategory; name: string; icon: string }[] = [
    { id: 'search', name: '搜索提供', icon: '🔍' },
    { id: 'action', name: '动作处理', icon: '⚡' },
    { id: 'workflow', name: '工作流', icon: '🔄' },
    { id: 'theme', name: '主题', icon: '🎨' },
    { id: 'integration', name: '集成', icon: '🔗' },
    { id: 'utility', name: '实用工具', icon: '🛠' },
  ]

  // 安装插件
  const installPlugin = async (plugin: MarketplacePlugin) => {
    // 如果有权限要求，显示权限对话框
    // 这里简化处理，直接安装
    setInstalling(plugin.metadata.id)
    try {
      await invoke('install_plugin', {
        pluginId: plugin.metadata.id,
        version: plugin.metadata.version,
        permissions: [], // 默认不授予权限
      })
      refetch()
    } catch (e) {
      console.error('Failed to install plugin:', e)
    } finally {
      setInstalling(null)
    }
  }

  return (
    <div class="flex h-full">
      {/* 侧边栏 - 分类 */}
      <div class="w-48 border-r border-gray-200 dark:border-gray-700 p-4">
        <h3 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase mb-3">
          分类
        </h3>
        <div class="space-y-1">
          <button
            class={`w-full text-left px-3 py-2 rounded-lg text-sm transition-colors ${
              !selectedCategory()
                ? 'bg-blue-50 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400'
                : 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'
            }`}
            onClick={() => setSelectedCategory(undefined)}
          >
            全部
          </button>
          <For each={categories}>
            {(cat) => (
              <button
                class={`w-full text-left px-3 py-2 rounded-lg text-sm transition-colors flex items-center gap-2 ${
                  selectedCategory() === cat.id
                    ? 'bg-blue-50 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400'
                    : 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800'
                }`}
                onClick={() => setSelectedCategory(cat.id)}
              >
                <span>{cat.icon}</span>
                <span>{cat.name}</span>
              </button>
            )}
          </For>
        </div>
      </div>

      {/* 主内容区域 */}
      <div class="flex-1 overflow-hidden flex flex-col">
        <Show
          when={!selectedPlugin()}
          fallback={
            <PluginDetail 
              plugin={selectedPlugin()!} 
              onBack={() => setSelectedPlugin(null)}
              onInstall={installPlugin}
              installing={installing()}
            />
          }
        >
          {/* 搜索和筛选 */}
          <div class="p-4 border-b border-gray-200 dark:border-gray-700">
            <div class="flex items-center gap-4">
              {/* 搜索框 */}
              <div class="flex-1 relative">
                <Search class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-gray-400" />
                <input
                  type="text"
                  placeholder="搜索插件..."
                  value={searchQuery()}
                  onInput={(e) => setSearchQuery(e.currentTarget.value)}
                  class="w-full pl-10 pr-4 py-2 rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              {/* 排序 */}
              <select
                value={sortBy()}
                onChange={(e) => setSortBy(e.currentTarget.value as any)}
                class="px-4 py-2 rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="popular">最热门</option>
                <option value="newest">最新</option>
                <option value="updated">最近更新</option>
                <option value="rating">评分最高</option>
              </select>

              {/* 刷新 */}
              <button
                class="p-2 rounded-lg border border-gray-200 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                onClick={() => refetch()}
              >
                <RefreshCw class={`h-4 w-4 ${marketData.loading ? 'animate-spin' : ''}`} />
              </button>
            </div>
          </div>

          {/* 推荐插件（仅在未搜索时显示） */}
          <Show when={!searchQuery() && !selectedCategory() && featured()?.length}>
            <div class="p-4 border-b border-gray-200 dark:border-gray-700">
              <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3">
                ✨ 推荐插件
              </h3>
              <div class="grid grid-cols-3 gap-3">
                <For each={featured()?.slice(0, 3)}>
                  {(plugin) => (
                    <div
                      class="p-3 bg-gradient-to-br from-blue-50 to-purple-50 dark:from-blue-900/20 dark:to-purple-900/20 rounded-lg cursor-pointer hover:shadow-md transition-shadow"
                      onClick={() => setSelectedPlugin(plugin)}
                    >
                      <div class="flex items-center gap-2 mb-2">
                        <span class="text-xl">{plugin.metadata.icon || '📦'}</span>
                        <span class="font-medium text-gray-900 dark:text-white truncate">
                          {plugin.metadata.name}
                        </span>
                      </div>
                      <p class="text-xs text-gray-500 dark:text-gray-400 line-clamp-2">
                        {plugin.metadata.description}
                      </p>
                    </div>
                  )}
                </For>
              </div>
            </div>
          </Show>

          {/* 插件列表 */}
          <div class="flex-1 overflow-y-auto p-4">
            <Show
              when={!marketData.loading && marketData()?.plugins.length! > 0}
              fallback={
                <div class="h-full flex items-center justify-center">
                  <Show
                    when={!marketData.loading}
                    fallback={
                      <div class="flex items-center gap-2 text-gray-500">
                        <RefreshCw class="h-5 w-5 animate-spin" />
                        <span>加载中...</span>
                      </div>
                    }
                  >
                    <div class="text-center">
                      <Package class="h-12 w-12 mx-auto text-gray-300 dark:text-gray-600 mb-3" />
                      <p class="text-gray-500 dark:text-gray-400">
                        没有找到匹配的插件
                      </p>
                    </div>
                  </Show>
                </div>
              }
            >
              <div class="grid grid-cols-2 gap-4">
                <For each={marketData()?.plugins}>
                  {(plugin) => (
                    <PluginCard
                      plugin={plugin}
                      onClick={() => setSelectedPlugin(plugin)}
                      onInstall={() => installPlugin(plugin)}
                      installing={installing() === plugin.metadata.id}
                    />
                  )}
                </For>
              </div>

              {/* 分页信息 */}
              <div class="mt-4 text-center text-sm text-gray-500 dark:text-gray-400">
                共 {marketData()?.total} 个插件
              </div>
            </Show>
          </div>
        </Show>
      </div>
    </div>
  )
}

/**
 * 插件卡片组件
 */
const PluginCard: Component<{
  plugin: MarketplacePlugin
  onClick: () => void
  onInstall: () => void
  installing: boolean
}> = (props) => {
  const formatDownloads = (count: number) => {
    if (count >= 10000) return `${(count / 10000).toFixed(1)}万`
    if (count >= 1000) return `${(count / 1000).toFixed(1)}k`
    return count.toString()
  }

  return (
    <div
      class="p-4 bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 hover:shadow-lg transition-shadow cursor-pointer"
      onClick={props.onClick}
    >
      <div class="flex items-start gap-3">
        <div class="text-3xl flex-shrink-0">
          {props.plugin.metadata.icon || '📦'}
        </div>
        <div class="flex-1 min-w-0">
          <div class="flex items-center gap-2">
            <h4 class="font-semibold text-gray-900 dark:text-white truncate">
              {props.plugin.metadata.name}
            </h4>
            {props.plugin.installed && (
              <span class="px-1.5 py-0.5 text-xs bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400 rounded">
                已安装
              </span>
            )}
            {props.plugin.hasUpdate && (
              <span class="px-1.5 py-0.5 text-xs bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400 rounded">
                有更新
              </span>
            )}
          </div>
          <p class="text-sm text-gray-500 dark:text-gray-400 mt-1 line-clamp-2">
            {props.plugin.metadata.description}
          </p>
        </div>
      </div>

      <div class="flex items-center justify-between mt-4">
        <div class="flex items-center gap-3 text-xs text-gray-500 dark:text-gray-400">
          <span class="flex items-center gap-1">
            <Star class="h-3 w-3 fill-yellow-400 text-yellow-400" />
            {props.plugin.rating.toFixed(1)}
          </span>
          <span class="flex items-center gap-1">
            <Download class="h-3 w-3" />
            {formatDownloads(props.plugin.downloads)}
          </span>
        </div>

        <button
          class={`px-3 py-1.5 rounded-lg text-sm font-medium transition-colors ${
            props.plugin.installed
              ? 'bg-gray-100 text-gray-500 dark:bg-gray-700 dark:text-gray-400 cursor-default'
              : 'bg-blue-500 text-white hover:bg-blue-600'
          }`}
          onClick={(e) => {
            e.stopPropagation()
            if (!props.plugin.installed) {
              props.onInstall()
            }
          }}
          disabled={props.plugin.installed || props.installing}
        >
          {props.installing ? (
            <RefreshCw class="h-4 w-4 animate-spin" />
          ) : props.plugin.installed ? (
            <Check class="h-4 w-4" />
          ) : (
            '安装'
          )}
        </button>
      </div>
    </div>
  )
}

/**
 * 插件详情组件
 */
const PluginDetail: Component<{
  plugin: MarketplacePlugin
  onBack: () => void
  onInstall: (plugin: MarketplacePlugin) => void
  installing: string | null
}> = (props) => {
  return (
    <div class="h-full flex flex-col">
      {/* 头部 */}
      <div class="p-4 border-b border-gray-200 dark:border-gray-700">
        <button
          class="flex items-center gap-1 text-sm text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300 mb-4"
          onClick={props.onBack}
        >
          <ChevronLeft class="h-4 w-4" />
          返回市场
        </button>

        <div class="flex items-start gap-4">
          <div class="text-5xl">
            {props.plugin.metadata.icon || '📦'}
          </div>
          <div class="flex-1">
            <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
              {props.plugin.metadata.name}
            </h2>
            <p class="text-gray-500 dark:text-gray-400 mt-1">
              {props.plugin.metadata.description}
            </p>
            <div class="flex items-center gap-4 mt-3">
              <span class="flex items-center gap-1 text-sm">
                <Star class="h-4 w-4 fill-yellow-400 text-yellow-400" />
                {props.plugin.rating.toFixed(1)} ({props.plugin.ratingCount})
              </span>
              <span class="flex items-center gap-1 text-sm text-gray-500 dark:text-gray-400">
                <Download class="h-4 w-4" />
                {props.plugin.downloads.toLocaleString()} 次下载
              </span>
              <span class="text-sm text-gray-500 dark:text-gray-400">
                v{props.plugin.metadata.version}
              </span>
              <span class="text-sm text-gray-500 dark:text-gray-400">
                by {props.plugin.metadata.author}
              </span>
            </div>
          </div>

          <button
            class={`px-6 py-2 rounded-lg font-medium transition-colors ${
              props.plugin.installed
                ? 'bg-gray-100 text-gray-500 dark:bg-gray-700 dark:text-gray-400'
                : 'bg-blue-500 text-white hover:bg-blue-600'
            }`}
            onClick={() => props.onInstall(props.plugin)}
            disabled={props.plugin.installed || props.installing === props.plugin.metadata.id}
          >
            {props.installing === props.plugin.metadata.id ? (
              <span class="flex items-center gap-2">
                <RefreshCw class="h-4 w-4 animate-spin" />
                安装中...
              </span>
            ) : props.plugin.installed ? (
              <span class="flex items-center gap-2">
                <Check class="h-4 w-4" />
                已安装
              </span>
            ) : (
              '安装插件'
            )}
          </button>
        </div>
      </div>

      {/* 内容 */}
      <div class="flex-1 overflow-y-auto p-4">
        <div class="grid grid-cols-3 gap-6">
          {/* 主要内容 */}
          <div class="col-span-2 space-y-6">
            {/* README */}
            <Show when={props.plugin.readme}>
              <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
                  说明
                </h3>
                <div class="prose dark:prose-invert prose-sm max-w-none">
                  <pre class="whitespace-pre-wrap text-sm text-gray-600 dark:text-gray-400">
                    {props.plugin.readme}
                  </pre>
                </div>
              </div>
            </Show>

            {/* 更新日志 */}
            <Show when={props.plugin.changelog}>
              <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
                  更新日志
                </h3>
                <pre class="whitespace-pre-wrap text-sm text-gray-600 dark:text-gray-400">
                  {props.plugin.changelog}
                </pre>
              </div>
            </Show>
          </div>

          {/* 侧边栏信息 */}
          <div class="space-y-4">
            {/* 信息卡片 */}
            <div class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700">
              <h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3">
                信息
              </h4>
              <dl class="space-y-2 text-sm">
                <div class="flex justify-between">
                  <dt class="text-gray-500 dark:text-gray-400">版本</dt>
                  <dd class="text-gray-900 dark:text-white">{props.plugin.metadata.version}</dd>
                </div>
                <div class="flex justify-between">
                  <dt class="text-gray-500 dark:text-gray-400">作者</dt>
                  <dd class="text-gray-900 dark:text-white">{props.plugin.metadata.author}</dd>
                </div>
                {props.plugin.metadata.license && (
                  <div class="flex justify-between">
                    <dt class="text-gray-500 dark:text-gray-400">许可证</dt>
                    <dd class="text-gray-900 dark:text-white">{props.plugin.metadata.license}</dd>
                  </div>
                )}
                <div class="flex justify-between">
                  <dt class="text-gray-500 dark:text-gray-400">更新时间</dt>
                  <dd class="text-gray-900 dark:text-white">
                    {new Date(props.plugin.lastUpdated).toLocaleDateString()}
                  </dd>
                </div>
              </dl>
            </div>

            {/* 链接 */}
            <Show when={props.plugin.metadata.homepage || props.plugin.metadata.repository}>
              <div class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700">
                <h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3">
                  链接
                </h4>
                <div class="space-y-2">
                  <Show when={props.plugin.metadata.homepage}>
                    <a
                      href={props.plugin.metadata.homepage}
                      target="_blank"
                      rel="noopener noreferrer"
                      class="flex items-center gap-2 text-sm text-blue-500 hover:text-blue-600"
                    >
                      <ExternalLink class="h-4 w-4" />
                      主页
                    </a>
                  </Show>
                  <Show when={props.plugin.metadata.repository}>
                    <a
                      href={props.plugin.metadata.repository}
                      target="_blank"
                      rel="noopener noreferrer"
                      class="flex items-center gap-2 text-sm text-blue-500 hover:text-blue-600"
                    >
                      <ExternalLink class="h-4 w-4" />
                      源代码
                    </a>
                  </Show>
                </div>
              </div>
            </Show>

            {/* 关键词 */}
            <Show when={props.plugin.metadata.keywords?.length}>
              <div class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700">
                <h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3">
                  关键词
                </h4>
                <div class="flex flex-wrap gap-2">
                  <For each={props.plugin.metadata.keywords}>
                    {(keyword) => (
                      <span class="px-2 py-1 text-xs bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 rounded">
                        {keyword}
                      </span>
                    )}
                  </For>
                </div>
              </div>
            </Show>
          </div>
        </div>
      </div>
    </div>
  )
}
