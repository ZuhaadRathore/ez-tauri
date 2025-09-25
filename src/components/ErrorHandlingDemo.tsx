import { useState } from 'react'
import { useErrorHandler } from '../utils/error-handling'
import { checkDatabaseConnection, getAllUsers } from '../api'
import ErrorBoundary from './ErrorBoundary'
import { sanitizeText } from '../utils/sanitization'
import type { DatabaseStatus } from '../types/database'

// Component that throws an error for testing
const BuggyComponent = ({ shouldThrow }: { shouldThrow: boolean }) => {
  if (shouldThrow) {
    throw new Error('This is a test error from BuggyComponent')
  }
  return <div className='p-2 text-green-600'>Component is working fine! ✅</div>
}

const ErrorHandlingDemo = () => {
  const [shouldThrowError, setShouldThrowError] = useState(false)
  const [dbStatus, setDbStatus] = useState<DatabaseStatus | null>(null)
  const [loading, setLoading] = useState(false)
  const {
    handleApiError,
    handleNetworkError,
    handleValidationError,
    AppError,
    ErrorCodes,
  } = useErrorHandler()

  const testDatabaseConnection = async () => {
    setLoading(true)
    try {
      const status = await checkDatabaseConnection()
      setDbStatus(status)
    } catch (error) {
      await handleApiError(error as Error, {
        component: 'ErrorHandlingDemo',
        action: 'test_database_connection',
      })
    } finally {
      setLoading(false)
    }
  }

  const testApiError = async () => {
    try {
      // This will likely fail if no users exist or DB isn't set up
      await getAllUsers()
    } catch (error) {
      await handleApiError(error as Error, {
        component: 'ErrorHandlingDemo',
        action: 'test_api_error',
      })
    }
  }

  const testNetworkError = async () => {
    const networkError = new Error('Network connection failed')
    await handleNetworkError(networkError, {
      component: 'ErrorHandlingDemo',
      action: 'test_network_error',
    })
  }

  const testValidationError = async () => {
    await handleValidationError('Invalid email format provided', {
      component: 'ErrorHandlingDemo',
      action: 'test_validation_error',
      metadata: { field: 'email', value: 'invalid-email' },
    })
  }

  const testCustomError = async () => {
    const customError = new AppError(
      'This is a custom application error',
      ErrorCodes.CONFIGURATION_ERROR,
      {
        component: 'ErrorHandlingDemo',
        action: 'test_custom_error',
        metadata: {
          setting: 'database_url',
          reason: 'missing_environment_variable',
        },
      }
    )

    try {
      throw customError
    } catch (error) {
      await handleApiError(error as Error, {
        component: 'ErrorHandlingDemo',
        action: 'test_custom_error',
      })
    }
  }

  return (
    <div className='space-y-6'>
      <header className='space-y-2'>
        <h2 className='text-xl font-semibold'>Error Handling Demo</h2>
        <p className='text-sm text-slate-600 dark:text-slate-300'>
          Test different error handling scenarios and see how they&apos;re
          handled gracefully.
        </p>
      </header>

      <div className='grid gap-4 sm:grid-cols-2'>
        {/* React Error Boundary Demo */}
        <div className='rounded-lg border border-slate-200 bg-white p-4 dark:border-slate-800 dark:bg-slate-900'>
          <h3 className='font-medium mb-3'>React Error Boundary</h3>
          <div className='space-y-3'>
            <ErrorBoundary name='Buggy Component Demo' isolate>
              <BuggyComponent shouldThrow={shouldThrowError} />
            </ErrorBoundary>
            <button
              onClick={() => setShouldThrowError(!shouldThrowError)}
              className='rounded bg-red-600 px-3 py-1.5 text-sm font-medium text-white hover:bg-red-500'
            >
              {shouldThrowError ? 'Fix Component' : 'Break Component'}
            </button>
          </div>
        </div>

        {/* API Error Handling Demo */}
        <div className='rounded-lg border border-slate-200 bg-white p-4 dark:border-slate-800 dark:bg-slate-900'>
          <h3 className='font-medium mb-3'>API Error Handling</h3>
          <div className='space-y-2'>
            <button
              onClick={testDatabaseConnection}
              disabled={loading}
              className='w-full rounded bg-blue-600 px-3 py-1.5 text-sm font-medium text-white hover:bg-blue-500 disabled:opacity-50'
            >
              {loading ? 'Testing...' : 'Test Database Connection'}
            </button>

            <button
              onClick={testApiError}
              className='w-full rounded bg-orange-600 px-3 py-1.5 text-sm font-medium text-white hover:bg-orange-500'
            >
              Test API Error
            </button>

            {dbStatus && (
              <div className='mt-2 p-2 text-xs'>
                <div
                  className={`font-medium ${dbStatus.connected ? 'text-green-600' : 'text-red-600'}`}
                >
                  Status: {dbStatus.connected ? 'Connected' : 'Disconnected'}
                </div>
                {dbStatus.databaseName && (
                  <div>DB: {sanitizeText(dbStatus.databaseName)}</div>
                )}
                {dbStatus.error && (
                  <div className='text-red-600'>
                    Error: {sanitizeText(dbStatus.error)}
                  </div>
                )}
              </div>
            )}
          </div>
        </div>

        {/* Network Error Demo */}
        <div className='rounded-lg border border-slate-200 bg-white p-4 dark:border-slate-800 dark:bg-slate-900'>
          <h3 className='font-medium mb-3'>Network Error Handling</h3>
          <button
            onClick={testNetworkError}
            className='w-full rounded bg-purple-600 px-3 py-1.5 text-sm font-medium text-white hover:bg-purple-500'
          >
            Simulate Network Error
          </button>
        </div>

        {/* Validation Error Demo */}
        <div className='rounded-lg border border-slate-200 bg-white p-4 dark:border-slate-800 dark:bg-slate-900'>
          <h3 className='font-medium mb-3'>Validation Error</h3>
          <div className='space-y-2'>
            <button
              onClick={testValidationError}
              className='w-full rounded bg-yellow-600 px-3 py-1.5 text-sm font-medium text-white hover:bg-yellow-500'
            >
              Test Validation Error
            </button>
            <button
              onClick={testCustomError}
              className='w-full rounded bg-gray-600 px-3 py-1.5 text-sm font-medium text-white hover:bg-gray-500'
            >
              Test Custom Error
            </button>
          </div>
        </div>
      </div>

      <div className='rounded-lg border border-blue-200 bg-blue-50 p-4 dark:border-blue-800 dark:bg-blue-900/20'>
        <h4 className='font-medium text-blue-800 dark:text-blue-200 mb-2'>
          How it works:
        </h4>
        <ul className='text-sm text-blue-700 dark:text-blue-300 space-y-1'>
          <li>• React Error Boundaries catch component render errors</li>
          <li>• API errors are automatically logged to the backend</li>
          <li>
            • Users see friendly error messages instead of technical details
          </li>
          <li>
            • Errors include context for debugging (check browser console)
          </li>
          <li>• Different error types are handled appropriately</li>
        </ul>
      </div>
    </div>
  )
}

export default ErrorHandlingDemo
