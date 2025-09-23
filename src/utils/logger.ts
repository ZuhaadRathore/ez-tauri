import { invoke } from '@tauri-apps/api/core'

export enum LogLevel {
  ERROR = 'error',
  WARN = 'warn',
  INFO = 'info',
  DEBUG = 'debug',
  TRACE = 'trace',
}

export interface LogEntry {
  timestamp: string
  level: string
  target: string
  message: string
  fields: Record<string, unknown>
  span?: string
  threadName?: string
  file?: string
  line?: number
}

export interface LogContext {
  userId?: string
  sessionId?: string
  component?: string
  action?: string
  [key: string]: unknown
}

export interface LogQueryParams {
  level?: string
  startTime?: string
  endTime?: string
  target?: string
  messageContains?: string
  limit?: number
  offset?: number
}

export interface LogResponse {
  logs: LogEntry[]
  totalCount: number
  hasMore: boolean
}

export interface LogConfig {
  enabled: boolean
  level: LogLevel
  console: {
    enabled: boolean
    format: string
    colors: boolean
  }
  file: {
    enabled: boolean
    directory: string
    filenamePrefix: string
    rotation: string
    maxFiles: number
    maxSizeMb?: number
  }
  structured: {
    enabled: boolean
    includeSpans: boolean
    includeTargets: boolean
    includeThreadNames: boolean
    includeFileInfo: boolean
  }
}

class Logger {
  private context: LogContext = {}
  private isProduction = import.meta.env.PROD
  private sessionId = this.generateSessionId()

  constructor() {
    // Set default context
    this.setContext({
      sessionId: this.sessionId,
      userAgent: navigator.userAgent,
      platform: navigator.platform,
    })
  }

