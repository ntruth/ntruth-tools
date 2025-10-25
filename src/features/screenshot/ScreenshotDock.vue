<template>
  <section class="dock">
    <header>
      <h3>截图与贴图</h3>
      <span>快速生成参考贴图，固定重要画面。</span>
    </header>

    <div class="controls">
      <input v-model="note" placeholder="截图备注（可选）" />
      <button :disabled="store.state.capturing" @click="capture">
        {{ store.state.capturing ? "正在捕获..." : "开始截图" }}
      </button>
    </div>

    <div v-if="store.state.loading" class="loading">加载截图记录...</div>

    <div v-else class="grid">
      <article v-for="shot in store.state.items" :key="shot.id" class="card">
        <img :src="shot.data_url" alt="screenshot" />
        <footer>
          <div>
            <strong>{{ formatTime(shot.created_at) }}</strong>
            <p v-if="shot.note">{{ shot.note }}</p>
          </div>
          <div class="actions">
            <button class="ghost" @click="() => togglePin(shot)">
              {{ shot.pinned ? "取消固定" : "固定" }}
            </button>
            <button class="ghost" @click="() => remove(shot)">删除</button>
          </div>
        </footer>
      </article>

      <p v-if="!store.state.items.length" class="empty">
        暂无截图。点击“开始截图”将生成示例图像并保存到贴图列表。
      </p>
    </div>
  </section>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useScreenshotStore } from "@/stores/screenshot";
import type { ScreenshotEntry } from "@/stores/screenshot";

const store = useScreenshotStore();
const note = ref("");

const capture = () => {
  void store.capture(note.value || undefined);
  note.value = "";
};

const togglePin = (entry: ScreenshotEntry) => {
  void store.togglePin(entry);
};

const remove = (entry: ScreenshotEntry) => {
  void store.remove(entry);
};

const formatTime = (iso: string) => new Date(iso).toLocaleString();

onMounted(() => {
  void store.load();
});
</script>

<style scoped>
.dock {
  background: var(--color-surface);
  border-radius: 1.25rem;
  padding: 1.25rem;
  border: 1px solid rgba(15, 23, 42, 0.05);
  display: grid;
  gap: 1rem;
}

header h3 {
  margin: 0;
  font-size: 1.1rem;
}

header span {
  color: var(--color-muted);
  font-size: 0.85rem;
}

.controls {
  display: flex;
  gap: 0.75rem;
}

.controls input {
  flex: 1;
  border-radius: 0.75rem;
  border: 1px solid rgba(148, 163, 184, 0.35);
  padding: 0.6rem 0.85rem;
}

.controls button {
  border: none;
  border-radius: 0.75rem;
  background: linear-gradient(120deg, #818cf8 0%, #6366f1 100%);
  color: #fff;
  padding: 0.55rem 1.1rem;
  cursor: pointer;
}

.grid {
  display: grid;
  gap: 1rem;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
}

.card {
  background: rgba(148, 163, 184, 0.08);
  border-radius: 1rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
}

.card img {
  width: 100%;
  height: 120px;
  object-fit: cover;
}

.card footer {
  padding: 0 0.75rem 0.85rem;
  display: flex;
  justify-content: space-between;
  gap: 0.5rem;
  align-items: flex-start;
}

.card footer p {
  margin: 0.25rem 0 0;
  font-size: 0.85rem;
  color: var(--color-muted);
}

.actions {
  display: flex;
  flex-direction: column;
  gap: 0.45rem;
}

.ghost {
  border: none;
  border-radius: 0.65rem;
  background: rgba(148, 163, 184, 0.25);
  padding: 0.3rem 0.7rem;
  cursor: pointer;
}

.empty {
  grid-column: 1 / -1;
  text-align: center;
  color: var(--color-muted);
}

.loading {
  text-align: center;
  color: var(--color-muted);
}
</style>
