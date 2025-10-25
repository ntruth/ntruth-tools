import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { computed, reactive, ref } from "vue";
import type { ClipboardEntry, ClipboardKind } from "@/features/clipboard/types";
import { useSettingsStore } from "@/stores/settings";

interface ClipboardState {
  loading: boolean;
  items: ClipboardEntry[];
  search: string;
  filter: ClipboardKind | "all";
}

const sortEntries = (entries: ClipboardEntry[]) =>
  [...entries].sort((a, b) => {
    if (a.pinned !== b.pinned) {
      return a.pinned ? -1 : 1;
    }
    return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
  });

export const useClipboardStore = defineStore("clipboard", () => {
  const settings = useSettingsStore();
  const state = reactive<ClipboardState>({
    loading: false,
    items: [],
    search: "",
    filter: "all",
  });

  const fetchHistory = async () => {
    state.loading = true;
    try {
      const result = await invoke<ClipboardEntry[]>("clipboard_get_history");
      state.items = sortEntries(result);
    } finally {
      state.loading = false;
    }
  };

  const captureCurrent = async () => {
    if (!settings.state.features.clipboardHistory) {
      return;
    }
    const text = await invoke<string>("plugin:clipboard-manager|read_text");
    if (!text) {
      return;
    }
    const saved = await invoke<ClipboardEntry>("clipboard_save_entry", {
      payload: { content: text, type: "text" }
    });
    state.items = sortEntries([saved, ...state.items]);
  };

  const togglePin = async (entry: ClipboardEntry) => {
    await invoke("clipboard_set_pin", { id: entry.id, pinned: !entry.pinned });
    state.items = state.items.map((item) =>
      item.id === entry.id ? { ...item, pinned: !item.pinned } : item
    );
    state.items = sortEntries(state.items);
  };

  const remove = async (entry: ClipboardEntry) => {
    await invoke("clipboard_remove", { id: entry.id });
    state.items = state.items.filter((item) => item.id !== entry.id);
  };

  const clearUnpinned = async () => {
    await invoke("clipboard_clear_unpinned");
    state.items = state.items.filter((item) => item.pinned);
  };

  const copyToClipboard = async (entry: ClipboardEntry) => {
    if (entry.type === "text") {
      await invoke("plugin:clipboard-manager|write_text", { text: entry.content });
    }
  };

  const filteredItems = computed(() => {
    const keyword = state.search.trim().toLowerCase();
    return state.items.filter((item) => {
      if (state.filter !== "all" && item.type !== state.filter) {
        return false;
      }
      if (!keyword) {
        return true;
      }
      return (
        item.content.toLowerCase().includes(keyword) ||
        item.tags.some((tag) => tag.toLowerCase().includes(keyword))
      );
    });
  });

  const setSearch = (value: string) => {
    state.search = value;
  };

  const setFilter = (value: ClipboardKind | "all") => {
    state.filter = value;
  };

  const isPanelVisible = ref(false);

  const togglePanel = (value?: boolean) => {
    if (!settings.state.features.clipboardHistory) {
      isPanelVisible.value = false;
      return;
    }
    isPanelVisible.value = value ?? !isPanelVisible.value;
  };

  return {
    state,
    isPanelVisible,
    filteredItems,
    fetchHistory,
    captureCurrent,
    togglePin,
    remove,
    clearUnpinned,
    copyToClipboard,
    setSearch,
    setFilter,
    togglePanel,
  };
});
