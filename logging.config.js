/**
 * Environment-specific logging configuration for the application.
 *
 * Provides different logging levels and outputs based on the deployment environment:
 * - Development: Debug level, pretty formatting, console + file output
 * - Staging: Info level, JSON formatting, structured logging
 * - Production: Warn level, JSON formatting, file-only output for security
 *
 * Configuration includes file rotation, size limits, and retention policies.
 */

export const getLoggingConfig = (
  env = process.env.NODE_ENV || 'development'
) => {
  const baseConfig = {
    // Development environment: Verbose logging for debugging
    development: {
      level: 'debug',
      format: 'pretty',
      console: {
        enabled: true,
        colors: true,
      },
      file: {
        enabled: true,
        directory: 'logs',
        prefix: 'ez-tauri-dev',
        rotation: 'daily',
        maxFiles: 30,
        maxSize: 100, // MB - larger files for development debugging
      },
      json: false,
    },

    // Staging environment: Structured logging for testing
    staging: {
      level: 'info',
      format: 'json',
      console: {
        enabled: true,
        colors: false,
      },
      file: {
        enabled: true,
        directory: 'logs',
        prefix: 'ez-tauri-staging',
        rotation: 'daily',
        maxFiles: 14,
        maxSize: 100, // MB - moderate file size for testing
      },
      json: true,
    },

    // Production environment: Minimal logging for performance
    production: {
      level: 'warn',
      format: 'json',
      console: {
        enabled: false,
        colors: false,
      },
      file: {
        enabled: true,
        directory: '/var/log/ez-tauri', // System log directory for production
        prefix: 'ez-tauri-prod',
        rotation: 'daily',
        maxFiles: 7,
        maxSize: 50, // MB - smaller files for production efficiency
      },
      json: true,
    },
  }

  // Return environment-specific config or fallback to development
  return baseConfig[env] || baseConfig.development
}

export default getLoggingConfig()
