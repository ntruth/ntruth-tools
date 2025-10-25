import { beforeEach, describe, expect, it, vi } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useClipboardStore } from "./clipboard";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockImplementation(async (cmd: string) => {
    if (cmd === "plugin:clipboard-manager|read_text") {
      return "";
    }
    return [];
  })
}));

describe("useClipboardStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("filters items by type and keyword", () => {
    const store = useClipboardStore();
    store.state.items = [
      {
        id: "1",
        type: "text",
        content: "https://unitools.dev",
        created_at: new Date().toISOString(),
        pinned: false,
        tags: ["link"]
      },
      {
        id: "2",
        type: "text",
        content: "Alt + Space",
        created_at: new Date().toISOString(),
        pinned: true,
        tags: ["hotkey"]
      }
    ];

    store.setFilter("text");
    store.setSearch("link");

    expect(store.filteredItems).toHaveLength(1);
    expect(store.filteredItems[0].tags).toContain("link");
  });
});
