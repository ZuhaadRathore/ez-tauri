/**
 * Database API functions for interacting with the Tauri backend.
 *
 * This module provides secure, sanitized interfaces for:
 * - Database connection management
 * - User CRUD operations with input validation
 * - Application logging with XSS prevention
 * - Authentication with bcrypt password handling
 *
 * All user inputs are sanitized before being sent to the backend,
 * and errors are handled gracefully with context information.
 */

import { safeInvoke } from '../utils/api-wrapper'
import {
  sanitizeEmail,
  sanitizeUsername,
  sanitizeName,
  sanitizeMetadata,
  sanitizeLogLevel,
} from '../utils/sanitization'
import type {
  DatabaseStatus,
  User,
  CreateUser,
  UpdateUser,
  LoginRequest,
  AppLog,
  CreateAppLog,
  LogQuery,
} from '../types/database'

// ==================== Database Management ====================
export const checkDatabaseConnection = async (): Promise<DatabaseStatus> => {
  return await safeInvoke<DatabaseStatus>(
    'check_database_connection',
    undefined,
    {
      context: { component: 'database', action: 'check_connection' },
    }
  )
}

/** Initializes the database with required tables and indexes. */
export const initializeDatabase = async (): Promise<string> => {
  return await safeInvoke<string>('initialize_database', undefined, {
    context: { component: 'database', action: 'initialize' },
  })
}

/** Runs database migrations to update schema to the latest version. */
export const runMigrations = async (): Promise<string> => {
  return await safeInvoke<string>('run_migrations', undefined, {
    context: { component: 'database', action: 'run_migrations' },
  })
}

// ==================== User Management ====================

/** Retrieves all users from the database (excluding password hashes). */
export const getAllUsers = async (): Promise<User[]> => {
  return await safeInvoke<User[]>('get_all_users', undefined, {
    context: { component: 'users', action: 'get_all' },
  })
}

/** Retrieves a specific user by their unique identifier. */
export const getUserById = async (userId: string): Promise<User | null> => {
  return await safeInvoke<User | null>(
    'get_user_by_id',
    { userId },
    {
      context: { component: 'users', action: 'get_by_id', userId },
    }
  )
}

/** Creates a new user account with input validation and password hashing. */
export const createUser = async (userData: CreateUser): Promise<User> => {
  const sanitizedUserData: CreateUser = {
    email: sanitizeEmail(userData.email),
    username: sanitizeUsername(userData.username),
    password: userData.password, // Don't sanitize password - handled by bcrypt
    firstName: userData.firstName
      ? sanitizeName(userData.firstName)
      : undefined,
    lastName: userData.lastName ? sanitizeName(userData.lastName) : undefined,
  }

  return await safeInvoke<User>(
    'create_user',
    { userData: sanitizedUserData },
    {
      context: { component: 'users', action: 'create' },
    }
  )
}

/** Updates an existing user's information with validation. */
export const updateUser = async (
  userId: string,
  userData: UpdateUser
): Promise<User> => {
  const sanitizedUserData: UpdateUser = {
    email: userData.email ? sanitizeEmail(userData.email) : undefined,
    username: userData.username
      ? sanitizeUsername(userData.username)
      : undefined,
    firstName: userData.firstName
      ? sanitizeName(userData.firstName)
      : undefined,
    lastName: userData.lastName ? sanitizeName(userData.lastName) : undefined,
    isActive: userData.isActive,
  }

  return await safeInvoke<User>(
    'update_user',
    { userId, userData: sanitizedUserData },
    {
      context: { component: 'users', action: 'update', userId },
    }
  )
}

export const deleteUser = async (userId: string): Promise<string> => {
  return await safeInvoke<string>(
    'delete_user',
    { userId },
    {
      context: { component: 'users', action: 'delete', userId },
    }
  )
}

export const authenticateUser = async (
  loginData: LoginRequest
): Promise<User | null> => {
  // Sanitize login input
  const sanitizedLoginData: LoginRequest = {
    email: sanitizeEmail(loginData.email),
    password: loginData.password, // Don't sanitize password
  }

  return await safeInvoke<User | null>(
    'authenticate_user',
    { loginData: sanitizedLoginData },
    {
      context: { component: 'auth', action: 'authenticate' },
    }
  )
}

// Logging
export const createLog = async (logData: CreateAppLog): Promise<AppLog> => {
  // Sanitize log data to prevent XSS in log viewing interfaces
  const sanitizedLogData: CreateAppLog = {
    level: sanitizeLogLevel(logData.level),
    message:
      typeof logData.message === 'string'
        ? logData.message.slice(0, 1000)
        : 'Invalid message',
    metadata: logData.metadata ? sanitizeMetadata(logData.metadata) : undefined,
    userId: logData.userId,
  }

  return await safeInvoke<AppLog>(
    'create_log',
    { logData: sanitizedLogData },
    {
      context: { component: 'logs', action: 'create' },
      silent: true, // Don't show errors for logging calls
    }
  )
}

export const getLogs = async (query: LogQuery = {}): Promise<AppLog[]> => {
  return await safeInvoke<AppLog[]>(
    'get_logs',
    { query },
    {
      context: { component: 'logs', action: 'get_all' },
    }
  )
}

export const deleteOldLogs = async (daysOld: number): Promise<string> => {
  return await safeInvoke<string>(
    'delete_old_logs',
    { daysOld },
    {
      context: { component: 'logs', action: 'delete_old' },
    }
  )
}

// Utility functions
export const logError = async (
  message: string,
  metadata?: Record<string, unknown>,
  userId?: string
) => {
  try {
    return await createLog({
      level: 'error',
      message,
      metadata,
      userId,
    })
  } catch (error) {
    // Prevent infinite logging loops - just log to console
    console.error('Failed to log error to backend:', error)
    throw error
  }
}

export const logInfo = async (
  message: string,
  metadata?: Record<string, unknown>,
  userId?: string
) => {
  try {
    return await createLog({
      level: 'info',
      message,
      metadata,
      userId,
    })
  } catch (error) {
    console.warn('Failed to log info to backend:', error)
    throw error
  }
}

export const logDebug = async (
  message: string,
  metadata?: Record<string, unknown>,
  userId?: string
) => {
  try {
    return await createLog({
      level: 'debug',
      message,
      metadata,
      userId,
    })
  } catch (error) {
    console.debug('Failed to log debug to backend:', error)
    throw error
  }
}
