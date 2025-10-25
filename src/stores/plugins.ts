import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { reactive } from "vue";
import type { PluginManifest } from "@/features/plugins/types";

interface PluginState {
  marketplace: PluginManifest[];
  installed: PluginManifest[];
  loading: boolean;
}

export const usePluginStore = defineStore("plugins", () => {
  const state = reactive<PluginState>({
    marketplace: [],
    installed: [],
    loading: false,
  });

  const refresh = async () => {
    state.loading = true;
    try {
      const [market, installed] = await Promise.all([
        invoke<PluginManifest[]>("plugin_marketplace"),
        invoke<PluginManifest[]>("plugin_installed"),
      ]);
      state.marketplace = market;
      state.installed = installed;
    } finally {
      state.loading = false;
    }
  };

  const install = async (id: string) => {
    await invoke<PluginManifest>("plugin_install", { request: { id } });
    await refresh();
  };

  const uninstall = async (id: string) => {
    await invoke("plugin_uninstall", { id });
    await refresh();
  };

  return {
    state,
    refresh,
    install,
    uninstall,
  };
});
