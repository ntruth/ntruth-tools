<template>
  <div class="prefs-shell">
    <ClipboardPanel />
    <div class="prefs-window">
      <aside class="sidebar">
        <div class="sidebar-header">
          <div class="sidebar-logo">🎩</div>
          <div>
            <h1>UniTools</h1>
            <p>偏好设置</p>
          </div>
        </div>
        <nav class="sidebar-nav">
          <button
            v-for="item in sections"
            :key="item.value"
            :class="['nav-item', { active: section === item.value }]"
            @click="section = item.value"
          >
            <span class="nav-icon">{{ item.icon }}</span>
            <span>{{ item.label }}</span>
          </button>
        </nav>
        <footer class="sidebar-footer">
          <p>按下 ⌘ + , 即可打开本窗口</p>
        </footer>
      </aside>

      <main class="content">
        <section v-if="section === 'general'" class="panel">
          <header class="panel-header">
            <h2>通用</h2>
            <p>管理启动器的基本体验与系统集成。</p>
          </header>
          <div class="setting-row">
            <div class="setting-copy">
              <h3>开机自动启动</h3>
              <p>登录后立即加载 UniTools，并在后台待命。</p>
            </div>
            <label class="setting-toggle">
              <input
                type="checkbox"
                :checked="settings.state.general.launchAtLogin"
                @change="onToggleGeneral('launchAtLogin', $event)"
              />
              <span>启用</span>
            </label>
          </div>
          <div class="setting-row">
            <div class="setting-copy">
              <h3>自动唤起搜索</h3>
              <p>应用启动时立即弹出启动器，保持与 Alfred 一致。</p>
            </div>
            <label class="setting-toggle">
              <input
                type="checkbox"
                :checked="settings.state.general.autoLaunchSearch"
                @change="onToggleGeneral('autoLaunchSearch', $event)"
              />
              <span>启用</span>
            </label>
          </div>
          <div class="setting-row">
            <div class="setting-copy">
              <h3>菜单栏图标</h3>
              <p>在菜单栏显示状态图标，快速访问启动器与剪贴板历史。</p>
            </div>
            <label class="setting-toggle">
              <input
                type="checkbox"
                :checked="settings.state.general.showTrayIcon"
                @change="onToggleGeneral('showTrayIcon', $event)"
              />
              <span>显示</span>
            </label>
          </div>
          <div class="setting-card">
            <header>
              <h3>Web 备用搜索</h3>
              <p>按照顺序使用备用搜索引擎，使用 %s 填充关键字。</p>
            </header>
            <textarea
              rows="4"
              :value="settings.state.webFallbacks.join('\n')"
              @change="onFallbackChange"
            ></textarea>
          </div>
          <div class="setting-card">
            <header>
              <h3>主题</h3>
              <p>切换浅色或深色外观，与 Alfred 主题保持一致。</p>
            </header>
            <ThemeToggle />
          </div>
        </section>

        <section v-else-if="section === 'features'" class="panel features">
          <header class="panel-header">
            <h2>功能</h2>
            <p>启用或禁用不同模块，定制搜索结果与体验。</p>
          </header>
          <div class="features-body">
            <aside class="feature-nav">
              <button
                v-for="item in featureTabs"
                :key="item.key"
                :class="['feature-item', { active: activeFeature === item.key }]"
                @click="activeFeature = item.key"
              >
                {{ item.label }}
              </button>
            </aside>
            <div class="feature-content">
              <article v-if="activeFeature === 'defaultResults'" class="feature-card">
                <h3>默认结果</h3>
                <p>展示常用应用、命令与内部工具。</p>
                <label>
                  <input
                    type="checkbox"
                    :checked="settings.state.features.defaultResults"
                    @change="onToggleFeature('defaultResults', $event)"
                  />
                  启用默认结果
                </label>
              </article>

              <article v-else-if="activeFeature === 'fileSearch'" class="feature-card">
                <h3>文件搜索</h3>
                <p>包含 `~/Applications`、`/Applications` 以及当前工作区的索引。</p>
                <label>
                  <input
                    type="checkbox"
                    :checked="settings.state.features.fileSearch"
                    @change="onToggleFeature('fileSearch', $event)"
                  />
                  启用文件搜索
                </label>
              </article>

              <article v-else-if="activeFeature === 'universalActions'" class="feature-card">
                <h3>Universal Actions</h3>
                <p>对选中内容执行动作，后续版本将开放编辑器。</p>
                <label>
                  <input
                    type="checkbox"
                    :checked="settings.state.features.universalActions"
                    @change="onToggleFeature('universalActions', $event)"
                  />
                  启用 Universal Actions
                </label>
              </article>

              <article v-else-if="activeFeature === 'webSearch'" class="feature-card">
                <h3>Web 搜索</h3>
                <p>在搜索结果中显示在线搜索建议。</p>
                <label>
                  <input
                    type="checkbox"
                    :checked="settings.state.features.webSearch"
                    @change="onToggleFeature('webSearch', $event)"
                  />
                  启用 Web 搜索
                </label>
              </article>

              <article v-else-if="activeFeature === 'webBookmarks'" class="feature-card">
                <h3>Web 书签</h3>
                <p>索引 Chrome 与 Safari 书签，后台自动刷新。</p>
                <label>
                  <input
                    type="checkbox"
                    :checked="settings.state.features.webBookmarks"
                    @change="onToggleFeature('webBookmarks', $event)"
                  />
                  启用书签索引
                </label>
              </article>

              <article v-else-if="activeFeature === 'clipboardHistory'" class="feature-card">
                <h3>剪贴板历史</h3>
                <p>记录文本剪贴板内容，在启动器中输入 “clip” 即可查找。</p>
                <label>
                  <input
                    type="checkbox"
                    :checked="settings.state.features.clipboardHistory"
                    @change="onToggleFeature('clipboardHistory', $event)"
                  />
                  启用剪贴板历史
                </label>
              </article>

              <article v-else-if="activeFeature === 'snippets'" class="feature-card">
                <h3>Snippets</h3>
                <p>管理文本片段，准备支持通配符与动态变量。</p>
                <label>
                  <input
                    type="checkbox"
                    :checked="settings.state.features.snippets"
                    @change="onToggleFeature('snippets', $event)"
                  />
                  启用 Snippets
                </label>
              </article>

              <article v-else class="feature-card">
                <h3>计算器</h3>
                <p>在搜索框直接执行四则运算与函数计算。</p>
                <label>
                  <input
                    type="checkbox"
                    :checked="settings.state.features.calculator"
                    @change="onToggleFeature('calculator', $event)"
                  />
                  启用计算器
                </label>
              </article>
            </div>
          </div>
        </section>

        <section v-else class="panel placeholder">
          <header class="panel-header">
            <h2>{{ currentPlaceholder?.title }}</h2>
            <p>{{ currentPlaceholder?.description }}</p>
          </header>
          <div class="placeholder-body">
            <p>该区域的高级设置正在构建中，敬请期待。</p>
          </div>
        </section>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useSettingsStore } from "@/stores/settings";
