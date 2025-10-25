<template>
  <div class="preview">
    <header>
      <h3>快速启动器预览</h3>
      <span>实时展示搜索排名与内置工具结果</span>
    </header>
    <input
      :value="query"
      placeholder="尝试输入 1+2 或 tr en 你好"
      @input="onInput"
    />
    <div class="preview-results">
      <div v-for="item in results" :key="item.entry.id" class="preview-row">
        <span class="icon">{{ item.entry.icon ?? "🔍" }}</span>
        <div class="info">
          <strong>{{ item.entry.label }}</strong>
          <small>{{ item.meta ?? item.entry.description }}</small>
        </div>
        <span class="score">{{ item.score }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useLauncherSearch } from "./useSearch";

const { query, results, setQuery } = useLauncherSearch();

const onInput = (event: Event) => {
  const value = (event.target as HTMLInputElement).value;
  setQuery(value);
};
</script>

<style scoped>
.preview {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  background: var(--color-surface);
  border-radius: 1rem;
  padding: 1.25rem;
  border: 1px solid rgba(15, 23, 42, 0.05);
}

header h3 {
  margin: 0;
  font-size: 1.1rem;
}

header span {
  font-size: 0.85rem;
  color: var(--color-muted);
}

input {
  border-radius: 0.75rem;
  border: 1px solid rgba(148, 163, 184, 0.35);
  padding: 0.65rem 0.85rem;
  font-size: 0.95rem;
  color: inherit;
  background: var(--color-background);
}

.preview-results {
  display: flex;
  flex-direction: column;
  gap: 0.65rem;
}

.preview-row {
  display: flex;
  align-items: center;
  gap: 0.8rem;
}

.icon {
  width: 2rem;
  height: 2rem;
  display: grid;
  place-items: center;
  font-size: 1.2rem;
}

.info {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.info strong {
  font-weight: 600;
}

.info small {
  color: var(--color-muted);
}

.score {
  font-family: "JetBrains Mono", "SFMono-Regular", ui-monospace, monospace;
  font-size: 0.8rem;
  color: var(--color-muted);
}
</style>
