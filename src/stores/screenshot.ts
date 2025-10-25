import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { reactive } from "vue";

export interface ScreenshotEntry {
  id: string;
  data_url: string;
  created_at: string;
  pinned: boolean;
  note?: string | null;
}

interface ScreenshotState {
  items: ScreenshotEntry[];
  loading: boolean;
  capturing: boolean;
}

const sortScreens = (items: ScreenshotEntry[]) =>
  [...items].sort((a, b) => {
    if (a.pinned !== b.pinned) {
      return a.pinned ? -1 : 1;
    }
    return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
  });

export const useScreenshotStore = defineStore("screenshots", () => {
  const state = reactive<ScreenshotState>({
    items: [],
    loading: false,
    capturing: false,
  });

  const load = async () => {
    state.loading = true;
    try {
      const data = await invoke<ScreenshotEntry[]>("screenshot_list");
      state.items = sortScreens(data);
    } finally {
      state.loading = false;
    }
  };

  const capture = async (note?: string) => {
    state.capturing = true;
    try {
      const shot = await invoke<ScreenshotEntry>("screenshot_capture", { note });
      state.items = sortScreens([shot, ...state.items]);
    } finally {
      state.capturing = false;
    }
  };

  const togglePin = async (entry: ScreenshotEntry) => {
    await invoke("screenshot_set_pin", { id: entry.id, pinned: !entry.pinned });
    state.items = sortScreens(
      state.items.map((item) =>
        item.id === entry.id ? { ...item, pinned: !item.pinned } : item
      )
    );
  };

  const remove = async (entry: ScreenshotEntry) => {
    await invoke("screenshot_remove", { id: entry.id });
    state.items = state.items.filter((item) => item.id != entry.id);
  };

  return {
    state,
    load,
    capture,
    togglePin,
    remove,
  };
});