import ThemeToggle from "@/components/ThemeToggle.vue";
import ClipboardPanel from "@/features/clipboard/ClipboardPanel.vue";

const settings = useSettingsStore();

const sections = [
  { value: "general", label: "通用", icon: "⚙️" },
  { value: "features", label: "功能", icon: "✨" },
  { value: "workflows", label: "工作流", icon: "🧩" },
  { value: "appearance", label: "外观", icon: "🎨" },
  { value: "remote", label: "远程", icon: "📱" },
  { value: "advanced", label: "高级", icon: "🛠" },
  { value: "powerpack", label: "Powerpack", icon: "⚡" }
] as const;

type Section = (typeof sections)[number]["value"];

const section = ref<Section>("general");

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

const activeFeature = ref<(typeof featureTabs)[number]["key"]>("defaultResults");

const placeholderMap: Record<Exclude<Section, "general" | "features">, { title: string; description: string }> = {
  workflows: { title: "工作流", description: "自动化任务与组合动作的集中管理。" },
  appearance: { title: "外观", description: "自定义主题、字体与窗口展示效果。" },
  remote: { title: "远程", description: "使用移动端 Alfred Remote 扩展工作流程。" },
  advanced: { title: "高级", description: "调节索引、兼容性与开发者相关设置。" },
  powerpack: { title: "Powerpack", description: "管理授权、同步与高级功能解锁。" }
};

const currentPlaceholder = computed(() => {
  if (section.value === "general" || section.value === "features") {
    return null;
  }
  return placeholderMap[section.value];
});

const onToggleGeneral = (key: Parameters<typeof settings.toggleGeneral>[0], event: Event) => {
  const input = event.target as HTMLInputElement;
  settings.toggleGeneral(key, input.checked);
};

