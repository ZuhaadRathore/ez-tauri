import { invoke } from '@tauri-apps/api/core'
import {
  AppError,
  ErrorCodes,
  ErrorHandler,
  type ErrorContext,
  type ErrorCode,
} from './error-handling'

interface ApiOptions {
  timeout?: number
  retries?: number
  context?: ErrorContext
  silent?: boolean
}

interface RetryOptions {
  maxRetries: number
  baseDelay: number
  maxDelay: number
}

const DEFAULT_OPTIONS: Required<Omit<ApiOptions, 'context'>> = {
  timeout: 10000,
  retries: 2,
  silent: false,
}

const DEFAULT_RETRY_OPTIONS: RetryOptions = {
  maxRetries: 2,
  baseDelay: 1000,
  maxDelay: 10000,
}

export class ApiWrapper {
  private static instance: ApiWrapper
  private errorHandler: ErrorHandler

  private constructor() {
    this.errorHandler = ErrorHandler.getInstance()
  }

  static getInstance(): ApiWrapper {
    if (!ApiWrapper.instance) {
      ApiWrapper.instance = new ApiWrapper()
    }
    return ApiWrapper.instance
  }

  async invoke<T>(
    command: string,
    args?: Record<string, unknown>,
    options: ApiOptions = {}
  ): Promise<T> {
    const mergedOptions = { ...DEFAULT_OPTIONS, ...options }

    return this.withRetry(
      () => this.invokeCommand<T>(command, args, mergedOptions),
      {
        maxRetries: mergedOptions.retries,
        baseDelay: DEFAULT_RETRY_OPTIONS.baseDelay,
        maxDelay: DEFAULT_RETRY_OPTIONS.maxDelay,
      },
      mergedOptions.context
    )
  }

  private async invokeCommand<T>(
    command: string,
    args: Record<string, unknown> | undefined,
    options: Required<Omit<ApiOptions, 'context'>> & { context?: ErrorContext }
  ): Promise<T> {
    const controller = new AbortController()
    const timeoutId = setTimeout(() => controller.abort(), options.timeout)

    try {
      // Tauri's invoke doesn't support AbortController directly,
      // so we'll implement timeout via Promise.race
      const result = await Promise.race([
        invoke<T>(command, args),
        this.createTimeoutPromise(options.timeout),
      ])

      clearTimeout(timeoutId)
      return result
    } catch (error) {
      clearTimeout(timeoutId)

      if (error instanceof Error) {
        await this.handleInvokeError(
          error,
          command,
          options.context,
          options.silent
        )
      }

      throw error
    }
  }

  private createTimeoutPromise(timeout: number): Promise<never> {
    return new Promise((_, reject) => {
      setTimeout(() => {
        reject(
          new AppError(
            `Request timeout after ${timeout}ms`,
            ErrorCodes.TIMEOUT_ERROR,
            { metadata: { timeout } }
          )
        )
      }, timeout)
    })
  }

  private async withRetry<T>(
    operation: () => Promise<T>,
    retryOptions: RetryOptions,
    context: ErrorContext = {}
  ): Promise<T> {
    let lastError: Error | undefined

    for (let attempt = 0; attempt <= retryOptions.maxRetries; attempt++) {
      try {
        return await operation()
      } catch (error) {
        lastError = error instanceof Error ? error : new Error(String(error))

        if (attempt === retryOptions.maxRetries) {
          // Last attempt failed, throw the error
          break
        }

        if (!this.isRetryableError(lastError)) {
          // Don't retry non-retryable errors
          break
        }

        const delay = Math.min(
          retryOptions.baseDelay * Math.pow(2, attempt),
          retryOptions.maxDelay
        )

        // Add some jitter to prevent thundering herd
        const jitteredDelay = delay + Math.random() * 1000

        await this.sleep(jitteredDelay)
      }
    }

    if (lastError) {
      throw lastError
    }

    throw new AppError('Unknown retry error', ErrorCodes.UNKNOWN_ERROR, context)
  }

  private isRetryableError(error: Error): boolean {
    if (error instanceof AppError) {
      const retryableCodes: ErrorCode[] = [
        ErrorCodes.NETWORK_ERROR,
        ErrorCodes.TIMEOUT_ERROR,
        ErrorCodes.API_ERROR,
      ]
      return retryableCodes.includes(error.code as ErrorCode)
    }

    // Check for common retryable error patterns
    const retryablePatterns = [
      /network/i,
      /timeout/i,
      /connection/i,
      /temporary/i,
      /server error/i,
    ]

    return retryablePatterns.some(pattern => pattern.test(error.message))
  }

  private async handleInvokeError(
    error: Error,
    command: string,
    context?: ErrorContext,
    silent?: boolean
  ): Promise<void> {
    let appError: AppError

    if (error instanceof AppError) {
      appError = error
    } else {
      // Convert generic errors to AppErrors with appropriate codes
      let errorCode: ErrorCode = ErrorCodes.API_ERROR

      if (error.message.includes('timeout')) {
        errorCode = ErrorCodes.TIMEOUT_ERROR
      } else if (
        error.message.includes('network') ||
        error.message.includes('connection')
      ) {
        errorCode = ErrorCodes.NETWORK_ERROR
      } else if (
        error.message.includes('permission') ||
        error.message.includes('unauthorized')
      ) {
        errorCode = ErrorCodes.UNAUTHORIZED
      } else if (error.message.includes('forbidden')) {
        errorCode = ErrorCodes.FORBIDDEN
      } else if (
        error.message.includes('validation') ||
        error.message.includes('invalid')
      ) {
        errorCode = ErrorCodes.VALIDATION_ERROR
      }

      appError = new AppError(error.message, errorCode, {
        ...context,
        command,
        action: 'tauri_invoke',
      })
    }

    await this.errorHandler.handleError(appError, context, {
      logToBackend: true,
      showToUser: !silent,
    })
  }

  private sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms))
  }
}

// Convenience functions for common API patterns
export const safeInvoke = <T>(
  command: string,
  args?: Record<string, unknown>,
  options?: ApiOptions
): Promise<T> => {
  return ApiWrapper.getInstance().invoke<T>(command, args, options)
}

export const silentInvoke = <T>(
  command: string,
  args?: Record<string, unknown>,
  options?: Omit<ApiOptions, 'silent'>
): Promise<T> => {
  return ApiWrapper.getInstance().invoke<T>(command, args, {
    ...options,
    silent: true,
  })
}

export const quickInvoke = <T>(
  command: string,
  args?: Record<string, unknown>,
  options?: Omit<ApiOptions, 'timeout' | 'retries'>
): Promise<T> => {
  return ApiWrapper.getInstance().invoke<T>(command, args, {
    ...options,
    timeout: 3000,
    retries: 0,
  })
}
