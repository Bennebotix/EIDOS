import { defineConfig } from 'vite';

export default defineConfig({
  server: {
    fs: {
      // Allow serving files from the project root (to access rust/pkg)
      allow: ['..']
    }
  }
});
