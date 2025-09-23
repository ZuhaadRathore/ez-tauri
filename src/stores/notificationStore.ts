import { create } from 'zustand'
import { devtools } from 'zustand/middleware'

export interface Notification {
  id: string
  type: 'success' | 'error' | 'warning' | 'info'
  title: string
  message?: string
  duration?: number
  persistent?: boolean
  createdAt: Date
}

interface NotificationState {
  notifications: Notification[]

  // Actions
  addNotification: (
    notification: Omit<Notification, 'id' | 'createdAt'>
  ) => void
  removeNotification: (id: string) => void
  clearAll: () => void

  // Convenience methods
  success: (
    title: string,
    message?: string,
    options?: Partial<Notification>
  ) => void
  error: (
    title: string,
    message?: string,
    options?: Partial<Notification>
  ) => void
  warning: (
    title: string,
    message?: string,
    options?: Partial<Notification>
  ) => void
  info: (
    title: string,
    message?: string,
    options?: Partial<Notification>
  ) => void
}

const generateId = () => Math.random().toString(36).substring(2, 15)

export const useNotificationStore = create<NotificationState>()(
  devtools(
    (set, get) => ({
      notifications: [],

      addNotification: notification => {
        const newNotification: Notification = {
          id: generateId(),
          createdAt: new Date(),
          duration: 5000, // 5 seconds default
          persistent: false,
          ...notification,
        }

        set(
          state => ({
            notifications: [...state.notifications, newNotification],
          }),
          false,
          'addNotification'
        )

        // Auto-remove notification after duration (unless persistent)
        if (!newNotification.persistent && newNotification.duration) {
          setTimeout(() => {
            get().removeNotification(newNotification.id)
          }, newNotification.duration)
        }
      },

      removeNotification: id =>
        set(
          state => ({
            notifications: state.notifications.filter(n => n.id !== id),
          }),
          false,
          'removeNotification'
        ),

      clearAll: () => set({ notifications: [] }, false, 'clearAll'),

      // Convenience methods
      success: (title, message, options = {}) =>
        get().addNotification({
          type: 'success',
          title,
          message,
          ...options,
        }),

      error: (title, message, options = {}) =>
        get().addNotification({
          type: 'error',
          title,
          message,
          duration: 8000, // Longer duration for errors
          ...options,
        }),

      warning: (title, message, options = {}) =>
        get().addNotification({
          type: 'warning',
          title,
          message,
          duration: 6000,
          ...options,
        }),

      info: (title, message, options = {}) =>
        get().addNotification({
          type: 'info',
          title,
          message,
          ...options,
        }),
    }),
    {
      name: 'notification-store',
    }
  )
)
