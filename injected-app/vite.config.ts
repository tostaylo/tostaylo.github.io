import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    rollupOptions: {
      output: {
        entryFileNames: "main.js", // Entry file name
        chunkFileNames: "chunks/[name].js", // Chunk files (optional)
        assetFileNames: "assets/[name][extname]", // Asset files (optional)
      },
    },
  },
});
