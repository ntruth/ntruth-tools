import { Component, createSignal, createResource, For, Show } from 'solid-js'
import { invoke } from '@tauri-apps/api/core'
import { 
  Package, 
  Power, 
  PowerOff, 
  Trash2, 
  RefreshCw, 
  Settings, 
  Shield, 
  ExternalLink,
  AlertCircle,
  Check
} from 'lucide-solid'
import type { 
  InstalledPlugin, 
  PluginPermission, 
  PluginStatus,
  PluginUpdateInfo 
} from '../../types/plugin'

/**
 * 已安装插件管理页面
 */
export const Plugins: Component = () => {
  const [selectedPlugin, setSelectedPlugin] = createSignal<InstalledPlugin | null>(null)
  const [showPermissions, setShowPermissions] = createSignal(false)

  // 获取已安装的插件
  const [plugins, { refetch }] = createResource(async () => {
    try {
      return await invoke<InstalledPlugin[]>('get_installed_plugins')
    } catch (e) {
      console.error('Failed to load plugins:', e)
      return []
    }
  })

  // 检查更新
  const [updates] = createResource(async () => {
    try {
      return await invoke<PluginUpdateInfo[]>('check_plugin_updates')
    } catch (e) {
      console.error('Failed to check updates:', e)
      return []
    }
  })

  // 启用/禁用插件
  const togglePlugin = async (plugin: InstalledPlugin) => {
    try {
      if (plugin.status === 'enabled') {
        await invoke('disable_plugin', { pluginId: plugin.metadata.id })
      } else {
        await invoke('enable_plugin', { pluginId: plugin.metadata.id })
      }
      refetch()
    } catch (e) {
      console.error('Failed to toggle plugin:', e)
    }
  }

  // 卸载插件
  const uninstallPlugin = async (plugin: InstalledPlugin) => {
    if (!confirm(`确定要卸载插件 "${plugin.metadata.name}" 吗？`)) {
      return
    }
    try {
      await invoke('uninstall_plugin', { pluginId: plugin.metadata.id })
      setSelectedPlugin(null)
      refetch()
    } catch (e) {
      console.error('Failed to uninstall plugin:', e)
    }
  }

  // 更新插件
  const updatePlugin = async (plugin: InstalledPlugin) => {
    try {
      await invoke('update_plugin', { pluginId: plugin.metadata.id })
      refetch()
    } catch (e) {
      console.error('Failed to update plugin:', e)
    }
  }

  // 检查是否有更新
  const hasUpdate = (pluginId: string) => {
    return updates()?.some(u => u.pluginId === pluginId) ?? false
  }

  // 获取状态颜色
  const getStatusColor = (status: PluginStatus) => {
    switch (status) {
      case 'enabled': return 'text-green-500'
      case 'disabled': return 'text-gray-400'
      case 'error': return 'text-red-500'
      case 'updating': return 'text-yellow-500'
      default: return 'text-gray-500'
    }
  }

  // 获取状态文本
  const getStatusText = (status: PluginStatus) => {
    switch (status) {
      case 'enabled': return '已启用'
      case 'disabled': return '已禁用'
      case 'installed': return '已安装'
      case 'error': return '错误'
      case 'updating': return '更新中'
      default: return status
    }
  }

  // 获取权限描述
  const getPermissionLabel = (permission: PluginPermission) => {
    const labels: Record<PluginPermission, string> = {
      'clipboard:read': '读取剪贴板',
      'clipboard:write': '写入剪贴板',
      'fs:read': '读取文件',
      'fs:write': '写入文件',
      'network': '网络访问',
      'shell': 'Shell 命令',
      'notification': '系统通知',
      'system': '系统信息',
    }
    return labels[permission] || permission
  }

  // 获取分类图标
  const getCategoryIcon = (category: string) => {
    const icons: Record<string, string> = {
      'search': '🔍',
      'action': '⚡',
      'workflow': '🔄',
      'theme': '🎨',
      'integration': '🔗',
      'utility': '🛠',
      'other': '📦',
    }
    return icons[category] || '📦'
  }

  return (
    <div class="flex h-full">
      {/* 插件列表 */}
      <div class="w-80 border-r border-gray-200 dark:border-gray-700 overflow-y-auto">
        <div class="p-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
            已安装插件
          </h3>
          <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
            {plugins()?.length ?? 0} 个插件
          </p>
        </div>

        <Show
          when={!plugins.loading && plugins()?.length! > 0}
          fallback={
            <div class="p-8 text-center">
              <Show
                when={!plugins.loading}
                fallback={
                  <div class="flex items-center justify-center gap-2 text-gray-500">
                    <RefreshCw class="h-4 w-4 animate-spin" />
                    <span>加载中...</span>
                  </div>
                }
              >
                <Package class="h-12 w-12 mx-auto text-gray-300 dark:text-gray-600 mb-3" />
                <p class="text-gray-500 dark:text-gray-400">
                  暂无已安装的插件
                </p>
                <p class="text-sm text-gray-400 dark:text-gray-500 mt-1">
                  前往插件市场探索更多插件
                </p>
              </Show>
            </div>
          }
        >
          <div class="divide-y divide-gray-200 dark:divide-gray-700">
            <For each={plugins()}>
              {(plugin) => (
                <div
                  class="p-4 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
                  classList={{
                    'bg-blue-50 dark:bg-blue-900/20': selectedPlugin()?.metadata.id === plugin.metadata.id,
                  }}
                  onClick={() => setSelectedPlugin(plugin)}
                >
                  <div class="flex items-start gap-3">
                    <div class="text-2xl flex-shrink-0">
                      {plugin.metadata.icon || getCategoryIcon(plugin.metadata.category)}
                    </div>
                    <div class="min-w-0 flex-1">
                      <div class="flex items-center gap-2">
                        <span class="font-medium text-gray-900 dark:text-white truncate">
                          {plugin.metadata.name}
                        </span>
                        {hasUpdate(plugin.metadata.id) && (
                          <span class="px-1.5 py-0.5 text-xs bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300 rounded">
                            有更新
                          </span>
                        )}
                      </div>
                      <p class="text-sm text-gray-500 dark:text-gray-400 truncate">
                        v{plugin.metadata.version} · {plugin.metadata.author}
                      </p>
                      <div class="flex items-center gap-2 mt-1">
                        <span class={`text-xs ${getStatusColor(plugin.status)}`}>
                          {getStatusText(plugin.status)}
                        </span>
                        {plugin.error && (
                          <AlertCircle class="h-3 w-3 text-red-500" />
                        )}
                      </div>
                    </div>
                  </div>
                </div>
              )}
            </For>
          </div>
        </Show>
      </div>

      {/* 插件详情 */}
      <div class="flex-1 overflow-y-auto">
        <Show
          when={selectedPlugin()}
          fallback={
            <div class="h-full flex items-center justify-center">
              <div class="text-center">
                <Package class="h-16 w-16 mx-auto text-gray-300 dark:text-gray-600 mb-4" />
                <p class="text-gray-500 dark:text-gray-400">
                  选择一个插件查看详情
                </p>
              </div>
            </div>
          }
        >
          {(plugin) => (
            <div class="p-6">
              {/* 头部 */}
              <div class="flex items-start gap-4 mb-6">
                <div class="text-4xl">
                  {plugin().metadata.icon || getCategoryIcon(plugin().metadata.category)}
                </div>
                <div class="flex-1">
                  <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
                    {plugin().metadata.name}
                  </h2>
                  <p class="text-gray-500 dark:text-gray-400 mt-1">
                    {plugin().metadata.description}
                  </p>
                  <div class="flex items-center gap-4 mt-2 text-sm text-gray-500 dark:text-gray-400">
                    <span>v{plugin().metadata.version}</span>
                    <span>·</span>
                    <span>{plugin().metadata.author}</span>
                    {plugin().metadata.license && (
                      <>
                        <span>·</span>
                        <span>{plugin().metadata.license}</span>
                      </>
                    )}
                  </div>
                </div>
              </div>

              {/* 操作按钮 */}
              <div class="flex items-center gap-3 mb-6 pb-6 border-b border-gray-200 dark:border-gray-700">
                <button
                  class={`flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors ${
                    plugin().status === 'enabled'
                      ? 'bg-gray-100 text-gray-700 hover:bg-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700'
                      : 'bg-green-500 text-white hover:bg-green-600'
                  }`}
                  onClick={() => togglePlugin(plugin())}
                >
                  {plugin().status === 'enabled' ? (
                    <>
                      <PowerOff class="h-4 w-4" />
                      禁用
                    </>
                  ) : (
                    <>
                      <Power class="h-4 w-4" />
                      启用
                    </>
                  )}
                </button>

                {hasUpdate(plugin().metadata.id) && (
                  <button
                    class="flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg font-medium hover:bg-blue-600 transition-colors"
                    onClick={() => updatePlugin(plugin())}
                  >
                    <RefreshCw class="h-4 w-4" />
                    更新
                  </button>
                )}

                <button
                  class="flex items-center gap-2 px-4 py-2 text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg font-medium transition-colors"
                  onClick={() => uninstallPlugin(plugin())}
                >
                  <Trash2 class="h-4 w-4" />
                  卸载
                </button>

                <div class="flex-1" />

                {plugin().metadata.homepage && (
                  <a
                    href={plugin().metadata.homepage}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center gap-1 text-sm text-blue-500 hover:text-blue-600"
                  >
                    <ExternalLink class="h-4 w-4" />
                    主页
                  </a>
                )}
              </div>

              {/* 错误信息 */}
              {plugin().error && (
                <div class="mb-6 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
                  <div class="flex items-start gap-3">
                    <AlertCircle class="h-5 w-5 text-red-500 flex-shrink-0 mt-0.5" />
                    <div>
                      <p class="font-medium text-red-700 dark:text-red-400">
                        插件错误
                      </p>
                      <p class="text-sm text-red-600 dark:text-red-300 mt-1">
                        {plugin().error}
                      </p>
                    </div>
                  </div>
                </div>
              )}

              {/* 权限 */}
              <div class="mb-6">
                <button
                  class="flex items-center gap-2 text-gray-700 dark:text-gray-300 font-medium"
                  onClick={() => setShowPermissions(!showPermissions())}
                >
                  <Shield class="h-5 w-5" />
                  权限 ({plugin().grantedPermissions.length}/{plugin().permissions.length})
                </button>

                <Show when={showPermissions()}>
                  <div class="mt-3 space-y-2 pl-7">
                    <For each={plugin().permissions}>
                      {(permission) => (
                        <div class="flex items-center gap-2">
                          <Show
                            when={plugin().grantedPermissions.includes(permission)}
                            fallback={
                              <div class="w-5 h-5 rounded border-2 border-gray-300 dark:border-gray-600" />
                            }
                          >
                            <div class="w-5 h-5 rounded bg-green-500 flex items-center justify-center">
                              <Check class="h-3 w-3 text-white" />
                            </div>
                          </Show>
                          <span class="text-sm text-gray-600 dark:text-gray-400">
                            {getPermissionLabel(permission)}
                          </span>
                        </div>
                      )}
                    </For>
                  </div>
                </Show>
              </div>

              {/* 配置（如果有） */}
              {Object.keys(plugin().config || {}).length > 0 && (
                <div class="mb-6">
                  <button
                    class="flex items-center gap-2 text-gray-700 dark:text-gray-300 font-medium"
                  >
                    <Settings class="h-5 w-5" />
                    插件配置
                  </button>
                  <div class="mt-3 pl-7">
                    <pre class="text-sm bg-gray-100 dark:bg-gray-800 p-4 rounded-lg overflow-x-auto">
                      {JSON.stringify(plugin().config, null, 2)}
                    </pre>
                  </div>
                </div>
              )}

              {/* 元信息 */}
              <div class="text-sm text-gray-500 dark:text-gray-400 space-y-1">
                <p>
                  安装时间：{new Date(plugin().installedAt).toLocaleString()}
                </p>
                <p>
                  更新时间：{new Date(plugin().updatedAt).toLocaleString()}
                </p>
                {plugin().metadata.minAppVersion && (
                  <p>
                    最低版本要求：v{plugin().metadata.minAppVersion}
                  </p>
                )}
              </div>
            </div>
          )}
        </Show>
      </div>
    </div>
  )
}
