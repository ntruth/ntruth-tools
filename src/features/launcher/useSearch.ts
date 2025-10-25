import { computed, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { LAUNCHER_ENTRIES, TOOL_ENTRIES } from "./search-data";
import type { LauncherEntry, LauncherResult } from "./types";
import { useSettingsStore } from "@/stores/settings";
import { watch } from "vue";

interface TranslationPayload {
  text: string;
  target: string;
}

const MAX_RESULTS = 8;
const CALC_PATTERN = /^=?\s*([-+/*()\d\s.,^%]+)$/;
const TRANSLATE_PATTERN = /^(?:tr|translate)\s+(?<target>[a-z]{2})\s+(?<text>.+)$/i;

const isTauri = () => typeof window !== "undefined" && "__TAURI_IPC__" in window;

const calculatorEntry = TOOL_ENTRIES.find((item) => item.id === "calculator")!;
const translatorEntry = TOOL_ENTRIES.find((item) => item.id === "translator")!;

const translationCache = new Map<string, string>();
const FALLBACK_ICON = "🌍";
interface BackendSearchResult {
  document: {
    id: string;
    name: string;
    path: string;
    ext?: string | null;
  };
  score: number;
  highlight?: string | null;
}

const localeToLabel: Record<string, string> = {
  en: "英文",
  zh: "中文",
  ja: "日文",
  ko: "韩文"
};

const evaluateExpression = (raw: string): string | null => {
  const sanitized = raw.replace(/[^0-9+/*().,\-%^ ]/g, "");
  if (!sanitized.trim()) {
    return null;
  }

  try {
    const normalized = sanitized.replace(/%/g, "/100");
    // eslint-disable-next-line no-new-func
    const result = Function(`"use strict"; return (${normalized});`)();
    if (typeof result === "number" && Number.isFinite(result)) {
      return Number(result.toFixed(6)).toString();
    }
  } catch (error) {
    console.warn("Failed to evaluate expression", error);
  }

  return null;
};

const translateOffline = (payload: TranslationPayload): string => {
  const map: Record<string, Record<string, string>> = {
    en: {
      你好: "hello",
      谢谢: "thank you",
      剪贴板: "clipboard",
      搜索: "search"
    },
    zh: {
      hello: "你好",
      "thank you": "谢谢",
      clipboard: "剪贴板",
      search: "搜索"
    }
  };

  const dictionary = map[payload.target];
  if (!dictionary) {
    return payload.text;
  }

  return (
    dictionary[payload.text.toLowerCase() as keyof typeof dictionary] ?? payload.text
  );
};

const performTranslation = async (payload: TranslationPayload) => {
  if (payload.text.trim() === "") {
    return "";
  }

  const cacheKey = `${payload.target}:${payload.text}`;
  const cached = translationCache.get(cacheKey);
  if (cached) {
    return cached;
  }

  try {
    const result = await invoke<string>("translate_text", payload);
    translationCache.set(cacheKey, result);
    return result;
  } catch (error) {
    if (isTauri()) {
      console.warn("Translation fallback triggered", error);
    }
  }

  const fallback = translateOffline(payload);
  translationCache.set(cacheKey, fallback);
  return fallback;
};

const normalize = (value: string) =>
  value
    .normalize("NFD")
    .replace(/\p{Diacritic}/gu, "")
    .toLowerCase();

const scoreEntry = (entry: LauncherEntry, query: string) => {
  if (!query) {
    return entry.weight ?? 50;
  }

  const normalizedQuery = normalize(query);
  const label = normalize(entry.label);
  const keywords = entry.keywords.map(normalize);

  if (label === normalizedQuery) {
    return 1000 + (entry.weight ?? 0);
  }

  if (label.startsWith(normalizedQuery)) {
    return 850 + (entry.weight ?? 0);
  }

  if (keywords.some((keyword) => keyword.startsWith(normalizedQuery))) {
    return 800 + (entry.weight ?? 0);
  }

  if (label.includes(normalizedQuery)) {
    return 700 + (entry.weight ?? 0) - (label.length - normalizedQuery.length);
  }

  const keywordMatches = keywords.filter((keyword) => keyword.includes(normalizedQuery));
  if (keywordMatches.length > 0) {
    return (
      650 +
      (entry.weight ?? 0) -
      10 * (keywordMatches[0].length - normalizedQuery.length)
    );
  }

  const tokens = normalizedQuery.split(/\s+/);
  const coverage = tokens.filter((token) => label.includes(token)).length;
  if (coverage > 0) {
    return 600 + coverage * 10 + (entry.weight ?? 0);
  }

  return 0;
};

export const useLauncherSearch = () => {
  const settings = useSettingsStore();
  const query = ref("");
  const loading = ref(false);
  const error = ref<string | null>(null);
  const results = ref<LauncherResult[]>([]);
  const fileMatches = ref<LauncherResult[]>([]);
  const baseMatches = ref<LauncherResult[]>([]);
  const webFallbackMatches = ref<LauncherResult[]>([]);

  const context = reactive({
    calculator: null as LauncherResult | null,
    translator: null as LauncherResult | null
  });

  watch(
    () => ({ ...settings.state.features }),
    () => {
      if (!settings.state.features.fileSearch) {
        fileMatches.value = [];
      }
      setQuery(query.value);
    },
    { deep: true }
  );

  const applyResults = () => {
    const aggregated: LauncherResult[] = [];
    if (context.calculator) {
      aggregated.push(context.calculator);
    }
    if (context.translator) {
      aggregated.push(context.translator);
    }
    aggregated.push(...baseMatches.value);
    aggregated.push(...fileMatches.value);
    aggregated.push(...webFallbackMatches.value);
    results.value = aggregated.slice(0, MAX_RESULTS);
  };

  const resolveCalculator = (value: string) => {
    if (!settings.state.features.calculator) {
      context.calculator = null;
      applyResults();
      return;
    }
    const match = value.match(CALC_PATTERN);
    if (!match) {
      context.calculator = null;
      applyResults();
      return;
    }
    const expression = match[1];
    const result = evaluateExpression(expression);
    if (result === null) {
      context.calculator = null;
      applyResults();
      return;
    }

    context.calculator = {
      entry: calculatorEntry,
      score: 950,
      badge: "计算结果",
      meta: `${expression.replace(/\s+/g, " ").trim()} = ${result}`,
      data: { expression, result }
    };

    applyResults();
  };

  let translationTask = 0;

  const resolveTranslator = async (value: string) => {
    const matches = value.match(TRANSLATE_PATTERN);
    if (!matches?.groups) {
      context.translator = null;
      loading.value = false;
      applyResults();
      return;
    }

    if (!settings.state.features.webSearch) {
      context.translator = null;
      applyResults();
      return;
    }

    const currentTask = translationTask + 1;
    translationTask = currentTask;

    loading.value = true;
    error.value = null;
    const { target, text } = matches.groups as { target: string; text: string };
    try {
      const translated = await performTranslation({ target: target.toLowerCase(), text });
      if (translationTask !== currentTask) {
        return;
      }
      context.translator = {
        entry: translatorEntry,
        score: 900,
        badge: `翻译到${localeToLabel[target.toLowerCase()] ?? target.toUpperCase()}`,
        meta: translated,
        data: { translated, target, text }
      };
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      context.translator = null;
    } finally {
      if (translationTask === currentTask) {
        loading.value = false;
        applyResults();
      }
    }
  };

  const resolveWebFallbacks = (value: string) => {
    if (!settings.state.features.webSearch) {
      webFallbackMatches.value = [];
      applyResults();
      return;
    }

    const trimmed = value.trim();
    if (!trimmed) {
      webFallbackMatches.value = [];
      applyResults();
      return;
    }

    const templates = settings.state.webFallbacks.length
      ? settings.state.webFallbacks
      : ["https://www.google.com/search?q=%s"];

    const encoded = encodeURIComponent(trimmed);
    webFallbackMatches.value = templates.map((template, index) => {
      const url = template.includes("%s")
        ? template.replace(/%s/g, encoded)
        : `${template}${template.includes("?") ? "&" : "?"}q=${encoded}`;

      let host: string | undefined;
      try {
        const parsed = new URL(url);
        host = parsed.hostname.replace(/^www\./, "");
      } catch {
        host = undefined;
      }

      return {
        entry: {
          id: `web-search-${index}`,
          label: host ? `使用 ${host} 搜索` : "Web 搜索",
          description: template,
          type: "web",
          keywords: [],
          icon: FALLBACK_ICON,
          execute: url
        },
        score: 400 - index,
        badge: host?.toUpperCase(),
        meta: url,
        data: {
          url,
          template,
          query: trimmed
        }
      } as LauncherResult;
    });

    applyResults();
  };

  const setQuery = (value: string) => {
    query.value = value;
    const trimmed = value.trim();
    resolveCalculator(trimmed);
    void resolveTranslator(trimmed);
    resolveWebFallbacks(trimmed);

    const coreEntries = settings.state.features.defaultResults
      ? LAUNCHER_ENTRIES
      : [];

    const scored = coreEntries
      .map((entry) => ({
        entry,
        score: scoreEntry(entry, trimmed)
      }))
      .filter((item) => item.score > 0)
      .sort((a, b) => b.score - a.score);

    baseMatches.value = scored;
    applyResults();

    if (settings.state.features.fileSearch && trimmed.length > 1) {
      fetchFiles(trimmed);
    } else {
      fileMatches.value = [];
      applyResults();
      loading.value = false;
    }
  };

  const clearQuery = () => {
    query.value = "";
    context.calculator = null;
    context.translator = null;
    webFallbackMatches.value = [];
    const coreEntries = settings.state.features.defaultResults
      ? LAUNCHER_ENTRIES
      : [];
    baseMatches.value = coreEntries
      .map((entry) => ({
        entry,
        score: entry.weight ?? 50
      }))
      .sort((a, b) => b.score - a.score);
    fileMatches.value = [];
    applyResults();
  };

  clearQuery();

  watch(
    () => settings.state.webFallbacks.slice(),
    () => {
      resolveWebFallbacks(query.value.trim());
    }
  );

  watch(
    () => settings.state.features.webSearch,
    () => {
      resolveWebFallbacks(query.value.trim());
    }
  );

  return {
    query,
    results,
    loading,
    error,
    setQuery,
    clearQuery,
    highlighted: computed(() => (results.value.length > 0 ? results.value[0] : null))
  };

  async function fetchFiles(value: string) {
    loading.value = true;
    try {
      const response = await invoke<BackendSearchResult[]>("search_files", { query: value });
      fileMatches.value = response.slice(0, 10).map((item) => ({
        entry: {
          id: item.document.id,
          label: item.document.name,
          description: item.document.path,
          type: "file",
          keywords: [],
          icon: "📄",
          execute: item.document.path
        },
        score: item.score,
        badge: item.highlight ?? item.document.ext ?? undefined,
        meta: item.document.path,
        data: {
          path: item.document.path
        }
      }));
      applyResults();
    } catch (err) {
      console.warn("File search failed", err);
    } finally {
      loading.value = false;
    }
  }
};
