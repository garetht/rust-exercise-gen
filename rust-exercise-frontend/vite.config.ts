import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import arraybuffer from "vite-plugin-arraybuffer";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), arraybuffer()],
  assetsInclude: ['**/*.bin'],
  base: '/be-the-borrow-checker/'
})
