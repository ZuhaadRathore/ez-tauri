/**
 * Main App component that provides routing, theming, and error boundary.
 *
 * Sets up the application's core infrastructure including:
 * - React Router for navigation
 * - Theme management
 * - Top-level error boundary for graceful error handling
 */

import { RouterProvider } from 'react-router-dom'
import { router } from './router'
import { useTheme } from './hooks'
import ErrorBoundary from './components/ErrorBoundary'

const App = () => {
  // Initialize theme management (dark/light mode)
  useTheme()

  return (
    <ErrorBoundary name='App Root'>
      <RouterProvider router={router} />
    </ErrorBoundary>
  )
}

export default App
