import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// A relative base makes the build portable: the same `dist/` works at
// mxqius.github.io/ForceConnect/, at a custom domain root, or on any static
// host, with no per-host config.
export default defineConfig({
  base: "./",
  plugins: [react(), tailwindcss()],
});
