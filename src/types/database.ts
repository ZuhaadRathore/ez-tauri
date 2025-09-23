// Database types matching Rust backend models

export interface User {
  id: string
  email: string
  username: string
  firstName?: string
  lastName?: string
  isActive: boolean
  createdAt: string
}

export interface CreateUser {
  email: string
  username: string
  password: string
  firstName?: string
  lastName?: string
}

export interface UpdateUser {
  email?: string
  username?: string
  firstName?: string
  lastName?: string
  isActive?: boolean
}

export interface LoginRequest {
  email: string
  password: string
}

export interface UserSettings {
  id: string
  userId: string
  theme: string
  language: string
  notificationsEnabled: boolean
  settingsData: Record<string, unknown>
  createdAt: string
  updatedAt: string
}

export interface CreateUserSettings {
  userId: string
  theme?: string
  language?: string
  notificationsEnabled?: boolean
  settingsData?: Record<string, unknown>
}

export interface UpdateUserSettings {
  theme?: string
  language?: string
  notificationsEnabled?: boolean
  settingsData?: Record<string, unknown>
}

export interface AppLog {
  id: string
  level: string
  message: string
  metadata: Record<string, unknown>
  userId?: string
  createdAt: string
}

export interface CreateAppLog {
  level: string
  message: string
  metadata?: Record<string, unknown>
  userId?: string
}

export interface LogQuery {
  level?: string
  userId?: string
  limit?: number
  offset?: number
}

export interface DatabaseStatus {
  connected: boolean
  databaseName?: string
  version?: string
  error?: string
}

export type LogLevel = 'error' | 'warn' | 'info' | 'debug' | 'trace'
