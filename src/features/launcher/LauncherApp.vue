<template>
  <div ref="shellRef" class="launcher-shell" @keydown="handleShortcut">
    <div class="launcher-inner">
      <header class="search-row">
        <div class="search-emblem">🎩</div>
        <div class="search-field">
          <input
            ref="inputRef"
            :value="query"
            placeholder="输入命令、应用或算式，例如：`1+2`、`tr en 你好`"
            spellcheck="false"
            @input="onInput"
            @keydown="handleKey"
          />
          <button v-if="query" class="clear-btn" @click="clear" aria-label="清除搜索">✕</button>
        </div>
      </header>

      <ul v-if="results.length" class="result-list">
        <li
          v-for="(item, index) in results"
          :key="item.entry.id + index"
          :class="['result-item', { active: index === selectedIndex }]"
          @mouseenter="selectedIndex = index"
          @click="() => launch(item)"
        >
          <div class="icon">{{ item.entry.icon ?? "🔍" }}</div>
          <div class="content">
            <span class="title">
              {{ item.entry.label }}
              <small v-if="item.badge" class="badge">{{ item.badge }}</small>
            </span>
            <span class="subtitle">{{ item.meta ?? item.entry.description }}</span>
          </div>
          <kbd class="hint">{{ index === 0 ? "回车" : `⌥${index + 1}` }}</kbd>
        </li>
      </ul>

      <p v-else-if="query.trim().length" class="empty">未找到匹配项，请尝试其他关键字。</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { useLauncherSearch } from "./useSearch";
import type { LauncherResult } from "./types";

const { query, results, setQuery, clearQuery, highlighted } = useLauncherSearch();

const selectedIndex = ref(0);
const inputRef = ref<HTMLInputElement>();
const shellRef = ref<HTMLElement>();
const windowHandle = getCurrentWindow();
const isTauri = typeof window !== "undefined" && "__TAURI_IPC__" in window;

const syncSelection = () => {
  if (selectedIndex.value >= results.value.length) {
    selectedIndex.value = results.value.length - 1;
  }
  if (selectedIndex.value < 0) {
    selectedIndex.value = 0;
  }
};

const launch = async (item: LauncherResult | null) => {
  if (!item) return;

  if (item.entry.id === "calculator" && item.data?.result) {
    await invoke("plugin:clipboard-manager|write_text", {
      text: String(item.data.result)
    });
  }

  try {
    await invoke("execute_entry", {
      entryId: item.entry.id,
      payload: item.data ?? {}
    });
  } catch (error) {
    console.warn("Launcher invocation failed", error);
  }

  await windowHandle.hide();
};

const submit = () => {
  const item = results.value[selectedIndex.value] ?? highlighted.value;
  void launch(item ?? null);
};

const onInput = (event: Event) => {
  const value = (event.target as HTMLInputElement).value;
  setQuery(value);
  selectedIndex.value = 0;
  void adjustWindowSize();
};

const clear = () => {
  clearQuery();
  if (inputRef.value) {
    inputRef.value.value = "";
    inputRef.value.focus();
  }
  selectedIndex.value = 0;
  void adjustWindowSize();
};

const handleKey = (event: KeyboardEvent) => {
  if (event.key === "ArrowDown") {
    event.preventDefault();
    selectedIndex.value = Math.min(
      results.value.length - 1,
      selectedIndex.value + 1
    );
    return;
  }

  if (event.key === "ArrowUp") {
    event.preventDefault();
    selectedIndex.value = Math.max(0, selectedIndex.value - 1);
    return;
  }

  if (event.key === "Enter") {
    event.preventDefault();
    submit();
  }

  if (event.key === "Escape") {
    event.preventDefault();
    void windowHandle.hide();
  }
};

const handleShortcut = (event: KeyboardEvent) => {
  if (event.altKey) {
    const numeric = Number.parseInt(event.key, 10);
    if (Number.isInteger(numeric) && numeric > 0 && numeric <= results.value.length) {
      event.preventDefault();
      const index = numeric - 1;
      selectedIndex.value = index;
      submit();
    }
  }
};

const adjustWindowSize = async () => {
  if (!isTauri) {
    return;
  }
  await nextTick();
  const element = shellRef.value;
  if (!element) {
    return;
  }
  const rect = element.getBoundingClientRect();
  try {
    await windowHandle.setSize(new LogicalSize(Math.ceil(rect.width), Math.ceil(rect.height)));
    await windowHandle.setMinSize(new LogicalSize(Math.ceil(rect.width), Math.ceil(rect.height)));
  } catch (error) {
    console.warn("Failed to resize launcher window", error);
  }
};

