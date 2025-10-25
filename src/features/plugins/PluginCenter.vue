<template>
  <section class="plugins">
    <header>
      <div>
        <h3>插件中心</h3>
        <span>扩展 UniTools 能力，连接第三方服务。</span>
      </div>
      <button class="ghost" @click="refresh">刷新</button>
    </header>

    <div v-if="store.state.loading" class="loading">加载插件数据...</div>

    <div v-else class="layout">
      <article class="panel">
        <h4>已安装</h4>
        <p v-if="!store.state.installed.length" class="muted">暂无插件。</p>
        <div class="list">
          <div v-for="plugin in store.state.installed" :key="plugin.id" class="item">
            <div>
              <strong>{{ plugin.name }}</strong>
              <small>{{ plugin.summary }}</small>
              <span class="meta">v{{ plugin.version }} · {{ plugin.author }}</span>
            </div>
            <button class="danger" @click="() => uninstall(plugin.id)">卸载</button>
          </div>
        </div>
      </article>

      <article class="panel">
        <h4>插件市场</h4>
        <div class="list">
          <div v-for="plugin in store.state.marketplace" :key="plugin.id" class="item">
            <div>
              <strong>{{ plugin.name }}</strong>
              <small>{{ plugin.summary }}</small>
              <span class="meta">{{ plugin.category }} · v{{ plugin.version }}</span>
            </div>
            <button
              class="primary"
              :disabled="installedIds.has(plugin.id)"
              @click="() => install(plugin.id)"
            >
              {{ installedIds.has(plugin.id) ? '已安装' : '安装' }}
            </button>
          </div>
        </div>
      </article>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { usePluginStore } from "@/stores/plugins";

const store = usePluginStore();

const installedIds = computed(() => new Set(store.state.installed.map((item) => item.id)));

const refresh = () => {
  void store.refresh();
};

const install = (id: string) => {
  void store.install(id);
};

const uninstall = (id: string) => {
  void store.uninstall(id);
};

onMounted(() => {
  void store.refresh();
});
</script>

<style scoped>
.plugins {
  background: var(--color-surface);
  border-radius: 1.25rem;
  padding: 1.25rem;
  border: 1px solid rgba(15, 23, 42, 0.05);
  display: grid;
  gap: 1rem;
}

header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

header span {
  color: var(--color-muted);
  font-size: 0.85rem;
}

.ghost,
.primary,
.danger {
  border: none;
  border-radius: 0.75rem;
  padding: 0.45rem 0.9rem;
  cursor: pointer;
}

.ghost {
  background: rgba(148, 163, 184, 0.25);
  color: inherit;
}

.primary {
  background: linear-gradient(120deg, #f97316 0%, #ef4444 100%);
  color: #fff;
}

.primary[disabled] {
  opacity: 0.5;
  cursor: default;
}

.danger {
  background: rgba(248, 113, 113, 0.22);
  color: #dc2626;
}

.layout {
  display: grid;
  gap: 1rem;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
}

.panel {
  background: rgba(148, 163, 184, 0.08);
  border-radius: 1rem;
  padding: 0.85rem;
  display: grid;
  gap: 0.75rem;
}

.list {
  display: grid;
  gap: 0.6rem;
}

.item {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  background: rgba(15, 23, 42, 0.06);
  padding: 0.6rem 0.75rem;
  border-radius: 0.9rem;
}

.item small {
  display: block;
  margin-top: 0.25rem;
  color: var(--color-muted);
}

.meta {
  display: block;
  margin-top: 0.3rem;
  font-size: 0.75rem;
  color: var(--color-muted);
}

.loading {
  text-align: center;
  color: var(--color-muted);
}

.muted {
  color: var(--color-muted);
}

@media (max-width: 768px) {
  .item {
    flex-direction: column;
  }

  .item button {
    align-self: flex-start;
  }
}
</style>
