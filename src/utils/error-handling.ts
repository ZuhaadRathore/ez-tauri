import { logError } from '../api'

export interface ErrorContext {
  component?: string
  action?: string
  userId?: string
  metadata?: Record<string, unknown>
  [key: string]: unknown
}

export class AppError extends Error {
  public readonly code: string
  public readonly context: ErrorContext
  public readonly timestamp: Date
  public readonly isOperational: boolean

  constructor(
    message: string,
    code: string = 'UNKNOWN_ERROR',
    context: ErrorContext = {},
    isOperational: boolean = true
  ) {
    super(message)
    this.name = 'AppError'
    this.code = code
    this.context = context
    this.timestamp = new Date()
    this.isOperational = isOperational

    // Maintain proper stack trace for where our error was thrown
    if (Error.captureStackTrace) {
      Error.captureStackTrace(this, AppError)
    }
  }

  static fromError(error: Error, context: ErrorContext = {}): AppError {
    if (error instanceof AppError) {
      return error
    }

    return new AppError(
      error.message,
      'WRAPPED_ERROR',
      { ...context, originalError: error.name },
      false
    )
  }
}

export const ErrorCodes = {
  // Network/API errors
  NETWORK_ERROR: 'NETWORK_ERROR',
  API_ERROR: 'API_ERROR',
  TIMEOUT_ERROR: 'TIMEOUT_ERROR',

  // Validation errors
  VALIDATION_ERROR: 'VALIDATION_ERROR',
  INVALID_INPUT: 'INVALID_INPUT',

  // Authentication/Authorization errors
  AUTH_ERROR: 'AUTH_ERROR',
  UNAUTHORIZED: 'UNAUTHORIZED',
  FORBIDDEN: 'FORBIDDEN',

  // Data/Storage errors
  DATABASE_ERROR: 'DATABASE_ERROR',
  FILE_ERROR: 'FILE_ERROR',
  CACHE_ERROR: 'CACHE_ERROR',

  // UI/Component errors
  RENDER_ERROR: 'RENDER_ERROR',
  COMPONENT_ERROR: 'COMPONENT_ERROR',

  // General errors
  UNKNOWN_ERROR: 'UNKNOWN_ERROR',
  CONFIGURATION_ERROR: 'CONFIGURATION_ERROR',
} as const

export type ErrorCode = (typeof ErrorCodes)[keyof typeof ErrorCodes]

interface ErrorHandlerOptions {
  showToUser?: boolean
  logToBackend?: boolean
  fallback?: () => void
}

export class ErrorHandler {
  private static instance: ErrorHandler

  private constructor() {}

  static getInstance(): ErrorHandler {
    if (!ErrorHandler.instance) {
      ErrorHandler.instance = new ErrorHandler()
    }
    return ErrorHandler.instance
  }

  async handleError(
    error: Error | AppError,
    context: ErrorContext = {},
    options: ErrorHandlerOptions = {}
  ): Promise<void> {
    const { showToUser = true, logToBackend = true, fallback } = options

    const appError =
      error instanceof AppError ? error : AppError.fromError(error, context)

    // Log to console for development
    if (import.meta.env.DEV) {
      console.error('Error handled:', appError)
    }

    // Log to backend if enabled
    if (logToBackend) {
      try {
        await logError(appError.message, {
          code: appError.code,
          context: appError.context,
          timestamp: appError.timestamp.toISOString(),
          stack: appError.stack,
          isOperational: appError.isOperational,
          userAgent: navigator.userAgent,
          url: window.location.href,
        })
      } catch (loggingError) {
        console.error('Failed to log error to backend:', loggingError)
      }
    }

    // Execute fallback if provided
    if (fallback) {
      try {
        fallback()
      } catch (fallbackError) {
        console.error('Error in fallback handler:', fallbackError)
      }
    }

    // Show user notification if enabled
    if (showToUser && this.shouldShowToUser(appError)) {
      this.showUserNotification(appError)
    }
  }

