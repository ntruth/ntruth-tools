<template>
  <div ref="shellRef" class="launcher-shell" :class="{ 'has-results': results.length > 0 }" @keydown="handleShortcut">
    <div class="search-surface">
      <div class="search-track">
        <span class="search-icon" aria-hidden="true">🔍</span>
        <input
          ref="inputRef"
          :value="query"
          placeholder="输入命令、应用或算式，例如：`1+2`、`tr en 你好`"
          spellcheck="false"
          @input="onInput"
          @keydown="handleKey"
        />
        <button v-if="query" class="clear-btn" @click="clear" aria-label="清除搜索">✕</button>
        <span class="search-badge" aria-hidden="true">
          <span class="badge-hat">🎩</span>
        </span>
      </div>
    </div>

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
  position: relative;
  display: inline-flex;
  flex-direction: column;
  color: #0c0d12;
  background: linear-gradient(145deg, #fcfcfe, #d8dbe6);
  border-radius: 26px;
  box-shadow:
    0 22px 40px rgba(28, 30, 41, 0.28),
    inset 0 1px 0 rgba(255, 255, 255, 0.6),
    inset 0 -1px 0 rgba(86, 88, 104, 0.32);
  padding: 0.65rem;
  min-width: 580px;
}

.launcher-shell.has-results {
  border-bottom-left-radius: 18px;
  border-bottom-right-radius: 18px;
}

.search-surface {
  border-radius: 20px;
  background: linear-gradient(160deg, rgba(255, 255, 255, 0.92), rgba(230, 233, 244, 0.9));
  padding: 0.35rem;
}

.search-track {
  position: relative;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  border-radius: 16px;
  padding: 0.85rem 1.2rem;
  background: linear-gradient(145deg, #eef0f7, #f7f8fc);
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.9),
    inset 0 -1px 0 rgba(201, 205, 216, 0.8);
}

.search-icon {
  width: 32px;
  height: 32px;
  border-radius: 10px;
  display: grid;
  place-items: center;
  background: linear-gradient(140deg, rgba(203, 211, 220, 0.9), rgba(175, 183, 197, 0.85));
  color: rgba(39, 45, 58, 0.85);
  font-size: 1rem;
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.75),
    inset 0 -1px 0 rgba(130, 135, 150, 0.6);
}

input {
  flex: 1;
  background: transparent;
  border: none;
  color: inherit;
  font-size: 1.2rem;
  line-height: 1.2;
  padding: 0;
}

input:focus-visible {
  outline: none;
  color: #07080f;
}

.clear-btn {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  border: none;
  background: linear-gradient(140deg, rgba(201, 205, 216, 0.7), rgba(176, 182, 195, 0.8));
  color: rgba(55, 62, 80, 0.9);
  cursor: pointer;
  font-size: 0.8rem;
  display: grid;
  place-items: center;
  margin-left: 0.5rem;
}

.clear-btn:hover {
  background: linear-gradient(140deg, rgba(166, 171, 188, 0.9), rgba(139, 145, 166, 0.9));
}

.search-badge {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  border-radius: 14px;
  background: linear-gradient(150deg, rgba(66, 30, 117, 0.85), rgba(118, 52, 222, 0.95));
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.35),
    inset 0 -1px 0 rgba(49, 16, 112, 0.65);
  margin-left: auto;
}

.badge-hat {
  font-size: 1.65rem;
  filter: drop-shadow(0 4px 6px rgba(33, 16, 76, 0.45));
}

.result-list {
  list-style: none;
  margin: 0;
  padding: 0.4rem 0.25rem 0.5rem;
  max-height: 320px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  background: rgba(255, 255, 255, 0.92);
  border-radius: 18px;
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.6),
    inset 0 -1px 0 rgba(208, 213, 222, 0.55);
}

.result-item {
  display: grid;
  grid-template-columns: 48px 1fr auto;
  align-items: center;
  gap: 0.85rem;
  padding: 0.6rem 0.75rem;
  border-radius: 12px;
  cursor: pointer;
  transition: background 0.12s ease, transform 0.12s ease;
}

.result-item:hover,
.result-item.active {
  background: linear-gradient(145deg, rgba(138, 143, 168, 0.22), rgba(177, 183, 206, 0.28));
  transform: translateY(-1px);
}

.icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: grid;
  place-items: center;
  font-size: 1.35rem;
  background: linear-gradient(140deg, rgba(210, 214, 226, 0.95), rgba(187, 192, 209, 0.9));
  color: rgba(47, 53, 70, 0.9);
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
  font-size: 0.8rem;
  color: rgba(70, 77, 96, 0.78);
}

.hint {
  font-size: 0.75rem;
  padding: 0.25rem 0.5rem;
  border-radius: 0.45rem;
  border: 1px solid rgba(153, 162, 183, 0.45);
  color: rgba(62, 70, 88, 0.7);
}

.empty {
  margin: 0;
  padding: 1.1rem 1.75rem 1.35rem;
  text-align: center;
  color: rgba(62, 70, 88, 0.72);
  background: rgba(255, 255, 255, 0.92);
  border-radius: 16px;
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.55),
    inset 0 -1px 0 rgba(210, 214, 225, 0.5);
}
</style>