let resizeObserver: ResizeObserver | undefined;

watch(results, async () => {
  syncSelection();
  await adjustWindowSize();
});

watch(query, () => {
  selectedIndex.value = 0;
  void adjustWindowSize();
});

onMounted(() => {
  if (inputRef.value) {
    inputRef.value.focus();
  }
  setQuery(query.value);
  syncSelection();
  if (typeof ResizeObserver !== "undefined" && shellRef.value && isTauri) {
    resizeObserver = new ResizeObserver(() => {
      void adjustWindowSize();
    });
    resizeObserver.observe(shellRef.value);
  } else {
    void adjustWindowSize();
  }
});

onBeforeUnmount(() => {
  resizeObserver?.disconnect();
});

syncSelection();
</script>

<style scoped>
.launcher-shell {
  width: 640px;
  max-width: 90vw;
  background: linear-gradient(165deg, rgba(40, 42, 56, 0.98), rgba(24, 25, 38, 0.96));
  color: #f8fafc;
  border-radius: 22px;
  box-shadow: 0 32px 80px rgba(10, 11, 21, 0.55);
  backdrop-filter: blur(28px);
  padding: 0.5rem;
}

.launcher-inner {
  display: flex;
  flex-direction: column;
  background: rgba(15, 16, 28, 0.4);
  border-radius: 18px;
  overflow: hidden;
  border: 1px solid rgba(148, 163, 184, 0.24);
}

.search-row {
  display: grid;
  grid-template-columns: 72px 1fr;
  align-items: center;
  padding: 0.9rem 1.1rem 0.8rem;
  gap: 1rem;
  background: rgba(17, 19, 32, 0.85);
  border-bottom: 1px solid rgba(99, 102, 241, 0.25);
}

.search-emblem {
  width: 54px;
  height: 54px;
  border-radius: 16px;
  display: grid;
  place-items: center;
  font-size: 1.85rem;
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.65), rgba(14, 165, 233, 0.45));
  color: #0b1020;
  font-weight: 700;
}

.search-field {
  position: relative;
  display: flex;
  align-items: center;
}

input {
  flex: 1;
  background: rgba(15, 15, 23, 0.8);
  border: 1px solid rgba(148, 163, 184, 0.35);
  border-radius: 14px;
  padding: 0.95rem 3rem 0.95rem 1.2rem;
  color: inherit;
  font-size: 1.15rem;
  transition: border 0.15s ease, box-shadow 0.15s ease;
}

input:focus-visible {
  outline: none;
  border-color: rgba(129, 140, 248, 0.9);
  box-shadow: 0 0 0 2px rgba(129, 140, 248, 0.35);
}

.clear-btn {
  position: absolute;
  right: 0.9rem;
  width: 1.75rem;
  height: 1.75rem;
  border-radius: 50%;
  border: none;
  background: rgba(51, 65, 85, 0.65);
  color: rgba(226, 232, 240, 0.82);
  cursor: pointer;
  font-size: 0.85rem;
}

.clear-btn:hover {
  background: rgba(99, 102, 241, 0.65);
}

.result-list {
  list-style: none;
  margin: 0;
  padding: 0.35rem 0.4rem 0.55rem;
  max-height: 360px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.result-item {
  display: grid;
  grid-template-columns: 48px 1fr auto;
  align-items: center;
  gap: 0.85rem;
  padding: 0.65rem 0.75rem;
  border-radius: 12px;
  cursor: pointer;
  transition: background 0.12s ease, transform 0.12s ease;
}

.result-item:hover,
.result-item.active {
  background: rgba(99, 102, 241, 0.28);
  transform: translateY(-1px);
}

.icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: grid;
  place-items: center;
  font-size: 1.35rem;
  background: rgba(148, 163, 184, 0.28);
  color: rgba(226, 232, 240, 0.95);
}

.content {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.title {
  font-weight: 600;
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.badge {
  font-size: 0.7rem;
  padding: 0.1rem 0.4rem;
  border-radius: 0.6rem;
  background: rgba(15, 23, 42, 0.55);
}

.subtitle {
  font-size: 0.82rem;
  color: rgba(226, 232, 240, 0.72);
}

.hint {
  font-size: 0.75rem;
  padding: 0.25rem 0.5rem;
  border-radius: 0.45rem;
  border: 1px solid rgba(148, 163, 184, 0.35);
  color: rgba(226, 232, 240, 0.7);
}

.empty {
  margin: 0;
  padding: 1.5rem;
  text-align: center;
  color: rgba(226, 232, 240, 0.7);
}
</style>
