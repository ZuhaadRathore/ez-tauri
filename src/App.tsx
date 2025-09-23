import { RouterProvider } from 'react-router-dom'
import { router } from './router'
import { useTheme } from './hooks'
import ErrorBoundary from './components/ErrorBoundary'

const App = () => {
  useTheme()

  return (
    <ErrorBoundary>
      <RouterProvider router={router} />
    </ErrorBoundary>
  )
}

export default App