  private generateSessionId(): string {
    return `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
  }

  /**
   * Set persistent context that will be included in all log entries
   */
  setContext(context: LogContext): void {
    this.context = { ...this.context, ...context }
  }

  /**
   * Get current context
   */
  getContext(): LogContext {
    return { ...this.context }
  }

  /**
   * Clear all context
   */
  clearContext(): void {
    this.context = {
      sessionId: this.sessionId,
      userAgent: navigator.userAgent,
      platform: navigator.platform,
    }
  }

  /**
   * Log an error message
   */
  async error(message: string, context?: LogContext): Promise<void> {
    await this.log(LogLevel.ERROR, message, context)
  }

  /**
   * Log a warning message
   */
  async warn(message: string, context?: LogContext): Promise<void> {
    await this.log(LogLevel.WARN, message, context)
  }

  /**
   * Log an info message
   */
  async info(message: string, context?: LogContext): Promise<void> {
    await this.log(LogLevel.INFO, message, context)
  }

  /**
   * Log a debug message (only in development)
   */
  async debug(message: string, context?: LogContext): Promise<void> {
    if (!this.isProduction) {
      await this.log(LogLevel.DEBUG, message, context)
    }
  }

  /**
   * Log a trace message (only in development)
   */
  async trace(message: string, context?: LogContext): Promise<void> {
    if (!this.isProduction) {
      await this.log(LogLevel.TRACE, message, context)
    }
  }

  /**
   * Log with explicit level
   */
  private async log(
    level: LogLevel,
    message: string,
    context?: LogContext
  ): Promise<void> {
    const logContext = { ...this.context, ...context }

    // Always log to browser console
    this.logToBrowser(level, message, logContext)

    // Send to Rust backend for file logging
    try {
      await invoke('create_test_log', {
        level: level.toString(),
        message: this.formatMessage(message, logContext),
      })
    } catch (error) {
      console.error('Failed to send log to backend:', error)
    }
  }

  /**
   * Log to browser console with appropriate styling
   */
  private logToBrowser(
    level: LogLevel,
    message: string,
    context: LogContext
  ): void {
    const timestamp = new Date().toISOString()
    const formattedMessage = `[${timestamp}] ${message}`

    switch (level) {
      case LogLevel.ERROR:
        console.error(formattedMessage, context)
        break
      case LogLevel.WARN:
        console.warn(formattedMessage, context)
        break
      case LogLevel.INFO:
        console.info(formattedMessage, context)
        break
      case LogLevel.DEBUG:
        console.debug(formattedMessage, context)
        break
      case LogLevel.TRACE:
        console.trace(formattedMessage, context)
        break
    }
  }

  /**
   * Format message with context
   */
  private formatMessage(message: string, context: LogContext): string {
    const contextStr =
      Object.keys(context).length > 0
        ? ` | Context: ${JSON.stringify(context)}`
        : ''
    return `${message}${contextStr}`
  }

  /**
   * Log performance timing
   */
  async timeOperation<T>(
    operationName: string,
    operation: () => Promise<T>,
    context?: LogContext
  ): Promise<T> {
    const startTime = performance.now()

    await this.debug(`Starting operation: ${operationName}`, context)

    try {
      const result = await operation()
      const duration = performance.now() - startTime

      await this.info(`Operation completed: ${operationName}`, {
        ...context,
        duration_ms: Math.round(duration),
        success: true,
      })

      return result
    } catch (error) {
      const duration = performance.now() - startTime

      await this.error(`Operation failed: ${operationName}`, {
        ...context,
        duration_ms: Math.round(duration),
        error: error instanceof Error ? error.message : String(error),
        success: false,
      })

      throw error
    }
  }

  /**
   * Create a child logger with additional context
   */
  child(context: LogContext): Logger {
    const childLogger = new Logger()
    childLogger.setContext({ ...this.context, ...context })
    return childLogger
  }

  /**
   * Get logs from the backend
   */
  async getLogs(params: LogQueryParams): Promise<LogResponse> {
    try {
      return await invoke('get_log_entries', { params })
    } catch (error) {
      console.error('Failed to get logs:', error)
      throw error
    }
  }

  /**
   * Get log configuration
   */
  async getConfig(): Promise<LogConfig> {
    try {
      return await invoke('get_log_config')
    } catch (error) {
      console.error('Failed to get log config:', error)
      throw error
    }
  }

  /**
   * Update log configuration
   */
  async updateConfig(config: LogConfig): Promise<string> {
    try {
      return await invoke('update_log_config', { config })
    } catch (error) {
      console.error('Failed to update log config:', error)
      throw error
    }
  }

  /**
   * Clear old logs
   */
  async clearOldLogs(daysToKeep: number): Promise<string> {
    try {
      return await invoke('clear_old_logs', { days_to_keep: daysToKeep })
    } catch (error) {
      console.error('Failed to clear old logs:', error)
      throw error
    }
  }

  /**
   * Get log statistics
   */
  async getStats(): Promise<Record<string, unknown>> {
    try {
      return await invoke('get_log_stats')
    } catch (error) {
      console.error('Failed to get log stats:', error)
      throw error
    }
  }
}

// Global logger instance
export const logger = new Logger()

// Hook for React components
export function useLogger(componentName?: string) {
  const componentLogger = componentName
    ? logger.child({ component: componentName })
    : logger

  return componentLogger
}

// Error boundary integration
export function logError(error: Error, errorInfo?: unknown): void {
  logger.error('Unhandled error caught by error boundary', {
    error: error.message,
    stack: error.stack,
    errorInfo,
    timestamp: new Date().toISOString(),
  })
}

// Global error handler
if (typeof window !== 'undefined') {
  window.addEventListener('error', event => {
    logger.error('Uncaught error', {
      message: event.message,
      filename: event.filename,
      lineno: event.lineno,
      colno: event.colno,
      error: event.error?.stack,
    })
  })

  window.addEventListener('unhandledrejection', event => {
    logger.error('Unhandled promise rejection', {
      reason: event.reason,
      stack: event.reason?.stack,
    })
  })
}

export default logger
