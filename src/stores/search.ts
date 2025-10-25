import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { reactive } from "vue";

export interface FileSearchResult {
  document: {
    id: string;
    name: string;
    path: string;
    ext?: string | null;
    size: number;
    modified: string;
  };
  score: number;
  highlight?: string | null;
}

interface SearchState {
  query: string;
  loading: boolean;
  results: FileSearchResult[];
}

export const useFileSearchStore = defineStore("file-search", () => {
  const state = reactive<SearchState>({
    query: "",
    loading: false,
    results: [],
  });

  const runSearch = async (query?: string) => {
    if (query !== undefined) {
      state.query = query;
    }
    state.loading = true;
    try {
      state.results = await invoke<FileSearchResult[]>("search_files", {
        query: state.query
      });
    } finally {
      state.loading = false;
    }
  };

  const refreshIndex = async () => {
    await invoke("search_refresh");
    await runSearch(state.query);
  };

  return {
    state,
    runSearch,
    refreshIndex,
  };
});
