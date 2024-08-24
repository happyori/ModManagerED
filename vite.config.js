import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import { enhancedImages } from "@sveltejs/enhanced-img";
import UnoCSS from '@unocss/svelte-scoped/vite';

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    enhancedImages(),
    UnoCSS({
      injectReset: '@unocss/reset/tailwind.css'
    }),
    sveltekit()
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
