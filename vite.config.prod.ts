import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// Production Vite configuration
export default defineConfig({
  plugins: [react()],

  // Production build optimization
  build: {
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true,
        drop_debugger: true,
      },
    },
    rollupOptions: {
      output: {
        manualChunks: {
          // Vendor chunks
          react: ['react', 'react-dom'],
          router: ['react-router-dom'],
          ui: ['@tauri-apps/api', '@tauri-apps/plugin-opener'],
          store: ['zustand'],
          utils: ['zod', 'dompurify'],
        },
        // Add hash to filenames for cache busting
        entryFileNames: 'assets/[name].[hash].js',
        chunkFileNames: 'assets/[name].[hash].js',
        assetFileNames: 'assets/[name].[hash].[ext]',
      },
    },
    // Generate source maps for debugging
    sourcemap: true,
  },

  // Production environment
  define: {
    __APP_ENV__: JSON.stringify('production'),
  },

  // Disable dev server options
  server: undefined,
  clearScreen: false,
})
