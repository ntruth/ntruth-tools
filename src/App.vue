<template>
  <div class="prefs-app">
    <ClipboardPanel />
    <div class="prefs-layout">
      <aside class="prefs-sidebar">
        <div class="sidebar-brand">
          <span class="brand-emblem">🎩</span>
          <div class="brand-copy">
            <p>UniTools</p>
            <h1>偏好设置</h1>
          </div>
        </div>
        <nav class="sidebar-groups">
          <section v-for="group in sectionGroups" :key="group.title" class="sidebar-group">
            <p class="sidebar-group-title">{{ group.title }}</p>
            <button
              v-for="item in group.items"
              :key="item.value"
              type="button"
              :class="['sidebar-item', { active: section === item.value }]"
              @click="section = item.value"
            >
              <span class="item-icon">{{ item.icon }}</span>
              <span class="item-label">{{ item.label }}</span>
            </button>
          </section>
        </nav>
        <footer class="sidebar-footer">
          <p>提示：按下 ⌘ + , 可随时打开偏好设置。</p>
        </footer>
      </aside>

      <div class="prefs-main">
        <header class="main-toolbar">
          <div class="toolbar-heading">
            <span class="toolbar-icon">{{ currentSection?.icon }}</span>
            <div class="toolbar-copy">
              <h2>{{ currentSection?.label }}</h2>
              <p>{{ currentSection?.description }}</p>
            </div>
          </div>
          <label class="toolbar-search" aria-label="搜索偏好">
            <input type="search" placeholder="搜索偏好…" />
          </label>
        </header>

        <main class="panel-area">
          <section v-if="section === 'general'" class="panel general-panel">
            <div class="panel-grid">
              <article class="preference-card span-2">
                <header>
                  <h3>启动与菜单栏</h3>
                  <p>让 UniTools 与系统保持同步，开机即用。</p>
                </header>
                <div class="card-table">
                  <div class="table-row">
                    <div class="table-copy">
                      <h4>开机自动启动</h4>
                      <p>登录后立即加载 UniTools，后台待命。</p>
                    </div>
                    <label class="table-control toggle">
                      <input
                        type="checkbox"
                        :checked="settings.state.general.launchAtLogin"
                        @change="onToggleGeneral('launchAtLogin', $event)"
                      />
                      <span>启用</span>
                    </label>
                  </div>
                  <div class="table-row">
                    <div class="table-copy">
                      <h4>自动唤起搜索</h4>
                      <p>应用启动时立即弹出搜索框，随时准备好。</p>
                    </div>
                    <label class="table-control toggle">
                      <input
                        type="checkbox"
                        :checked="settings.state.general.autoLaunchSearch"
                        @change="onToggleGeneral('autoLaunchSearch', $event)"
                      />
                      <span>启用</span>
                    </label>
                  </div>
                  <div class="table-row">
                    <div class="table-copy">
                      <h4>菜单栏图标</h4>
                      <p>在菜单栏显示图标，快速进入启动器与剪贴板历史。</p>
                    </div>
                    <label class="table-control toggle">
                      <input
                        type="checkbox"
                        :checked="settings.state.general.showTrayIcon"
                        @change="onToggleGeneral('showTrayIcon', $event)"
                      />
                      <span>显示</span>
                    </label>
                  </div>
                </div>
              </article>

              <article class="preference-card">
                <header>
                  <h3>Alfred Hotkey</h3>
                  <p>按下快捷键立即呼出 UniTools。</p>
                </header>
                <div class="hotkey-display">⌥ 空格</div>
                <p class="hotkey-hint">点击字段以重新录制快捷键（即将推出）。</p>
              </article>

              <article class="preference-card">
                <header>
                  <h3>权限</h3>
                  <p>授予辅助功能权限以捕获键盘输入、显示剪贴板历史。</p>
                </header>
                <button type="button" class="permission-button">请求权限…</button>
              </article>

              <article class="preference-card span-2">
                <header>
                  <h3>Web 备用搜索</h3>
                  <p>按照顺序使用备用搜索引擎，使用 %s 填充关键字。</p>
                </header>
                <textarea
                  rows="5"
                  :value="settings.state.webFallbacks.join('\n')"
                  @change="onFallbackChange"
                ></textarea>
              </article>

              <article class="preference-card span-2">
                <header>
                  <h3>主题</h3>
                  <p>切换浅色或深色外观，匹配 Alfred 风格。</p>
                </header>
                <ThemeToggle />
              </article>
            </div>
          </section>

          <section v-else-if="section === 'features'" class="panel features-panel">
            <div class="feature-shell">
              <aside class="feature-tabs">
                <button
                  v-for="item in featureTabs"
                  :key="item.key"
                  type="button"
                  :class="['feature-tab', { active: activeFeature === item.key }]"
                  @click="activeFeature = item.key"
                >
                  {{ item.label }}
                </button>
              </aside>
              <div class="feature-pane">
                <header class="feature-header">
                  <h3>{{ featureDescriptions[activeFeature].title }}</h3>
                  <p>{{ featureDescriptions[activeFeature].description }}</p>
                </header>

                <label class="feature-toggle">
                  <input
                    type="checkbox"
                    :checked="settings.state.features[activeFeature]"
                    @change="onToggleFeature(activeFeature, $event)"
                  />
                  <span>{{ featureDescriptions[activeFeature].toggleLabel }}</span>
                </label>

                <div v-if="activeFeature === 'defaultResults'" class="default-results">
                  <div class="results-types">
                    <h4>结果类型</h4>
                    <ul>
                      <li v-for="item in defaultResultTypes" :key="item">{{ item }}</li>
                    </ul>
                  </div>
                  <div class="results-scope">
                    <h4>索引位置</h4>
                    <ul>
                      <li v-for="path in defaultResultLocations" :key="path">{{ path }}</li>
                    </ul>
                  </div>
                </div>

                <div v-else-if="activeFeature === 'webSearch'" class="web-search">
                  <table>
                    <thead>
                      <tr>
                        <th>关键字</th>
                        <th>描述</th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr v-for="engine in webSearchEngines" :key="engine.keyword">
                        <td>{{ engine.keyword }}</td>
                        <td>{{ engine.description }}</td>
                      </tr>
                    </tbody>
                  </table>
                </div>

                <p v-else class="feature-placeholder">
                  {{ featureDescriptions[activeFeature].placeholder }}
                </p>
              </div>
            </div>
          </section>

          <section v-else class="panel placeholder-panel">
            <div class="placeholder-card">
              <h3>{{ currentSection?.placeholder?.title ?? currentSection?.label }}</h3>
              <p>{{ currentSection?.placeholder?.description ?? '该区域的高级设置正在构建中，敬请期待。' }}</p>
            </div>
          </section>
        </main>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useSettingsStore } from "@/stores/settings";
