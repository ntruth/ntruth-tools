import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      "@": "/src"
    }
  },
  server: {
    host: "127.0.0.1",
    port: 5173
  },
  build: {
    rollupOptions: {
      input: {
        main: "/index.html",
        launcher: "/launcher.html"
      }
    }
  },
  test: {
    environment: "jsdom",
    globals: true,
    setupFiles: ["vitest.setup.ts"]
  }
});
