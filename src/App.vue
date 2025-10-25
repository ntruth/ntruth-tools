<template>
  <div class="prefs">
    <aside class="nav">
      <div class="brand">
        <span class="logo">🚀</span>
        <div>
          <h1>UniTools</h1>
          <p>偏好设置</p>
        </div>
      </div>
      <nav>
        <button :class="{ active: section === 'general' }" @click="section = 'general'">
          通用
        </button>
        <button :class="{ active: section === 'features' }" @click="section = 'features'">
          功能
        </button>
      </nav>
      <footer>
        <p>Command + , 随时打开本窗口</p>
      </footer>
    </aside>

    <main>
      <section v-if="section === 'general'" class="panel">
        <header>
          <h2>通用</h2>
          <p>配置启动行为、界面元素以及默认体验。</p>
        </header>
        <div class="form-group">
          <label>
            <input
              type="checkbox"
              :checked="settings.state.general.launchAtLogin"
              @change="onToggleGeneral('launchAtLogin', $event)"
            />
            开机自动启动 UniTools
          </label>
          <small>保持工具始终就绪，登录后立即显示搜索框。</small>
        </div>
        <div class="form-group">
          <label>
            <input
              type="checkbox"
              :checked="settings.state.general.autoLaunchSearch"
              @change="onToggleGeneral('autoLaunchSearch', $event)"
            />
            启动后立即唤起搜索器
          </label>
          <small>应用启动后自动展示搜索输入框，模拟 Alfred 体验。</small>
        </div>
        <div class="form-group">
          <label>
            <input
              type="checkbox"
              :checked="settings.state.general.showTrayIcon"
              @change="onToggleGeneral('showTrayIcon', $event)"
            />
            在菜单栏显示状态图标
          </label>
          <small>通过菜单栏快速打开启动器、剪贴板历史或偏好设置。</small>
        </div>
        <div class="form-group">
          <label class="inline">Web 备用搜索</label>
          <small>按顺序回退的搜索引擎，使用 %s 填充查询词。</small>
          <textarea
            rows="4"
            :value="settings.state.webFallbacks.join('\n')"
            @change="onFallbackChange"
          ></textarea>
        </div>
        <div class="form-group">
          <label class="inline">主题</label>
          <ThemeToggle />
          <small>在深色与浅色之间切换界面外观。</small>
        </div>
      </section>

      <section v-else class="panel">
        <header class="features-header">
          <div>
            <h2>功能</h2>
            <p>启用或禁用不同功能模块，定制搜索结果。</p>
          </div>
          <div class="feature-tabs">
            <button
              v-for="item in featureTabs"
              :key="item.key"
              :class="{ active: activeFeature === item.key }"
              @click="activeFeature = item.key"
            >{{ item.label }}</button>
          </div>
        </header>

        <div v-if="activeFeature === 'defaultResults'" class="form-group">
          <h3>默认结果</h3>
          <label>
            <input
              type="checkbox"
              :checked="settings.state.features.defaultResults"
              @change="onToggleFeature('defaultResults', $event)"
            />
            展示常用应用、命令与内部工具
          </label>
          <small>关闭后仅保留插件和手动配置结果。</small>
        </div>

        <div v-else-if="activeFeature === 'fileSearch'" class="form-group">
          <h3>文件搜索</h3>
          <label>
            <input
              type="checkbox"
              :checked="settings.state.features.fileSearch"
              @change="onToggleFeature('fileSearch', $event)"
            />
            在搜索中包含文件与应用程序
          </label>
          <small>索引 `~/Applications`、`/Applications`、当前工作区等路径。</small>
        </div>

        <div v-else-if="activeFeature === 'universalActions'" class="form-group">
          <h3>Universal Actions</h3>
          <label>
            <input
              type="checkbox"
              :checked="settings.state.features.universalActions"
              @change="onToggleFeature('universalActions', $event)"
            />
            启用针对选中内容的通用操作
          </label>
          <small>后续版本将开放自定义动作编辑器。</small>
        </div>

        <div v-else-if="activeFeature === 'webSearch'" class="form-group">
          <h3>Web 搜索</h3>
          <label>
            <input
              type="checkbox"
              :checked="settings.state.features.webSearch"
              @change="onToggleFeature('webSearch', $event)"
            />
            启用 Web 搜索建议
          </label>
        </div>

        <div v-else-if="activeFeature === 'webBookmarks'" class="form-group">
          <h3>Web 书签</h3>
          <label>
            <input
              type="checkbox"
              :checked="settings.state.features.webBookmarks"
              @change="onToggleFeature('webBookmarks', $event)"
            />
            索引浏览器书签
          </label>
          <small>Chrome / Safari 将在后台周期刷新。</small>
        </div>

        <div v-else-if="activeFeature === 'clipboardHistory'" class="form-group">
          <h3>剪贴板历史</h3>
          <label>
            <input
              type="checkbox"
              :checked="settings.state.features.clipboardHistory"
              @change="onToggleFeature('clipboardHistory', $event)"
            />
            保存文本剪贴板内容
          </label>
          <small>在启动器中输入 `clip` 即可快速查找历史记录。</small>
        </div>

        <div v-else-if="activeFeature === 'snippets'" class="form-group">
          <h3>Snippets</h3>
          <label>
            <input
              type="checkbox"
              :checked="settings.state.features.snippets"
              @change="onToggleFeature('snippets', $event)"
            />
            启用文本片段管理器
          </label>
          <small>准备支持通配符与动态变量。</small>
        </div>

        <div v-else-if="activeFeature === 'calculator'" class="form-group">
          <h3>计算器</h3>
          <label>
            <input
              type="checkbox"
              :checked="settings.state.features.calculator"
              @change="onToggleFeature('calculator', $event)"
            />
            在搜索框直接执行四则运算
          </label>
        </div>
      </section>
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useSettingsStore } from "@/stores/settings";
import ThemeToggle from "@/components/ThemeToggle.vue";

