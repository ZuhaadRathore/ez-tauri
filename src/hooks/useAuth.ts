/**
 * React hook for authentication management
 */

import { useEffect, useState } from 'react'
import { AuthService, PublicUser, SessionInfo } from '../api/auth'

interface UseAuthReturn {
  user: PublicUser | null
  isAuthenticated: boolean
  isLoading: boolean
  error: string | null
  login: (email: string, password: string) => Promise<void>
  register: (
    email: string,
    username: string,
    password: string,
    firstName?: string,
    lastName?: string
  ) => Promise<void>
  logout: () => Promise<void>
  logoutAll: () => Promise<void>
  refreshUser: () => Promise<void>
  getSessions: () => Promise<SessionInfo[]>
  revokeSession: (sessionId: string) => Promise<void>
  clearError: () => void
}

/**
 * Hook for managing authentication state
 *
 * @example
 * ```tsx
 * function LoginForm() {
 *   const { login, isLoading, error } = useAuth()
 *
 *   const handleSubmit = async (email: string, password: string) => {
 *     await login(email, password)
 *   }
 *
 *   return <form onSubmit={handleSubmit}>...</form>
 * }
 * ```
 */
export function useAuth(): UseAuthReturn {
  const [user, setUser] = useState<PublicUser | null>(null)
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // Check authentication status on mount
  useEffect(() => {
    checkAuth()
  }, [])

  const checkAuth = async () => {
    try {
      setIsLoading(true)
      const currentUser = await AuthService.getCurrentUser()
      setUser(currentUser)
    } catch (err) {
      console.error('Auth check failed:', err)
      setUser(null)
    } finally {
      setIsLoading(false)
    }
  }

  const login = async (email: string, password: string) => {
    try {
      setIsLoading(true)
      setError(null)
      const loggedInUser = await AuthService.login(email, password)
      setUser(loggedInUser)
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Login failed'
      setError(errorMessage)
      throw err
    } finally {
      setIsLoading(false)
    }
  }

  const register = async (
    email: string,
    username: string,
    password: string,
    firstName?: string,
    lastName?: string
  ) => {
    try {
      setIsLoading(true)
      setError(null)
      const registeredUser = await AuthService.register(
        email,
        username,
        password,
        firstName,
        lastName
      )
      setUser(registeredUser)
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Registration failed'
      setError(errorMessage)
      throw err
    } finally {
      setIsLoading(false)
    }
  }

  const logout = async () => {
    try {
      setIsLoading(true)
      setError(null)
      await AuthService.logout()
      setUser(null)
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Logout failed'
      setError(errorMessage)
      throw err
    } finally {
      setIsLoading(false)
    }
  }

  const logoutAll = async () => {
    try {
      setIsLoading(true)
      setError(null)
      await AuthService.logoutAll()
      setUser(null)
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Logout all failed'
      setError(errorMessage)
      throw err
    } finally {
      setIsLoading(false)
    }
  }

  const refreshUser = async () => {
    try {
      setIsLoading(true)
      const currentUser = await AuthService.getCurrentUser()
      setUser(currentUser)
    } catch (err) {
      console.error('User refresh failed:', err)
      setUser(null)
    } finally {
      setIsLoading(false)
    }
  }

  const getSessions = async (): Promise<SessionInfo[]> => {
    try {
      setError(null)
      return await AuthService.getSessions()
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Failed to get sessions'
      setError(errorMessage)
      throw err
    }
  }

  const revokeSession = async (sessionId: string): Promise<void> => {
    try {
      setError(null)
      await AuthService.revokeSession(sessionId)
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Failed to revoke session'
      setError(errorMessage)
      throw err
    }
  }

  const clearError = () => {
    setError(null)
  }

  return {
    user,
    isAuthenticated: !!user,
    isLoading,
    error,
    login,
    register,
    logout,
    logoutAll,
    refreshUser,
    getSessions,
    revokeSession,
    clearError,
  }
}
