import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      "/trpc": {
        target: "http://127.0.0.1:4000",
      },
    },
  },
});
