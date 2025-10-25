<template>
  <div ref="shellRef" class="launcher-shell" @keydown="handleShortcut">
    <header class="search-bar">
      <input
        ref="inputRef"
        :value="query"
        placeholder="输入命令、应用或算式，例如：`1+2`、`tr en 你好`"
        spellcheck="false"
        @input="onInput"
        @keydown="handleKey"
      />
      <button v-if="query" class="clear-btn" @click="clear">
        ✕
      </button>
    </header>

    <section class="results" v-show="query.trim().length > 0">
      <div
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
          <span class="subtitle">
            {{ item.meta ?? item.entry.description }}
          </span>
        </div>
        <kbd class="hint">{{ index === 0 ? "Enter" : `Alt+${index + 1}` }}</kbd>
      </div>

      <p v-if="!results.length" class="empty">未找到匹配项，请尝试其他关键字。</p>
    </section>

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
  width: 620px;
  max-width: 90vw;
  background: rgba(15, 23, 42, 0.9);
  color: #f8fafc;
  border-radius: 18px;
  padding: 1.1rem;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  box-shadow: 0 30px 90px rgba(15, 23, 42, 0.45);
  backdrop-filter: blur(22px);
}

.search-bar {
  position: relative;
  display: flex;
  align-items: center;
}

input {
  flex: 1;
  background: rgba(15, 23, 42, 0.65);
  border: 1px solid rgba(129, 140, 248, 0.35);
  border-radius: 14px;
  padding: 0.9rem 1.2rem;
  color: inherit;
  font-size: 1.1rem;
}

input:focus-visible {
  outline: 2px solid rgba(129, 140, 248, 0.8);
  outline-offset: 2px;
}

.clear-btn {
  position: absolute;
  right: 0.5rem;
  background: none;
  border: none;
  color: rgba(226, 232, 240, 0.7);
  cursor: pointer;
  font-size: 0.85rem;
}


.results {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  max-height: 320px;
  overflow-y: auto;
  padding: 0.35rem 0;
}

.result-item {
  display: flex;
  align-items: center;
  gap: 0.85rem;
  padding: 0.75rem 0.85rem;
  border-radius: 0.85rem;
  transition: background 0.15s ease;
  cursor: pointer;
}

.result-item.active {
  background: rgba(99, 102, 241, 0.25);
}

.icon {
  width: 2.5rem;
  height: 2.5rem;
  display: grid;
  place-items: center;
  font-size: 1.4rem;
  background: rgba(148, 163, 184, 0.18);
  border-radius: 0.75rem;
}

.content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
}

.title {
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.badge {
  font-size: 0.7rem;
  padding: 0.1rem 0.4rem;
  border-radius: 0.5rem;
  background: rgba(148, 163, 184, 0.3);
}

.subtitle {
  font-size: 0.85rem;
  color: rgba(226, 232, 240, 0.7);
}

.hint {
  font-size: 0.75rem;
  padding: 0.2rem 0.45rem;
  border-radius: 0.45rem;
  border: 1px solid rgba(148, 163, 184, 0.4);
  color: rgba(226, 232, 240, 0.6);
}

.empty {
  margin: 0;
  padding: 1.2rem 0;
  text-align: center;
  color: rgba(226, 232, 240, 0.65);
}
</style>