import ThemeToggle from "@/components/ThemeToggle.vue";
import ClipboardPanel from "@/features/clipboard/ClipboardPanel.vue";

const settings = useSettingsStore();

const sectionGroups = [
  {
    title: "GENERAL",
    items: [
      {
        value: "general",
        label: "通用",
        icon: "⚙️",
        description: "调整启动器行为、快捷键与系统整合。"
      },
      {
        value: "features",
        label: "功能",
        icon: "✨",
        description: "启用默认结果、文件搜索、Web 搜索与剪贴板模块。"
      }
    ]
  },
  {
    title: "WORKFLOWS",
    items: [
      {
        value: "workflows",
        label: "工作流",
        icon: "🧩",
        description: "构建自动化与扩展动作。",
        placeholder: {
          title: "工作流",
          description: "自动化任务与组合动作的集中管理。"
        }
      }
    ]
  },
  {
    title: "APPEARANCE",
    items: [
      {
        value: "appearance",
        label: "外观",
        icon: "🎨",
        description: "自定义主题、字体与窗口展示效果。",
        placeholder: {
          title: "外观",
          description: "自定义主题、字体与窗口展示效果。"
        }
      },
      {
        value: "remote",
        label: "远程",
        icon: "📱",
        description: "使用移动端遥控扩展工作流程。",
        placeholder: {
          title: "远程",
          description: "通过移动端 Alfred Remote 控制桌面体验。"
        }
      },
      {
        value: "advanced",
        label: "高级",
        icon: "🛠",
        description: "调节索引、兼容性与开发者相关设置。",
        placeholder: {
          title: "高级",
          description: "调节索引、兼容性与开发者相关设置。"
        }
      },
      {
        value: "powerpack",
        label: "Powerpack",
        icon: "⚡",
        description: "管理授权、同步与高级功能解锁。",
        placeholder: {
          title: "Powerpack",
          description: "管理授权、同步与高级功能解锁。"
        }
      }
    ]
  }
] as const;

