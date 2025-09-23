import { Component, type ErrorInfo, type ReactNode } from 'react'

interface Props {
  children: ReactNode
  fallback?: ReactNode
  onError?: (error: Error, errorInfo: ErrorInfo) => void
}

interface State {
  hasError: boolean
  error?: Error
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = { hasError: false }
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error }
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Unhandled UI error', error, errorInfo)

    if (this.props.onError) {
      this.props.onError(error, errorInfo)
    }
  }

  handleReset = () => {
    this.setState({ hasError: false, error: undefined })
  }

  render() {
    if (this.state.hasError) {
      if (this.props.fallback) {
        return this.props.fallback
      }

      return (
        <div className='flex min-h-screen items-center justify-center bg-slate-100 px-4 dark:bg-slate-950'>
          <div className='w-full max-w-md space-y-6 rounded-2xl border border-slate-200 bg-white p-8 text-left shadow-sm dark:border-slate-800 dark:bg-slate-900'>
            <div>
              <h1 className='text-xl font-semibold text-slate-900 dark:text-slate-100'>
                Something went wrong
              </h1>
              <p className='mt-2 text-sm text-slate-600 dark:text-slate-300'>
                An unexpected error occurred. Reload the app or return to the
                previous screen and try again.
              </p>
            </div>

            <div className='flex flex-wrap gap-3'>
              <button
                type='button'
                onClick={() => window.location.reload()}
                className='rounded-full bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-500'
              >
                Reload app
              </button>
              <button
                type='button'
                onClick={this.handleReset}
                className='rounded-full border border-slate-300 px-4 py-2 text-sm font-medium text-slate-700 transition-colors hover:border-blue-500 hover:text-blue-600 dark:border-slate-700 dark:text-slate-200 dark:hover:border-blue-400 dark:hover:text-blue-300'
              >
                Return to app
              </button>
            </div>

            {import.meta.env.DEV && this.state.error && (
              <details className='space-y-2 rounded-lg border border-slate-200 bg-slate-50 p-4 text-sm text-slate-700 dark:border-slate-800 dark:bg-slate-900/60 dark:text-slate-200'>
                <summary className='cursor-pointer font-medium'>
                  Error details
                </summary>
                <pre className='overflow-auto whitespace-pre-wrap text-xs leading-6'>
                  {this.state.error.stack}
                </pre>
              </details>
            )}
          </div>
        </div>
      )
    }

    return this.props.children
  }
}

export default ErrorBoundary
