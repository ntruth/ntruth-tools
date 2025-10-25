<template>
  <section class="search">
    <header>
      <h3>文件搜索</h3>
      <span>即时搜索项目文档、源代码与配置。</span>
    </header>

    <div class="toolbar">
      <input
        v-model="input"
        placeholder="输入关键词，如 launcher、.rs 或 README"
        @keyup.enter="run"
      />
      <button :disabled="store.state.loading" @click="run">
        {{ store.state.loading ? "搜索中…" : "搜索" }}
      </button>
      <button class="ghost" @click="refresh">重建索引</button>
    </div>

    <div v-if="store.state.loading" class="loading">正在检索文件...</div>

    <ul v-else class="results">
      <li v-for="item in store.state.results" :key="item.document.id" class="row">
        <div class="meta">
          <strong>{{ item.document.name }}</strong>
          <p>{{ item.document.path }}</p>
        </div>
        <div class="info">
          <span>评分 {{ item.score.toFixed(1) }}</span>
          <span v-if="item.document.ext">类型 .{{ item.document.ext }}</span>
          <span>{{ formatSize(item.document.size) }}</span>
        </div>
      </li>
      <li v-if="!store.state.results.length" class="empty">
        暂无结果，尝试更精确的关键字或刷新索引。
      </li>
    </ul>
  </section>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useFileSearchStore } from "@/stores/search";

const store = useFileSearchStore();
const input = ref("");

const run = () => {
  void store.runSearch(input.value);
};

const refresh = () => {
  void store.refreshIndex();
};

const formatSize = (size: number) => {
  if (size > 1_000_000) {
    return `${(size / 1_000_000).toFixed(1)} MB`;
  }
  if (size > 1_000) {
    return `${(size / 1_000).toFixed(1)} KB`;
  }
  return `${size} B`;
};

onMounted(() => {
  void store.runSearch("");
});
</script>

<style scoped>
.search {
  background: var(--color-surface);
  border-radius: 1.25rem;
  border: 1px solid rgba(15, 23, 42, 0.05);
  padding: 1.25rem;
  display: grid;
  gap: 1rem;
}

header h3 {
  margin: 0;
}

header span {
  color: var(--color-muted);
  font-size: 0.85rem;
}

.toolbar {
  display: flex;
  gap: 0.75rem;
}

.toolbar input {
  flex: 1;
  border-radius: 0.75rem;
  border: 1px solid rgba(148, 163, 184, 0.35);
  padding: 0.6rem 0.85rem;
}

.toolbar button {
  border: none;
  border-radius: 0.75rem;
  background: linear-gradient(120deg, #34d399 0%, #22c55e 100%);
  color: #fff;
  padding: 0.55rem 1.2rem;
  cursor: pointer;
}

.toolbar .ghost {
  background: rgba(148, 163, 184, 0.25);
  color: inherit;
}

.results {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  gap: 0.85rem;
}

.row {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  background: rgba(148, 163, 184, 0.08);
  padding: 0.85rem;
  border-radius: 0.9rem;
}

.row p {
  margin: 0.35rem 0 0;
  font-size: 0.82rem;
  color: var(--color-muted);
}

.info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  align-items: flex-end;
  font-size: 0.8rem;
  color: var(--color-muted);
}

.empty {
  text-align: center;
  color: var(--color-muted);
}

.loading {
  text-align: center;
  color: var(--color-muted);
}

@media (max-width: 768px) {
  .toolbar {
    flex-direction: column;
  }

  .row {
    flex-direction: column;
    align-items: flex-start;
  }

  .info {
    align-items: flex-start;
  }
}
</style>