const allSections = sectionGroups.flatMap((group) => group.items);

type SectionValue = (typeof allSections)[number]["value"];

const section = ref<SectionValue>("general");

const currentSection = computed(() => allSections.find((item) => item.value === section.value));

const featureTabs = [
  { key: "defaultResults", label: "默认结果" },
  { key: "fileSearch", label: "文件搜索" },
  { key: "universalActions", label: "Universal Actions" },
  { key: "webSearch", label: "Web 搜索" },
  { key: "webBookmarks", label: "Web 书签" },
  { key: "clipboardHistory", label: "剪贴板历史" },
  { key: "snippets", label: "Snippets" },
  { key: "calculator", label: "计算器" }
] as const;

type FeatureKey = (typeof featureTabs)[number]["key"];

const activeFeature = ref<FeatureKey>("defaultResults");

const featureDescriptions: Record<FeatureKey, { title: string; description: string; toggleLabel: string; placeholder: string }>
  = {
    defaultResults: {
      title: "默认结果",
      description: "决定键入时展示的应用、文件与内部命令。",
      toggleLabel: "启用默认结果",
      placeholder: "勾选类型决定默认结果中出现的项目。"
    },
    fileSearch: {
      title: "文件搜索",
      description: "索引常用目录以便快速查找文档与文件夹。",
      toggleLabel: "启用文件搜索",
      placeholder: "即将提供更细粒度的索引范围设置。"
    },
    universalActions: {
      title: "Universal Actions",
      description: "对选中文本或文件执行上下文动作。",
      toggleLabel: "启用 Universal Actions",
      placeholder: "动作编辑器准备中，稍后即可自定义。"
    },
    webSearch: {
      title: "Web 搜索",
      description: "未命中的查询将自动跳转至配置的搜索引擎。",
      toggleLabel: "启用 Web 搜索",
      placeholder: "自定义关键字，即可像 Alfred 一样快速打开网页。"
    },
    webBookmarks: {
      title: "Web 书签",
      description: "索引浏览器书签并在搜索中显示。",
      toggleLabel: "启用书签索引",
      placeholder: "支持 Chrome 与 Safari，后续加入更多浏览器。"
    },
    clipboardHistory: {
      title: "剪贴板历史",
      description: "记录文本剪贴板内容，通过搜索立即调用。",
      toggleLabel: "启用剪贴板历史",
      placeholder: "输入 clip 即可在启动器中浏览历史记录。"
    },
    snippets: {
      title: "Snippets",
      description: "保存常用文本片段并一键粘贴。",
      toggleLabel: "启用 Snippets",
      placeholder: "即将支持片段集合与同步。"
    },
    calculator: {
      title: "计算器",
      description: "在搜索框内直接进行四则运算与单位换算。",
      toggleLabel: "启用计算器",
      placeholder: "输入表达式即可即时看到计算结果。"
    }
  };

const defaultResultTypes = ["应用程序", "联系人", "文件夹", "文档", "文本文件", "脚本"] as const;

const defaultResultLocations = [
  "~/Applications",
  "/Applications",
  "~/Library/Application Support",
  "~/Library/Caches",
  "~/Documents"
] as const;

