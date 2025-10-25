<template>
  <div class="theme-toggle">
    <label class="theme-label" for="theme-select">界面主题</label>
    <select
      id="theme-select"
      class="theme-select"
      :value="mode"
      @change="onChange"
    >
      <option value="system">跟随系统</option>
      <option value="light">浅色</option>
      <option value="dark">深色</option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useThemeStore, type ThemeMode } from "@/stores/theme";

const theme = useThemeStore();
const mode = computed(() => theme.mode);

const onChange = (event: Event) => {
  const value = (event.target as HTMLSelectElement).value as ThemeMode;
  theme.setMode(value);
};
</script>

<style scoped>
.theme-toggle {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.theme-label {
  font-size: 0.75rem;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  color: var(--color-muted);
}

.theme-select {
  background: var(--color-surface);
  border: 1px solid rgba(148, 163, 184, 0.4);
  border-radius: 0.75rem;
  padding: 0.4rem 0.75rem;
  color: inherit;
  font-size: 0.9rem;
}

.theme-select:focus-visible {
  outline: 2px solid var(--color-accent);
  outline-offset: 2px;
}
</style>
