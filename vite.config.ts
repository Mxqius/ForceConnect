import { fileURLToPath, URL } from "node:url";

import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// Set by `tauri dev --host` when testing on a physical device over the network.
const host = process.env.TAURI_DEV_HOST;
const isTauriDebug = process.env.TAURI_ENV_DEBUG === "true";

export default defineConfig({
  plugins: [react(), tailwindcss()],

  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },

  // Tauri owns the terminal during `tauri dev`; let its output through.
  clearScreen: false,

  server: {
    // Not Tauri's default 1420: Hyper-V and WSL reserve large blocks of low
    // ports on Windows (`netsh int ipv4 show excludedportrange protocol=tcp`),
    // and 1420 commonly lands inside one, which surfaces as a confusing
    // `EACCES` on ::1 rather than "port in use". 5173 sits above those blocks.
    port: 5173,
    strictPort: true,
    host: host ?? false,
    hmr: host ? { protocol: "ws", host, port: 5174 } : undefined,
    watch: {
      // Rust changes are picked up by cargo, not vite.
      ignored: ["**/src-tauri/**"],
    },
  },

  // Match the browser floor of the WebView2 runtime we ship against.
  build: {
    target: "chrome120",
    // Vite 8 minifies with oxc; naming esbuild explicitly hits a deprecated
    // path that requires installing esbuild separately.
    sourcemap: isTauriDebug,
    minify: !isTauriDebug,
  },
});
