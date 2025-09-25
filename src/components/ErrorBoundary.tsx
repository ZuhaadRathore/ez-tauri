import { Component, type ErrorInfo, type ReactNode } from 'react'
import { logError } from '../api'

interface Props {
  children: ReactNode
  fallback?: ReactNode
  onError?: (error: Error, errorInfo: ErrorInfo) => void
  isolate?: boolean
  name?: string
}

interface State {
  hasError: boolean
  error?: Error
  errorId?: string
  retryCount: number
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      hasError: false,
      retryCount: 0,
    }
  }

  static getDerivedStateFromError(error: Error): Partial<State> {
    const errorId = `error_${Date.now()}_${Math.random().toString(36).substring(7)}`
    return {
      hasError: true,
      error,
      errorId,
    }
  }

  async componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Unhandled UI error', error, errorInfo)

    // Log error to backend for analysis
    try {
      await logError(`UI Error in ${this.props.name || 'Unknown Component'}`, {
        error: {
          message: error.message,
          stack: error.stack,
          name: error.name,
        },
        errorInfo: {
          componentStack: errorInfo.componentStack,
        },
        errorId: this.state.errorId,
        retryCount: this.state.retryCount,
        userAgent: navigator.userAgent,
        url: window.location.href,
        timestamp: new Date().toISOString(),
      })
    } catch (loggingError) {
      console.error('Failed to log error to backend:', loggingError)
    }

    if (this.props.onError) {
      this.props.onError(error, errorInfo)
    }
  }

  handleReset = () => {
    this.setState({
      hasError: false,
      error: undefined,
      errorId: undefined,
      retryCount: this.state.retryCount + 1,
    })
  }

  handleReload = () => {
    window.location.reload()
  }

  render() {
    if (this.state.hasError) {
      if (this.props.fallback) {
        return this.props.fallback
      }

      const showFullscreen = !this.props.isolate

      if (showFullscreen) {
        return (
          <div className='flex min-h-screen items-center justify-center bg-slate-100 px-4 dark:bg-slate-950'>
            <div className='w-full max-w-md space-y-6 rounded-2xl border border-slate-200 bg-white p-8 text-left shadow-sm dark:border-slate-800 dark:bg-slate-900'>
              <div className='flex items-start gap-3'>
                <div className='flex-shrink-0'>
                  <div className='flex h-8 w-8 items-center justify-center rounded-full bg-red-100 dark:bg-red-900/20'>
                    <svg
                      className='h-4 w-4 text-red-500'
                      fill='none'
                      viewBox='0 0 24 24'
                      stroke='currentColor'
                    >
                      <path
                        strokeLinecap='round'
                        strokeLinejoin='round'
                        strokeWidth={2}
                        d='M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z'
                      />
                    </svg>
                  </div>
                </div>
                <div className='flex-1'>
                  <h1 className='text-lg font-semibold text-slate-900 dark:text-slate-100'>
                    {this.props.name
                      ? `Error in ${this.props.name}`
                      : 'Something went wrong'}
                  </h1>
                  <p className='mt-1 text-sm text-slate-600 dark:text-slate-300'>
                    An unexpected error occurred. You can try reloading the app
                    or returning to the previous screen.
                  </p>
                  {this.state.errorId && (
                    <p className='mt-2 text-xs text-slate-500 dark:text-slate-400'>
                      Error ID: {this.state.errorId}
                    </p>
                  )}
                </div>
              </div>

              <div className='flex gap-3'>
                <button
                  type='button'
                  onClick={this.handleReload}
                  className='rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2'
                >
                  Reload app
                </button>
                <button
                  type='button'
                  onClick={this.handleReset}
                  className='rounded-lg border border-slate-300 px-4 py-2 text-sm font-medium text-slate-700 transition-colors hover:border-blue-500 hover:text-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:border-slate-700 dark:text-slate-200 dark:hover:border-blue-400 dark:hover:text-blue-300'
                >
                  Try again{' '}
                  {this.state.retryCount > 0 && `(${this.state.retryCount})`}
                </button>
              </div>

              {import.meta.env.DEV && this.state.error && (
                <details className='space-y-2 rounded-lg border border-slate-200 bg-slate-50 p-4 text-sm text-slate-700 dark:border-slate-800 dark:bg-slate-900/60 dark:text-slate-200'>
                  <summary className='cursor-pointer font-medium'>
                    Error details (Development)
                  </summary>
                  <div className='space-y-2 text-xs'>
                    <div>
                      <strong>Message:</strong> {this.state.error.message}
                    </div>
                    <div>
                      <strong>Error Name:</strong> {this.state.error.name}
                    </div>
                    <details className='mt-2'>
                      <summary className='cursor-pointer'>Stack trace</summary>
                      <pre className='mt-1 overflow-auto whitespace-pre-wrap text-xs leading-5 text-slate-600 dark:text-slate-300'>
                        {this.state.error.stack}
                      </pre>
                    </details>
                  </div>
                </details>
              )}
            </div>
          </div>
        )
      }

      // Inline error display for isolated components
      return (
        <div className='rounded-lg border border-red-200 bg-red-50 p-4 dark:border-red-800 dark:bg-red-900/10'>
          <div className='flex items-start gap-3'>
            <div className='flex-shrink-0'>
              <svg
                className='h-5 w-5 text-red-400'
                fill='none'
                viewBox='0 0 24 24'
                stroke='currentColor'
              >
                <path
                  strokeLinecap='round'
                  strokeLinejoin='round'
                  strokeWidth={2}
                  d='M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z'
                />
              </svg>
            </div>
            <div className='flex-1'>
              <h3 className='text-sm font-medium text-red-800 dark:text-red-200'>
                {this.props.name
                  ? `Error in ${this.props.name}`
                  : 'Component Error'}
              </h3>
              <p className='mt-1 text-sm text-red-700 dark:text-red-300'>
                This section couldn&apos;t load properly.
              </p>
              <div className='mt-3 flex gap-2'>
                <button
                  type='button'
                  onClick={this.handleReset}
                  className='text-xs font-medium text-red-600 hover:text-red-500 dark:text-red-400 dark:hover:text-red-300'
                >
                  Try again
                </button>
              </div>
            </div>
          </div>
        </div>
      )
    }

    return this.props.children
  }
}

export default ErrorBoundary