const webSearchEngines = [
  { keyword: "gg", description: "使用 Google 搜索“{query}”" },
  { keyword: "bd", description: "使用 百度 搜索“{query}”" },
  { keyword: "wk", description: "搜索 维基百科“{query}”" },
  { keyword: "gh", description: "搜索 GitHub “{query}”" },
  { keyword: "amz", description: "搜索 亚马逊 “{query}”" }
] as const;

const onToggleGeneral = (key: Parameters<typeof settings.toggleGeneral>[0], event: Event) => {
  const input = event.target as HTMLInputElement;
  settings.toggleGeneral(key, input.checked);
};

const onToggleFeature = (key: FeatureKey, event: Event) => {
  const input = event.target as HTMLInputElement;
  settings.toggleFeature(key, input.checked);
};

const onFallbackChange = (event: Event) => {
  const value = (event.target as HTMLTextAreaElement).value
    .split("\n")
    .map((item) => item.trim())
    .filter(Boolean);
  settings.setFallbacks(value);
};
</script>

<style scoped>
.prefs-app {
  min-height: 100vh;
  background:
    radial-gradient(circle at 20% 20%, rgba(142, 151, 255, 0.12), transparent 45%),
    radial-gradient(circle at 85% 25%, rgba(171, 120, 255, 0.16), transparent 50%),
    repeating-linear-gradient(45deg, rgba(255, 255, 255, 0.12), rgba(255, 255, 255, 0.12) 12px, transparent 12px, transparent 24px),
    #e7e9f4;
  padding: 3rem 0;
  display: flex;
  justify-content: center;
  align-items: flex-start;
}

.prefs-layout {
  width: min(1140px, 100% - 3rem);
  background: #f0f2f7;
  border-radius: 28px;
  box-shadow: 0 42px 110px rgba(30, 34, 58, 0.28);
  display: grid;
  grid-template-columns: 240px 1fr;
  overflow: hidden;
  border: 1px solid rgba(148, 163, 184, 0.25);
}

.prefs-sidebar {
  background: linear-gradient(190deg, #43126f, #4a155f 45%, #2f0b5a 100%);
  color: rgba(255, 255, 255, 0.9);
  display: flex;
  flex-direction: column;
  padding: 2.5rem 1.75rem 1.75rem;
  gap: 2rem;
}

.sidebar-brand {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.brand-emblem {
  width: 48px;
  height: 48px;
  border-radius: 14px;
  display: grid;
  place-items: center;
  font-size: 1.8rem;
  background: rgba(255, 255, 255, 0.12);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.25), inset 0 -1px 0 rgba(0, 0, 0, 0.35);
}

.brand-copy p {
  margin: 0;
  font-size: 0.85rem;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  color: rgba(226, 232, 240, 0.7);
}

.brand-copy h1 {
  margin: 0.25rem 0 0;
  font-size: 1.45rem;
  letter-spacing: 0.02em;
}

.sidebar-groups {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.sidebar-group-title {
  margin: 0 0 0.75rem;
  font-size: 0.75rem;
  letter-spacing: 0.2em;
  text-transform: uppercase;
  color: rgba(226, 232, 240, 0.6);
}

.sidebar-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 0.8rem;
  border: none;
  border-radius: 0.9rem;
  padding: 0.6rem 0.75rem;
  background: transparent;
  color: inherit;
  font-size: 0.98rem;
  cursor: pointer;
  transition: background 0.15s ease, color 0.15s ease;
}

.sidebar-item:hover {
  background: rgba(255, 255, 255, 0.12);
}

.sidebar-item.active {
  background: rgba(255, 255, 255, 0.18);
  color: #fff;
}

.item-icon {
  width: 28px;
  height: 28px;
  border-radius: 8px;
  display: grid;
  place-items: center;
  background: rgba(255, 255, 255, 0.16);
  font-size: 1.1rem;
}

.item-label {
  flex: 1;
  text-align: left;
}

.sidebar-footer {
  margin-top: auto;
  font-size: 0.75rem;
  color: rgba(226, 232, 240, 0.6);
  line-height: 1.6;
}

.prefs-main {
  background: linear-gradient(180deg, #f7f8fb, #e3e6f0);
  display: flex;
  flex-direction: column;
}

.main-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.75rem 2.5rem 1.25rem;
  border-bottom: 1px solid rgba(148, 163, 184, 0.3);
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.92), rgba(233, 236, 245, 0.85));
  backdrop-filter: blur(14px);
}

