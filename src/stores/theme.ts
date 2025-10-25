import { defineStore } from "pinia";
import { onUnmounted, ref, watch } from "vue";

export type ThemeMode = "light" | "dark" | "system";

const STORAGE_KEY = "unitools.theme";
const MEDIA_QUERY = "(prefers-color-scheme: dark)";

export const useThemeStore = defineStore("theme", () => {
  const mode = ref<ThemeMode>("system");
  const isDark = ref(false);
  const media = typeof window !== "undefined" ? window.matchMedia(MEDIA_QUERY) : undefined;

  const persist = (value: ThemeMode) => {
    if (typeof window === "undefined") {
      return;
    }
    localStorage.setItem(STORAGE_KEY, value);
  };

  const computeIsDark = () => {
    if (mode.value === "dark") {
      return true;
    }
    if (mode.value === "light") {
      return false;
    }
    return media?.matches ?? false;
  };

  const applyTheme = () => {
    if (typeof document === "undefined") {
      return;
    }
    isDark.value = computeIsDark();
    document.documentElement.classList.toggle("dark", isDark.value);
  };

  const setMode = (value: ThemeMode) => {
    mode.value = value;
    persist(value);
  };

  const loadInitial = () => {
    if (typeof window === "undefined") {
      return;
    }
    const stored = window.localStorage.getItem(STORAGE_KEY) as ThemeMode | null;
    if (stored === "light" || stored === "dark" || stored === "system") {
      mode.value = stored;
    }
  };

  if (typeof window !== "undefined") {
    loadInitial();
    applyTheme();
    const stopWatch = watch(mode, () => applyTheme(), { immediate: false });
    const handleSystem = () => applyTheme();
    media?.addEventListener("change", handleSystem);
    onUnmounted(() => {
      stopWatch();
      media?.removeEventListener("change", handleSystem);
    });
  }

  return {
    mode,
    isDark,
    setMode,
    apply: applyTheme
  };
});
