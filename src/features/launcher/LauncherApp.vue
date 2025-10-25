<template>
  <div ref="shellRef" class="launcher-shell" :class="{ 'has-results': results.length > 0 }" @keydown="handleShortcut">
    <div class="capsule" role="search">
      <span class="capsule-caret" aria-hidden="true" />
      <input
        ref="inputRef"
        :value="query"
        placeholder="输入命令、应用或算式，例如：`1+2`、`tr en 你好`"
        spellcheck="false"
        @input="onInput"
        @keydown="handleKey"
      />
      <button v-if="query" class="clear" type="button" @click="clear" aria-label="清除搜索">✕</button>
      <span class="capsule-badge" aria-hidden="true">
        <span class="capsule-halo"></span>
        <span class="capsule-hat">🎩</span>
      </span>
    </div>

    <ul v-if="results.length" class="result-panel">
      <li
        v-for="(item, index) in results"
        :key="item.entry.id + index"
        :class="['result-row', { active: index === selectedIndex }]"
        @mouseenter="selectedIndex = index"
        @click="() => launch(item)"
      >
        <div class="result-icon">{{ item.entry.icon ?? "🔍" }}</div>
        <div class="result-copy">
          <span class="result-title">
            {{ item.entry.label }}
            <small v-if="item.badge" class="result-badge">{{ item.badge }}</small>
          </span>
          <span class="result-subtitle">{{ item.meta ?? item.entry.description }}</span>
        </div>
        <kbd class="result-hint">{{ index === 0 ? "回车" : `⌥${index + 1}` }}</kbd>
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
  gap: 0.45rem;
  color: #1b1d23;
  min-width: 560px;
}

.capsule {
  position: relative;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.85rem 1.1rem 0.85rem 1.6rem;
  border-radius: 27px;
  background: linear-gradient(180deg, #f0f1f3, #cfd2d7);
  box-shadow:
    inset 0 1px 1px rgba(255, 255, 255, 0.65),
    inset 0 -1px 1px rgba(143, 146, 154, 0.4),
    0 16px 38px rgba(34, 36, 44, 0.3);
}

.capsule::after {
  content: "";
  position: absolute;
  inset: 3px;
  border-radius: 24px;
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.35), rgba(179, 183, 191, 0.18));
  pointer-events: none;
}

.capsule input {
  position: relative;
  z-index: 1;
  flex: 1;
  border: none;
  background: transparent;
  font-size: 1.28rem;
  font-weight: 500;
  color: inherit;
}

.capsule input::placeholder {
  color: rgba(60, 63, 72, 0.5);
}

.capsule input:focus-visible {
  outline: none;
}

.capsule-caret {
  position: relative;
  z-index: 1;
  width: 2px;
  height: 28px;
  border-radius: 1px;
  background: linear-gradient(180deg, #1f2531, #6c707b);
  margin-right: 0.3rem;
}

.clear {
  position: relative;
  z-index: 1;
  width: 28px;
  height: 28px;
  display: grid;
  place-items: center;
  border-radius: 50%;
  border: none;
  background: linear-gradient(180deg, rgba(214, 217, 223, 0.8), rgba(171, 176, 187, 0.95));
  color: rgba(52, 55, 66, 0.75);
  cursor: pointer;
  font-size: 0.8rem;
  display: grid;
  place-items: center;
  margin-left: 0.5rem;
}

.clear:hover {
  background: linear-gradient(180deg, rgba(189, 194, 205, 0.95), rgba(147, 152, 165, 0.95));
}

.capsule-badge {
  position: relative;
  z-index: 1;
  display: grid;
  place-items: center;
  width: 52px;
  height: 52px;
  border-radius: 50%;
  background: linear-gradient(200deg, #29243f, #5d3c9f);
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.25),
    inset 0 -1px 0 rgba(27, 12, 56, 0.55);
}

.capsule-halo {
  position: absolute;
  inset: 10px;
  border-radius: 50%;
  background: radial-gradient(circle at 35% 35%, rgba(255, 255, 255, 0.8), transparent 70%);
  opacity: 0.7;
}

.capsule-hat {
  position: relative;
  font-size: 1.65rem;
  filter: drop-shadow(0 6px 8px rgba(19, 9, 44, 0.45));
}

.result-panel {
  position: relative;
  z-index: 0;
  list-style: none;
  margin: 0;
  padding: 0.35rem;
  border-radius: 18px;
  background: linear-gradient(180deg, rgba(246, 247, 249, 0.96), rgba(215, 218, 226, 0.92));
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.65),
    inset 0 -1px 0 rgba(180, 184, 194, 0.45),
    0 16px 28px rgba(34, 36, 44, 0.18);
  max-height: 320px;
  overflow-y: auto;
}

.result-row {
  display: grid;
  grid-template-columns: 44px 1fr auto;
  align-items: center;
  gap: 0.75rem;
  padding: 0.55rem 0.7rem;
  border-radius: 12px;
  cursor: pointer;
  color: rgba(35, 38, 48, 0.92);
  transition: background 0.12s ease;
}

.result-row.active,
.result-row:hover {
  background: linear-gradient(180deg, rgba(102, 148, 255, 0.22), rgba(71, 129, 255, 0.2));
}

.result-icon {
  width: 44px;
  height: 44px;
  border-radius: 11px;
  display: grid;
  place-items: center;
  font-size: 1.25rem;
  background: linear-gradient(180deg, #f7f8fb, #ccd1dd);
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.8),
    inset 0 -1px 0 rgba(155, 159, 170, 0.55);
}

.result-copy {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.result-title {
  font-weight: 600;
  display: inline-flex;
  gap: 0.4rem;
  align-items: center;
}

.result-badge {
  padding: 0.1rem 0.4rem;
  border-radius: 0.5rem;
  background: rgba(61, 76, 110, 0.22);
  font-size: 0.7rem;
}

.result-subtitle {
  font-size: 0.8rem;
  color: rgba(65, 70, 82, 0.72);
}

.result-hint {
  font-size: 0.75rem;
  padding: 0.2rem 0.45rem;
  border-radius: 0.4rem;
  border: 1px solid rgba(139, 144, 156, 0.4);
  background: rgba(255, 255, 255, 0.65);
  color: rgba(59, 64, 78, 0.7);
}

.empty {
  margin: 0;
  padding: 0.85rem 1.4rem;
  text-align: center;
  color: rgba(59, 64, 78, 0.7);
  border-radius: 16px;
  background: linear-gradient(180deg, rgba(246, 247, 249, 0.96), rgba(215, 218, 226, 0.92));
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.6),
    inset 0 -1px 0 rgba(182, 187, 198, 0.45),
    0 12px 24px rgba(34, 36, 44, 0.16);
}

@media (max-width: 600px) {
  .launcher-shell {
    min-width: auto;
    width: min(96vw, 560px);
  }
}
</style>
