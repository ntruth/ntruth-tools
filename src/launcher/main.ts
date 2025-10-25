import { createApp } from "vue";
import { createPinia } from "pinia";
import { VueQueryPlugin } from "@tanstack/vue-query";
import LauncherApp from "@/features/launcher/LauncherApp.vue";
import "@/styles.css";

const bootstrap = () => {
  const app = createApp(LauncherApp);
  const pinia = createPinia();

  app.use(pinia);
  app.use(VueQueryPlugin);

  app.mount("#launcher-root");
};

bootstrap();
