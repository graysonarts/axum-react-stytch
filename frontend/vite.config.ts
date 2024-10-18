import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import wasmPack from "vite-plugin-wasm-pack";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), wasmPack("../common_types")],
  optimizeDeps: {
    exclude: ["common_types"],
  },
  assetsInclude: ["../**/*.wasm"],
});
