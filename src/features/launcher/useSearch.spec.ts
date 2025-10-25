import { describe, expect, it, vi, beforeEach } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useSettingsStore } from "@/stores/settings";
import { useLauncherSearch } from "./useSearch";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn((command: string) => {
    if (command === "search_files") {
      return Promise.resolve([]);
    }
    return Promise.resolve("mock translation");
  })
}));

const flushPromises = () => new Promise((resolve) => setTimeout(resolve, 0));

describe("useLauncherSearch", () => {
  beforeEach(() => {
    Reflect.deleteProperty(window as unknown as Record<string, unknown>, "__TAURI_IPC__");
    setActivePinia(createPinia());
    const settings = useSettingsStore();
    settings.state.features.defaultResults = true;
    settings.state.features.fileSearch = false;
    settings.state.features.calculator = true;
    settings.state.features.webSearch = true;
  });

  it("prioritises calculator expressions", () => {
    const { setQuery, results } = useLauncherSearch();
    setQuery("=1+2*5");

    expect(results.value[0]?.entry.id).toBe("calculator");
    expect(results.value[0]?.meta).toContain("11");
  });

  it("triggers translation workflow", async () => {
    const { setQuery, results } = useLauncherSearch();

    setQuery("tr en 你好");
    await flushPromises();

    const translation = results.value.find((item) => item.entry.id === "translator");
    expect(translation).toBeDefined();
    expect(translation?.meta).toBe("mock translation");
  });

  it("scores application keywords", () => {
    const { setQuery, results } = useLauncherSearch();
    setQuery("notepad");

    const first = results.value[0];
    expect(first.entry.id).toBe("text-editor");
  });
});
