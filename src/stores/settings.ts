import { defineStore } from "pinia";
import { ref, watch } from "vue";

type FeatureKey =
  | "defaultResults"
  | "fileSearch"
  | "universalActions"
  | "webSearch"
  | "webBookmarks"
  | "clipboardHistory"
  | "snippets"
  | "calculator";

type GeneralKey = "launchAtLogin" | "showTrayIcon" | "autoLaunchSearch";

interface SettingsState {
  general: Record<GeneralKey, boolean>;
  features: Record<FeatureKey, boolean>;
  webFallbacks: string[];
}

const STORAGE_KEY = "unitools.settings";

const defaultState: SettingsState = {
  general: {
    launchAtLogin: false,
    showTrayIcon: true,
    autoLaunchSearch: true
  },
  features: {
    defaultResults: true,
    fileSearch: true,
    universalActions: true,
    webSearch: true,
    webBookmarks: false,
    clipboardHistory: true,
    snippets: false,
    calculator: true
  },
  webFallbacks: ["https://www.google.com/search?q=%s", "https://www.dictionary.com/browse/%s"]
};

const loadState = (): SettingsState => {
  if (typeof localStorage === "undefined") {
    return structuredClone(defaultState);
  }
  const raw = localStorage.getItem(STORAGE_KEY);
  if (!raw) {
    return structuredClone(defaultState);
  }
  try {
    const parsed = JSON.parse(raw) as SettingsState;
    return {
      ...structuredClone(defaultState),
      ...parsed,
      general: { ...defaultState.general, ...parsed.general },
      features: { ...defaultState.features, ...parsed.features }
    };
  } catch (error) {
    console.warn("Failed to parse settings", error);
    return structuredClone(defaultState);
  }
};

export const useSettingsStore = defineStore("settings", () => {
  const state = ref<SettingsState>(loadState());

  watch(
    state,
    (value) => {
      if (typeof localStorage !== "undefined") {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
      }
    },
    { deep: true }
  );

  const toggleGeneral = (key: GeneralKey, value: boolean) => {
    state.value.general[key] = value;
  };

  const toggleFeature = (key: FeatureKey, value: boolean) => {
    state.value.features[key] = value;
  };

  const setFallbacks = (entries: string[]) => {
    state.value.webFallbacks = entries;
  };

  return {
    state,
    toggleGeneral,
    toggleFeature,
    setFallbacks
  };
});
