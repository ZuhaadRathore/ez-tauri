/**
 * Authentication API for user login, registration, and session management
 */

import { invoke } from '@tauri-apps/api/core'

export interface LoginRequest {
  email: string
  password: string
  deviceInfo?: string
}

export interface LoginResponse {
  accessToken: string
  refreshToken: string
  user: PublicUser
  expiresIn: number
}

export interface RegisterRequest {
  email: string
  username: string
  password: string
  firstName?: string
  lastName?: string
}

export interface PublicUser {
  id: string
  email: string
  username: string
  firstName?: string
  lastName?: string
  isActive: boolean
  createdAt: string
  updatedAt: string
}

export interface RefreshTokenRequest {
  refreshToken: string
}

export interface RefreshTokenResponse {
  accessToken: string
  refreshToken: string
  expiresIn: number
}

export interface SessionInfo {
  id: string
  deviceInfo?: string
  createdAt: string
  lastUsedAt: string
}

/**
 * Register a new user account
 */
export async function register(
  request: RegisterRequest
): Promise<LoginResponse> {
  return invoke<LoginResponse>('auth_register', { request })
}

/**
 * Login with email and password
 */
export async function login(request: LoginRequest): Promise<LoginResponse> {
  return invoke<LoginResponse>('auth_login', { request })
}

/**
 * Logout from current session
 */
export async function logout(refreshToken: string): Promise<void> {
  return invoke<void>('auth_logout', { refreshToken })
}

/**
 * Logout from all devices (revoke all sessions)
 */
export async function logoutAll(accessToken: string): Promise<void> {
  return invoke<void>('auth_logout_all', { accessToken })
}

/**
 * Refresh access token using refresh token
 */
export async function refreshToken(
  request: RefreshTokenRequest
): Promise<RefreshTokenResponse> {
  return invoke<RefreshTokenResponse>('auth_refresh_token', { request })
}

/**
 * Verify access token and get current user
 */
export async function verifyToken(accessToken: string): Promise<PublicUser> {
  return invoke<PublicUser>('auth_verify_token', { accessToken })
}

/**
 * Get all active sessions for current user
 */
export async function getSessions(accessToken: string): Promise<SessionInfo[]> {
  return invoke<SessionInfo[]>('auth_get_sessions', { accessToken })
}

/**
 * Revoke a specific session
 */
export async function revokeSession(
  accessToken: string,
  sessionId: string
): Promise<void> {
  return invoke<void>('auth_revoke_session', { accessToken, sessionId })
}

/**
 * Auth service for managing authentication state
 */
export class AuthService {
  private static readonly ACCESS_TOKEN_KEY = 'access_token'
  private static readonly REFRESH_TOKEN_KEY = 'refresh_token'
  private static readonly USER_KEY = 'user'

  /**
   * Login and store tokens
   */
  static async login(email: string, password: string): Promise<PublicUser> {
    const deviceInfo = `${navigator.platform} - ${navigator.userAgent}`

    const response = await login({
      email,
      password,
      deviceInfo,
    })

    // Store tokens and user info
    this.setAccessToken(response.accessToken)
    this.setRefreshToken(response.refreshToken)
    this.setUser(response.user)

    return response.user
  }

  /**
   * Register new user and store tokens
   */
  static async register(
    email: string,
    username: string,
    password: string,
    firstName?: string,
    lastName?: string
  ): Promise<PublicUser> {
    const response = await register({
      email,
      username,
      password,
      firstName,
      lastName,
    })

    // Store tokens and user info
    this.setAccessToken(response.accessToken)
    this.setRefreshToken(response.refreshToken)
    this.setUser(response.user)

    return response.user
  }

  /**
   * Logout and clear stored tokens
   */
  static async logout(): Promise<void> {
    const refreshToken = this.getRefreshToken()

    if (refreshToken) {
      try {
        await logout(refreshToken)
      } catch (error) {
        console.error('Logout failed:', error)
      }
    }

    this.clearAuth()
  }

  /**
   * Logout from all devices
   */
  static async logoutAll(): Promise<void> {
    const accessToken = this.getAccessToken()

    if (accessToken) {
      try {
        await logoutAll(accessToken)
      } catch (error) {
        console.error('Logout all failed:', error)
      }
    }

    this.clearAuth()
  }

  /**
   * Refresh access token
   */
  static async refreshAccessToken(): Promise<boolean> {
    const refreshTokenValue = this.getRefreshToken()

    if (!refreshTokenValue) {
      return false
    }

    try {
      const response = await refreshToken({
        refreshToken: refreshTokenValue,
      })

      this.setAccessToken(response.accessToken)
      // Refresh token might be rotated
      this.setRefreshToken(response.refreshToken)

      return true
    } catch (error) {
      console.error('Token refresh failed:', error)
      this.clearAuth()
      return false
    }
  }

  /**
   * Get current user from token
   */
  static async getCurrentUser(): Promise<PublicUser | null> {
    const accessToken = this.getAccessToken()

    if (!accessToken) {
      return null
    }

    try {
      const user = await verifyToken(accessToken)
      this.setUser(user)
      return user
    } catch (error) {
      console.error('Token verification failed:', error)

      // Try to refresh token
      const refreshed = await this.refreshAccessToken()
      if (refreshed) {
        const newAccessToken = this.getAccessToken()
        if (newAccessToken) {
          const user = await verifyToken(newAccessToken)
          this.setUser(user)
          return user
        }
      }

      this.clearAuth()
      return null
    }
  }

  /**
   * Check if user is authenticated
   */
  static isAuthenticated(): boolean {
    return !!this.getAccessToken() && !!this.getRefreshToken()
  }

  /**
   * Get all active sessions
   */
  static async getSessions(): Promise<SessionInfo[]> {
    const accessToken = this.getAccessToken()

    if (!accessToken) {
      throw new Error('Not authenticated')
    }

    return getSessions(accessToken)
  }

  /**
   * Revoke a specific session
   */
  static async revokeSession(sessionId: string): Promise<void> {
    const accessToken = this.getAccessToken()

    if (!accessToken) {
      throw new Error('Not authenticated')
    }

    return revokeSession(accessToken, sessionId)
  }

  // Token storage methods
  static setAccessToken(token: string): void {
    localStorage.setItem(this.ACCESS_TOKEN_KEY, token)
  }

  static getAccessToken(): string | null {
    return localStorage.getItem(this.ACCESS_TOKEN_KEY)
  }

  static setRefreshToken(token: string): void {
    localStorage.setItem(this.REFRESH_TOKEN_KEY, token)
  }

  static getRefreshToken(): string | null {
    return localStorage.getItem(this.REFRESH_TOKEN_KEY)
  }

  static setUser(user: PublicUser): void {
    localStorage.setItem(this.USER_KEY, JSON.stringify(user))
  }

  static getUser(): PublicUser | null {
    const userJson = localStorage.getItem(this.USER_KEY)
    return userJson ? JSON.parse(userJson) : null
  }

  static clearAuth(): void {
    localStorage.removeItem(this.ACCESS_TOKEN_KEY)
    localStorage.removeItem(this.REFRESH_TOKEN_KEY)
    localStorage.removeItem(this.USER_KEY)
  }
}