.toolbar-heading {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.toolbar-icon {
  width: 52px;
  height: 52px;
  border-radius: 15px;
  display: grid;
  place-items: center;
  font-size: 1.6rem;
  background: linear-gradient(180deg, rgba(71, 55, 120, 0.18), rgba(60, 41, 104, 0.32));
  color: #3c2d69;
}

.toolbar-copy h2 {
  margin: 0;
  font-size: 1.6rem;
  letter-spacing: 0.01em;
}

.toolbar-copy p {
  margin: 0.35rem 0 0;
  color: rgba(71, 85, 105, 0.7);
  font-size: 0.9rem;
}

.toolbar-search {
  display: inline-flex;
  align-items: center;
  padding: 0.35rem 0.6rem;
  border-radius: 10px;
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.9), rgba(206, 211, 222, 0.65));
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.6), inset 0 -1px 0 rgba(148, 163, 184, 0.45);
}

.toolbar-search input {
  border: none;
  background: transparent;
  font-size: 0.95rem;
  padding: 0.35rem 0.6rem;
  width: 220px;
}

.toolbar-search input:focus-visible {
  outline: none;
}

.panel-area {
  padding: 2.5rem;
  flex: 1;
  overflow-y: auto;
}

.panel {
  background: rgba(255, 255, 255, 0.85);
  border-radius: 18px;
  border: 1px solid rgba(148, 163, 184, 0.25);
  box-shadow: 0 18px 45px rgba(15, 23, 42, 0.12);
  padding: 2rem;
}

.panel-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 1.5rem;
}

.preference-card {
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.92), rgba(239, 243, 250, 0.92));
  border-radius: 16px;
  border: 1px solid rgba(148, 163, 184, 0.25);
  padding: 1.5rem;
  display: grid;
  gap: 1.1rem;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.6);
}

.preference-card header h3 {
  margin: 0;
  font-size: 1.2rem;
}

.preference-card header p {
  margin: 0.4rem 0 0;
  color: rgba(71, 85, 105, 0.72);
}

.card-table {
  display: grid;
  gap: 1.1rem;
}

.table-row {
  display: flex;
  justify-content: space-between;
  gap: 1.5rem;
  align-items: center;
}

.table-copy h4 {
  margin: 0;
  font-size: 1rem;
}

.table-copy p {
  margin: 0.35rem 0 0;
  color: rgba(71, 85, 105, 0.7);
}

.table-control {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: 600;
}

.toggle input {
  transform: scale(1.1);
}

.hotkey-display {
  border-radius: 12px;
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.96), rgba(222, 228, 238, 0.92));
  border: 1px solid rgba(148, 163, 184, 0.35);
  padding: 0.75rem 1.2rem;
  font-size: 1.1rem;
  letter-spacing: 0.08em;
  text-align: center;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.6);
}

.hotkey-hint {
  margin: 0;
  font-size: 0.85rem;
  color: rgba(71, 85, 105, 0.65);
}

.permission-button {
  justify-self: start;
  border: none;
  border-radius: 10px;
  padding: 0.6rem 1rem;
  background: linear-gradient(180deg, rgba(153, 102, 255, 0.92), rgba(91, 41, 172, 0.95));
  color: #fff;
  font-weight: 600;
  cursor: pointer;
  box-shadow: 0 12px 24px rgba(92, 41, 172, 0.35);
}

.preference-card textarea {
  width: 100%;
  border-radius: 12px;
  border: 1px solid rgba(148, 163, 184, 0.35);
  background: rgba(255, 255, 255, 0.85);
  font-family: "JetBrains Mono", "Menlo", monospace;
  padding: 0.85rem;
  color: #111827;
  resize: vertical;
}

.span-2 {
  grid-column: span 2;
}

