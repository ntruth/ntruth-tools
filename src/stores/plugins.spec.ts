import { beforeEach, describe, expect, it, vi } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { usePluginStore } from "./plugins";
import type { PluginManifest } from "@/features/plugins/types";

const marketplace: PluginManifest[] = [
  {
    id: "plugin-a",
    name: "Plugin A",
    version: "1.0.0",
    author: "Tester",
    summary: "desc",
    category: "tools",
  },
];

const installed: PluginManifest[] = [
  {
    id: "plugin-b",
    name: "Plugin B",
    version: "1.0.0",
    author: "Tester",
    summary: "desc",
    category: "tools",
  },
];

const invokeMock = vi.hoisted(() =>
  vi.fn((cmd: string) => {
    switch (cmd) {
      case "plugin_marketplace":
        return Promise.resolve(marketplace);
      case "plugin_installed":
        return Promise.resolve(installed);
      case "plugin_install":
        return Promise.resolve(marketplace[0]);
      case "plugin_uninstall":
        return Promise.resolve(null);
      default:
        return Promise.resolve(null);
    }
  })
);

vi.mock("@tauri-apps/api/core", () => ({
  invoke: invokeMock,
}));

describe("usePluginStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    invokeMock.mockClear();
  });

  it("refreshes marketplace and installed lists", async () => {
    const store = usePluginStore();
    await store.refresh();
    expect(store.state.marketplace).toHaveLength(1);
    expect(store.state.installed).toHaveLength(1);
  });

  it("invokes install command", async () => {
    const store = usePluginStore();
    await store.install("plugin-a");
    expect(invokeMock).toHaveBeenCalledWith("plugin_install", { request: { id: "plugin-a" } });
  });
});
