import { createApp } from "vue";
import { createPinia } from "pinia";
import { VueQueryPlugin } from "@tanstack/vue-query";
import App from "./App.vue";
import "./styles.css";
import { useThemeStore } from "./stores/theme";
import { useClipboardStore } from "./stores/clipboard";
import { useSettingsStore } from "./stores/settings";
import { getCurrentWindow } from "@tauri-apps/api/window";

const mountApp = () => {
  const app = createApp(App);
  const pinia = createPinia();

  app.use(pinia);
  app.use(VueQueryPlugin);

  const themeStore = useThemeStore(pinia);
  themeStore.apply();

  const clipboardStore = useClipboardStore(pinia);
  void clipboardStore.fetchHistory();

  const settings = useSettingsStore(pinia);
  if (!settings.state.features.clipboardHistory) {
    clipboardStore.state.items = [];
  }

  settings.$subscribe((_, value) => {
    if (!value.features.clipboardHistory) {
      clipboardStore.state.items = [];
    } else {
      void clipboardStore.fetchHistory();
    }
  });

  const window = getCurrentWindow();

  void window.listen("clipboard:toggle", () => {
    clipboardStore.togglePanel();
  });

  void window.listen("clipboard:capture", () => {
    void clipboardStore.captureCurrent();
  });

  app.mount("#app");
};

mountApp();
