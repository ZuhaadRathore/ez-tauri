/**
 * React Router configuration for the application.
 *
 * Sets up client-side routing with:
 * - Lazy-loaded components for better performance
 * - Loading states with suspense boundaries
 * - 404 error handling
 * - Nested layout structure
 */

import { createBrowserRouter } from 'react-router-dom'
import { lazy, Suspense } from 'react'
import Layout from './layouts/Layout'
import { LoadingSpinner } from './components'

// Lazy load pages for code splitting and better performance
const Home = lazy(() => import('./pages/Home'))

// Custom 404 error page component
const notFoundElement = (
  <div className='mx-auto max-w-2xl px-6 py-16 text-center text-slate-600 dark:text-slate-300'>
    <h1 className='text-2xl font-semibold text-slate-900 dark:text-slate-100'>
      Page not found
    </h1>
    <p className='mt-3 text-sm'>
      Use the navigation above to return to a valid route.
    </p>
  </div>
)

export const router = createBrowserRouter([
  {
    path: '/',
    element: <Layout />,
    errorElement: notFoundElement,
    children: [
      {
        index: true,
        element: (
          <Suspense fallback={<LoadingSpinner />}>
            <Home />
          </Suspense>
        ),
      },
      {
        path: '*',
        element: notFoundElement,
      },
    ],
  },
])