const onToggleFeature = (key: Parameters<typeof settings.toggleFeature>[0], event: Event) => {
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
.prefs-shell {
  min-height: 100vh;
  background: radial-gradient(circle at top, rgba(99, 102, 241, 0.25), transparent 55%),
    var(--color-background);
  padding: 2rem 0;
  display: flex;
  justify-content: center;
}

.prefs-window {
  width: min(1080px, 100% - 3rem);
  min-height: 680px;
  background: var(--color-surface);
  border-radius: 24px;
  border: 1px solid rgba(148, 163, 184, 0.25);
  display: grid;
  grid-template-columns: 220px 1fr;
  box-shadow: 0 40px 95px rgba(15, 23, 42, 0.22);
  overflow: hidden;
}

.sidebar {
  background: linear-gradient(165deg, rgba(15, 23, 42, 0.95), rgba(30, 41, 59, 0.92));
  color: #f8fafc;
  padding: 2rem 1.5rem 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.sidebar-header {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.sidebar-logo {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  display: grid;
  place-items: center;
  font-size: 1.6rem;
  background: rgba(99, 102, 241, 0.35);
}

.sidebar-header h1 {
  margin: 0;
  font-size: 1.35rem;
}

.sidebar-header p {
  margin: 0.3rem 0 0;
  color: rgba(226, 232, 240, 0.7);
  font-size: 0.9rem;
}

.sidebar-nav {
  display: grid;
  gap: 0.4rem;
}

.nav-item {
  border: none;
  border-radius: 0.9rem;
  padding: 0.65rem 0.8rem;
  display: flex;
  align-items: center;
  gap: 0.65rem;
  background: transparent;
  color: inherit;
  cursor: pointer;
  font-size: 0.95rem;
  transition: background 0.15s ease;
}

.nav-item:hover {
  background: rgba(99, 102, 241, 0.3);
}

.nav-item.active {
  background: rgba(129, 140, 248, 0.35);
}

.nav-icon {
  font-size: 1.05rem;
}

.sidebar-footer {
  margin-top: auto;
  font-size: 0.8rem;
  color: rgba(226, 232, 240, 0.7);
}

.content {
  padding: 2.5rem;
  display: flex;
}

.panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.panel-header h2 {
  margin: 0;
  font-size: 1.75rem;
}

.panel-header p {
  margin: 0.5rem 0 0;
  color: var(--color-muted);
}

.setting-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.2rem 1.4rem;
  background: rgba(148, 163, 184, 0.12);
  border-radius: 1.1rem;
  gap: 1.5rem;
}

.setting-copy h3 {
  margin: 0;
  font-size: 1rem;
}

.setting-copy p {
  margin: 0.35rem 0 0;
  color: var(--color-muted);
}

.setting-toggle {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: 600;
}

.setting-toggle input {
  transform: scale(1.2);
}

.setting-card {
  background: rgba(148, 163, 184, 0.1);
  border-radius: 1.1rem;
  padding: 1.4rem;
  display: grid;
  gap: 0.8rem;
}

.setting-card header h3 {
  margin: 0;
  font-size: 1.1rem;
}

.setting-card header p {
  margin: 0.4rem 0 0;
  color: var(--color-muted);
}

.setting-card textarea {
  width: 100%;
  border-radius: 0.75rem;
  border: 1px solid rgba(148, 163, 184, 0.35);
  padding: 0.75rem;
  font-family: var(--font-mono, "JetBrains Mono", monospace);
  background: var(--color-background);
  color: inherit;
  resize: vertical;
}

.features {
  gap: 1.5rem;
}

.features-body {
  display: grid;
  grid-template-columns: 200px 1fr;
  gap: 1.5rem;
}

.feature-nav {
  background: rgba(148, 163, 184, 0.12);
  border-radius: 1.1rem;
  padding: 1rem;
  display: grid;
  gap: 0.5rem;
}

.feature-item {
  border: none;
  border-radius: 0.75rem;
  padding: 0.6rem 0.8rem;
  text-align: left;
  background: transparent;
  cursor: pointer;
  font-size: 0.95rem;
  color: inherit;
  transition: background 0.15s ease;
}

.feature-item:hover {
  background: rgba(129, 140, 248, 0.22);
}

.feature-item.active {
  background: rgba(99, 102, 241, 0.22);
  color: #312e81;
}

.feature-content {
  display: grid;
  grid-template-columns: 1fr;
}

.feature-card {
  background: rgba(148, 163, 184, 0.08);
  border-radius: 1.1rem;
  padding: 1.4rem;
  display: grid;
  gap: 0.8rem;
}

.feature-card h3 {
  margin: 0;
}

.feature-card p {
  margin: 0;
  color: var(--color-muted);
}

.placeholder {
  align-items: flex-start;
}

.placeholder-body {
  padding: 2.5rem;
  background: rgba(148, 163, 184, 0.1);
  border-radius: 1.2rem;
  color: var(--color-muted);
}

@media (max-width: 1080px) {
  .prefs-window {
    grid-template-columns: 1fr;
  }

  .sidebar {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
  }

  .sidebar-nav {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }

  .content {
    padding: 2rem 1.5rem 2.5rem;
  }

  .features-body {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 720px) {
  .prefs-shell {
    padding: 1rem;
  }

  .prefs-window {
    border-radius: 18px;
  }
}
</style>
