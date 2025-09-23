import { invoke } from '@tauri-apps/api/core'
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

// Database management
export const checkDatabaseConnection = async (): Promise<DatabaseStatus> => {
  return await invoke('check_database_connection')
}

export const initializeDatabase = async (): Promise<string> => {
  return await invoke('initialize_database')
}

export const runMigrations = async (): Promise<string> => {
  return await invoke('run_migrations')
}

// User management
export const getAllUsers = async (): Promise<User[]> => {
  return await invoke('get_all_users')
}

export const getUserById = async (userId: string): Promise<User | null> => {
  return await invoke('get_user_by_id', { userId })
}

export const createUser = async (userData: CreateUser): Promise<User> => {
  return await invoke('create_user', { userData })
}

export const updateUser = async (
  userId: string,
  userData: UpdateUser
): Promise<User> => {
  return await invoke('update_user', { userId, userData })
}

export const deleteUser = async (userId: string): Promise<string> => {
  return await invoke('delete_user', { userId })
}

export const authenticateUser = async (
  loginData: LoginRequest
): Promise<User | null> => {
  return await invoke('authenticate_user', { loginData })
}

// Logging
export const createLog = async (logData: CreateAppLog): Promise<AppLog> => {
  return await invoke('create_log', { logData })
}

export const getLogs = async (query: LogQuery = {}): Promise<AppLog[]> => {
  return await invoke('get_logs', { query })
}

export const deleteOldLogs = async (daysOld: number): Promise<string> => {
  return await invoke('delete_old_logs', { daysOld })
}

// Utility functions
export const logError = async (
  message: string,
  metadata?: Record<string, unknown>,
  userId?: string
) => {
  return await createLog({
    level: 'error',
    message,
    metadata,
    userId,
  })
}

export const logInfo = async (
  message: string,
  metadata?: Record<string, unknown>,
  userId?: string
) => {
  return await createLog({
    level: 'info',
    message,
    metadata,
    userId,
  })
}

export const logDebug = async (
  message: string,
  metadata?: Record<string, unknown>,
  userId?: string
) => {
  return await createLog({
    level: 'debug',
    message,
    metadata,
    userId,
  })
}
