<template>
  <teleport to="body">
    <transition name="fade">
      <div
        v-if="store.isPanelVisible && settings.state.features.clipboardHistory"
        class="overlay"
        @click.self="close"
      >
        <section class="panel">
          <header>
            <div>
              <h2>剪贴板历史</h2>
              <p>固定关键片段、搜索历史并一键复制。</p>
            </div>
            <div class="actions">
              <button class="ghost" @click.stop="refresh">刷新</button>
              <button class="ghost" @click.stop="capture">捕获当前</button>
              <button class="danger" @click.stop="clear">清除未固定</button>
              <button class="ghost" @click.stop="close">ESC</button>
            </div>
          </header>

          <div class="controls">
            <input
              :value="store.state.search"
              placeholder="搜索内容或标签"
              @input="(event) => store.setSearch((event.target as HTMLInputElement).value)"
            />
            <div class="filters">
              <button
                v-for="option in filters"
                :key="option.value"
                :class="['chip', { active: store.state.filter === option.value }]"
                @click="store.setFilter(option.value)"
              >
                {{ option.label }}
              </button>
            </div>
          </div>

          <div class="list" v-if="!store.state.loading">
            <article
              v-for="item in store.filteredItems"
              :key="item.id"
              class="row"
              @dblclick="() => onCopy(item)"
            >
              <aside>
                <button class="pin" @click.stop="() => onPin(item)">
                  {{ item.pinned ? '★' : '☆' }}
                </button>
              </aside>
              <div class="body">
                <header>
                  <strong>{{ previewTitle(item) }}</strong>
                  <time>{{ formatTime(item.created_at) }}</time>
                </header>
                <p>{{ previewContent(item) }}</p>
                <footer v-if="item.tags.length" class="tags">
                  <span v-for="tag in item.tags" :key="tag">#{{ tag }}</span>
                </footer>
              </div>
              <div class="row-actions">
                <button class="ghost" @click.stop="() => onCopy(item)">复制</button>
                <button class="ghost" @click.stop="() => onRemove(item)">删除</button>
              </div>
            </article>

            <p v-if="!store.filteredItems.length" class="empty">
              暂无记录，点击“捕获当前”保存系统剪贴板。
            </p>
          </div>
          <div v-else class="loading">加载中...</div>
        </section>
      </div>
    </transition>
  </teleport>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useClipboardStore } from "@/stores/clipboard";
import type { ClipboardEntry } from "@/features/clipboard/types";
import { useSettingsStore } from "@/stores/settings";

const store = useClipboardStore();
const settings = useSettingsStore();

const filters = computed(() => [
  { label: "全部", value: "all" },
  { label: "文本", value: "text" },
  { label: "图片", value: "image" },
  { label: "文件", value: "file" }
]);

const formatTime = (iso: string) => {
  const date = new Date(iso);
  return date.toLocaleString();
};

const previewTitle = (item: ClipboardEntry) => (item.pinned ? "已固定" : item.type);

const previewContent = (item: ClipboardEntry) => {
  if (item.content.length > 140) {
    return `${item.content.slice(0, 140)}...`;
  }
  return item.content;
};

const refresh = () => {
  void store.fetchHistory();
};

const capture = () => {
  void store.captureCurrent();
};

const onPin = (item: ClipboardEntry) => {
  void store.togglePin(item);
};

const onRemove = (item: ClipboardEntry) => {
  void store.remove(item);
};

const onCopy = (item: ClipboardEntry) => {
  void store.copyToClipboard(item);
};

const clear = () => {
  void store.clearUnpinned();
};

const close = () => {
  store.togglePanel(false);
};
</script>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(15, 23, 42, 0.6);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1200;
  padding: 2rem;
}

.panel {
  width: min(980px, 100%);
  max-height: 90vh;
  background: var(--color-surface);
  border-radius: 1.5rem;
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  box-shadow: 0 30px 80px rgba(15, 23, 42, 0.25);
}

header {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  align-items: flex-start;
}

header h2 {
  margin: 0 0 0.25rem;
}

header p {
  margin: 0;
  color: var(--color-muted);
}

.actions {
  display: flex;
  gap: 0.5rem;
}

.ghost,
.danger {
  border: none;
  border-radius: 0.75rem;
  padding: 0.4rem 0.85rem;
  cursor: pointer;
  background: rgba(148, 163, 184, 0.18);
  color: inherit;
}

.danger {
  background: rgba(248, 113, 113, 0.18);
  color: #dc2626;
}

.controls {
  display: grid;
  gap: 0.75rem;
}

.controls input {
  border-radius: 0.75rem;
  border: 1px solid rgba(148, 163, 184, 0.35);
  padding: 0.6rem 0.85rem;
  font-size: 0.95rem;
}

.filters {
  display: flex;
  gap: 0.5rem;
}

.chip {
  border: none;
  padding: 0.35rem 0.9rem;
  border-radius: 999px;
  background: rgba(148, 163, 184, 0.18);
  cursor: pointer;
}

.chip.active {
  background: rgba(99, 102, 241, 0.2);
  color: #4338ca;
}

.list {
  flex: 1;
  overflow-y: auto;
  display: grid;
  gap: 0.75rem;
}

.row {
  display: grid;
  grid-template-columns: auto 1fr auto;
  gap: 0.85rem;
  padding: 0.85rem;
  border-radius: 1rem;
  background: rgba(99, 102, 241, 0.05);
}

.pin {
  border: none;
  background: none;
  font-size: 1.3rem;
  cursor: pointer;
}

.body header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.body header time {
  font-size: 0.75rem;
  color: var(--color-muted);
}

.body p {
  margin: 0.5rem 0 0;
  font-size: 0.95rem;
}

.tags {
  display: flex;
  gap: 0.35rem;
  margin-top: 0.75rem;
}

.tags span {
  background: rgba(148, 163, 184, 0.2);
  border-radius: 0.5rem;
  padding: 0.15rem 0.5rem;
  font-size: 0.75rem;
}

.row-actions {
  display: flex;
  flex-direction: column;
  gap: 0.45rem;
}

.empty {
  text-align: center;
  color: var(--color-muted);
}

.loading {
  text-align: center;
  padding: 2rem 0;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

@media (max-width: 960px) {
  .panel {
    padding: 1rem;
    max-height: 100vh;
  }

  header,
  .row {
    grid-template-columns: 1fr;
  }

  .actions {
    flex-wrap: wrap;
    justify-content: flex-end;
  }
}
</style>
