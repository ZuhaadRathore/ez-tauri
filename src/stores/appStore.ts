import { create } from 'zustand'
import { devtools, persist } from 'zustand/middleware'

export interface User {
  id: string
  email: string
  username: string
  firstName?: string
  lastName?: string
}

export interface AppSettings {
  theme: 'light' | 'dark' | 'system'
  language: string
  sidebarCollapsed: boolean
  autoSave: boolean
  notifications: boolean
}

interface AppState {
  user: User | null
  isAuthenticated: boolean
  settings: AppSettings
  isLoading: boolean
  error: string | null
  setUser: (user: User | null) => void
  login: (user: User) => void
  logout: () => void
  updateSettings: (settings: Partial<AppSettings>) => void
  setTheme: (theme: AppSettings['theme']) => void
  toggleSidebar: () => void
  setLoading: (loading: boolean) => void
  setError: (error: string | null) => void
}

const defaultSettings: AppSettings = {
  theme: 'system',
  language: 'en',
  sidebarCollapsed: false,
  autoSave: true,
  notifications: true,
}

const createDefaultSettings = (): AppSettings => ({ ...defaultSettings })

export const useAppStore = create<AppState>()(
  devtools(
    persist(
      set => ({
        user: null,
        isAuthenticated: false,
        settings: createDefaultSettings(),
        isLoading: false,
        error: null,

        setUser: user =>
          set({ user, isAuthenticated: !!user }, false, 'setUser'),

        login: user =>
          set({ user, isAuthenticated: true, error: null }, false, 'login'),

        logout: () =>
          set(
            {
              user: null,
              isAuthenticated: false,
              error: null,
              settings: createDefaultSettings(),
            },
            false,
            'logout'
          ),

        updateSettings: newSettings =>
          set(
            state => ({
              settings: { ...state.settings, ...newSettings },
            }),
            false,
            'updateSettings'
          ),

        setTheme: theme =>
          set(
            state => ({
              settings: { ...state.settings, theme },
            }),
            false,
            'setTheme'
          ),

        toggleSidebar: () =>
          set(
            state => ({
              settings: {
                ...state.settings,
                sidebarCollapsed: !state.settings.sidebarCollapsed,
              },
            }),
            false,
            'toggleSidebar'
          ),

        setLoading: loading => set({ isLoading: loading }, false, 'setLoading'),

        setError: error => set({ error }, false, 'setError'),
      }),
      {
        name: 'tauri-app-storage',
        partialize: state => ({
          settings: state.settings,
        }),
      }
    ),
    {
      name: 'app-store',
    }
  )
)