const settings = useSettingsStore();
const section = ref<"general" | "features">("general");
const featureTabs = [
  { key: "defaultResults", label: "Default Results" },
  { key: "fileSearch", label: "File Search" },
  { key: "universalActions", label: "Universal Actions" },
  { key: "webSearch", label: "Web Search" },
  { key: "webBookmarks", label: "Web Bookmarks" },
  { key: "clipboardHistory", label: "Clipboard History" },
  { key: "snippets", label: "Snippets" },
  { key: "calculator", label: "Calculator" }
] as const;
const activeFeature = ref<(typeof featureTabs)[number]["key"]>("defaultResults");

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
.prefs {
  display: grid;
  grid-template-columns: 260px 1fr;
  min-height: 100vh;
  background: var(--color-background);
}

.nav {
  padding: 2rem 1.5rem;
  border-right: 1px solid rgba(148, 163, 184, 0.2);
  display: flex;
  flex-direction: column;
  gap: 2rem;
  background: linear-gradient(160deg, rgba(99, 102, 241, 0.18), rgba(14, 165, 233, 0.1));
}

.brand {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.brand h1 {
  margin: 0;
  font-size: 1.4rem;
}

.brand p {
  margin: 0.25rem 0 0;
  color: var(--color-muted);
  font-size: 0.85rem;
}

.logo {
  font-size: 2.4rem;
}

.nav nav {
  display: grid;
  gap: 0.5rem;
}

.nav button {
  border: none;
  border-radius: 0.8rem;
  padding: 0.5rem 0.75rem;
  text-align: left;
  background: rgba(15, 23, 42, 0.05);
  color: inherit;
  cursor: pointer;
}

.nav button.active {
  background: rgba(99, 102, 241, 0.25);
  color: #312e81;
}

.nav footer {
  margin-top: auto;
  font-size: 0.8rem;
  color: rgba(15, 23, 42, 0.55);
}

main {
  padding: 2.5rem;
}

.panel {
  background: var(--color-surface);
  border-radius: 1.25rem;
  border: 1px solid rgba(148, 163, 184, 0.2);
  padding: 2rem;
  display: grid;
  gap: 1.75rem;
  box-shadow: 0 30px 70px rgba(15, 23, 42, 0.12);
}

.panel header h2 {
  margin: 0;
  font-size: 1.6rem;
}

.panel header p {
  margin: 0.4rem 0 0;
  color: var(--color-muted);
}

.form-group {
  display: grid;
  gap: 0.5rem;
  background: rgba(15, 23, 42, 0.03);
  border-radius: 1rem;
  padding: 1rem 1.25rem;
}

.form-group label {
  font-weight: 600;
  display: flex;
  gap: 0.75rem;
  align-items: center;
}

.form-group small {
  color: var(--color-muted);
  line-height: 1.4;
}

.form-group textarea {
  width: 100%;
  resize: vertical;
  min-height: 120px;
  border-radius: 0.75rem;
  border: 1px solid rgba(148, 163, 184, 0.35);
  padding: 0.75rem;
  font-family: var(--font-mono, "JetBrains Mono", monospace);
  background: var(--color-background);
  color: inherit;
}

.features-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
}

.feature-tabs {
  display: grid;
  gap: 0.35rem;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.feature-tabs button {
  border: none;
  border-radius: 0.75rem;
  padding: 0.45rem 0.6rem;
  background: rgba(148, 163, 184, 0.2);
  cursor: pointer;
  font-size: 0.85rem;
}

.feature-tabs button.active {
  background: rgba(14, 165, 233, 0.25);
  color: #0369a1;
}

@media (max-width: 960px) {
  .prefs {
    grid-template-columns: 1fr;
  }

  .nav {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
  }

  .nav nav {
    display: flex;
  }

  main {
    padding: 1.5rem;
  }
}
</style>