  private shouldShowToUser(error: AppError): boolean {
    // Don't show non-operational errors to users
    if (!error.isOperational) {
      return false
    }

    // Don't show certain error codes to users
    const hiddenCodes: ErrorCode[] = [ErrorCodes.UNKNOWN_ERROR]
    return !hiddenCodes.includes(error.code as ErrorCode)
  }

  private showUserNotification(error: AppError): void {
    // In a real app, this would integrate with your notification system
    // For now, we'll use the browser's notification API if available
    const message = this.getUserFriendlyMessage(error)

    if ('Notification' in window && Notification.permission === 'granted') {
      new Notification('Application Error', {
        body: message,
        icon: '/icon.png',
      })
    } else {
      // Fallback to console for now - in practice you'd integrate with toast notifications
      console.warn(`User notification: ${message}`)
    }
  }

  private getUserFriendlyMessage(error: AppError): string {
    switch (error.code) {
      case ErrorCodes.NETWORK_ERROR:
        return 'Network connection problem. Please check your internet connection.'
      case ErrorCodes.API_ERROR:
        return 'Server error occurred. Please try again later.'
      case ErrorCodes.TIMEOUT_ERROR:
        return 'The request took too long. Please try again.'
      case ErrorCodes.VALIDATION_ERROR:
      case ErrorCodes.INVALID_INPUT:
        return 'Please check your input and try again.'
      case ErrorCodes.AUTH_ERROR:
      case ErrorCodes.UNAUTHORIZED:
        return 'Authentication required. Please log in.'
      case ErrorCodes.FORBIDDEN:
        return "You don't have permission to perform this action."
      case ErrorCodes.DATABASE_ERROR:
        return 'Data storage error. Please try again.'
      case ErrorCodes.FILE_ERROR:
        return 'File operation failed. Please try again.'
      default:
        return 'An unexpected error occurred. Please try again.'
    }
  }
}

// Convenience functions for common error handling scenarios
export const handleApiError = (error: Error, context: ErrorContext = {}) => {
  const appError = new AppError(error.message, ErrorCodes.API_ERROR, {
    ...context,
    action: 'api_call',
  })
  return ErrorHandler.getInstance().handleError(appError, context)
}

export const handleNetworkError = (
  error: Error,
  context: ErrorContext = {}
) => {
  const appError = new AppError(
    error.message || 'Network connection failed',
    ErrorCodes.NETWORK_ERROR,
    { ...context, action: 'network_request' }
  )
  return ErrorHandler.getInstance().handleError(appError, context)
}

export const handleValidationError = (
  message: string,
  context: ErrorContext = {}
) => {
  const appError = new AppError(message, ErrorCodes.VALIDATION_ERROR, {
    ...context,
    action: 'validation',
  })
  return ErrorHandler.getInstance().handleError(appError, context)
}

// React hook for error handling
export const useErrorHandler = () => {
  const errorHandler = ErrorHandler.getInstance()

  return {
    handleError: errorHandler.handleError.bind(errorHandler),
    handleApiError,
    handleNetworkError,
    handleValidationError,
    AppError,
    ErrorCodes,
  }
}

// Global unhandled error handlers
export const setupGlobalErrorHandlers = () => {
  const errorHandler = ErrorHandler.getInstance()

  // Handle unhandled promise rejections
  window.addEventListener('unhandledrejection', event => {
    const promiseError =
      event.reason instanceof Error
        ? event.reason
        : new Error(String(event.reason))

    errorHandler.handleError(
      promiseError,
      { component: 'global', action: 'unhandled_promise_rejection' },
      { showToUser: false }
    )

    event.preventDefault()
  })

  // Handle global JavaScript errors
  window.addEventListener('error', event => {
    const jsError = event.error || new Error(event.message)

    errorHandler.handleError(
      jsError,
      {
        component: 'global',
        action: 'global_error',
        metadata: {
          filename: event.filename,
          lineno: event.lineno,
          colno: event.colno,
        },
      },
      { showToUser: false }
    )
  })
}