.features-panel {
  padding: 0;
}

.feature-shell {
  display: grid;
  grid-template-columns: 220px 1fr;
  min-height: 420px;
}

.feature-tabs {
  background: linear-gradient(180deg, rgba(83, 35, 149, 0.14), rgba(83, 35, 149, 0.05));
  border-right: 1px solid rgba(148, 163, 184, 0.25);
  padding: 1.5rem 1rem;
  display: grid;
  gap: 0.5rem;
}

.feature-tab {
  border: none;
  border-radius: 10px;
  padding: 0.65rem 0.8rem;
  text-align: left;
  background: transparent;
  cursor: pointer;
  font-size: 0.95rem;
  color: rgba(55, 65, 81, 0.85);
  transition: background 0.15s ease, color 0.15s ease;
}

.feature-tab:hover {
  background: rgba(99, 102, 241, 0.18);
}

.feature-tab.active {
  background: rgba(99, 102, 241, 0.22);
  color: #312e81;
  font-weight: 600;
}

.feature-pane {
  padding: 2.25rem 2.5rem;
  display: grid;
  gap: 1.5rem;
  background: rgba(255, 255, 255, 0.92);
}

.feature-header h3 {
  margin: 0;
  font-size: 1.35rem;
}

.feature-header p {
  margin: 0.4rem 0 0;
  color: rgba(71, 85, 105, 0.72);
}

.feature-toggle {
  display: inline-flex;
  align-items: center;
  gap: 0.6rem;
  font-weight: 600;
}

.default-results {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 1.5rem;
}

.default-results h4,
.web-search h4 {
  margin: 0 0 0.75rem;
  font-size: 1rem;
}

.default-results ul,
.web-search ul {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  gap: 0.5rem;
  color: rgba(55, 65, 81, 0.85);
  font-size: 0.95rem;
}

.web-search table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.95rem;
  color: rgba(55, 65, 81, 0.85);
}

.web-search th,
.web-search td {
  padding: 0.6rem 0.75rem;
  border: 1px solid rgba(148, 163, 184, 0.25);
}

.web-search thead {
  background: rgba(99, 102, 241, 0.08);
}

.feature-placeholder {
  margin: 0;
  color: rgba(71, 85, 105, 0.7);
  line-height: 1.6;
}

.placeholder-panel {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 360px;
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.92), rgba(239, 243, 250, 0.92));
}

.placeholder-card {
  text-align: center;
  max-width: 360px;
  display: grid;
  gap: 0.75rem;
}

.placeholder-card h3 {
  margin: 0;
  font-size: 1.4rem;
}

.placeholder-card p {
  margin: 0;
  color: rgba(71, 85, 105, 0.72);
}

@media (max-width: 1080px) {
  .prefs-layout {
    grid-template-columns: 200px 1fr;
  }

  .prefs-sidebar {
    padding: 2rem 1.25rem;
  }

  .prefs-main {
    border-top-right-radius: 28px;
    border-bottom-right-radius: 28px;
  }

  .panel-area {
    padding: 2rem;
  }
}

@media (max-width: 860px) {
  .prefs-app {
    padding: 1.5rem 0;
  }

  .prefs-layout {
    grid-template-columns: 1fr;
    border-radius: 22px;
  }

  .prefs-sidebar {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    padding: 1.5rem;
    gap: 1.25rem;
  }

  .sidebar-groups {
    flex-direction: row;
    gap: 1rem;
  }

  .sidebar-group {
    display: contents;
  }

  .sidebar-group-title {
    display: none;
  }

  .sidebar-item {
    flex-direction: column;
    padding: 0.5rem;
  }

  .item-label {
    font-size: 0.75rem;
  }

  .prefs-main {
    border-radius: 0;
  }

  .panel-grid {
    grid-template-columns: 1fr;
  }

  .span-2 {
    grid-column: span 1;
  }

  .feature-shell {
    grid-template-columns: 1fr;
  }

  .feature-tabs {
    grid-auto-flow: column;
    justify-content: flex-start;
    overflow-x: auto;
  }
}
</style>
