/**
 * Main entry point for the Tauri React application.
 *
 * Sets up the React DOM root with error handling, strict mode,
 * and global application configuration.
 */

import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import { setupGlobalErrorHandlers } from './utils/error-handling'
import './index.css'

// Initialize global error handlers for uncaught exceptions and promise rejections
setupGlobalErrorHandlers()

// Render the application with React StrictMode for development warnings
ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)
