import { z } from 'zod'

// User schemas
export const userSchema = z.object({
  id: z.string().uuid(),
  email: z.string().email('Invalid email address'),
  username: z
    .string()
    .min(3, 'Username must be at least 3 characters')
    .max(50, 'Username must be less than 50 characters')
    .regex(
      /^[a-zA-Z0-9_]+$/,
      'Username can only contain letters, numbers, and underscores'
    ),
  firstName: z
    .string()
    .min(1, 'First name is required')
    .max(100, 'First name is too long'),
  lastName: z
    .string()
    .min(1, 'Last name is required')
    .max(100, 'Last name is too long'),
})

export const createUserSchema = userSchema.omit({ id: true })

export const updateUserSchema = userSchema.partial().omit({ id: true })

// Login schema
export const loginSchema = z.object({
  email: z.string().email('Invalid email address'),
  password: z.string().min(6, 'Password must be at least 6 characters'),
  rememberMe: z.boolean(),
})

// Registration schema
export const registerSchema = z
  .object({
    email: z.string().email('Invalid email address'),
    username: z
      .string()
      .min(3, 'Username must be at least 3 characters')
      .max(50, 'Username must be less than 50 characters')
      .regex(
        /^[a-zA-Z0-9_]+$/,
        'Username can only contain letters, numbers, and underscores'
      ),
    firstName: z
      .string()
      .min(1, 'First name is required')
      .max(100, 'First name is too long'),
    lastName: z
      .string()
      .min(1, 'Last name is required')
      .max(100, 'Last name is too long'),
    password: z.string().min(6, 'Password must be at least 6 characters'),
    confirmPassword: z.string(),
    acceptTerms: z
      .boolean()
      .refine(val => val === true, 'You must accept the terms and conditions'),
  })
  .refine(data => data.password === data.confirmPassword, {
    message: 'Passwords do not match',
    path: ['confirmPassword'],
  })

// Settings schema
export const settingsSchema = z.object({
  theme: z.enum(['light', 'dark', 'system']),
  language: z.string().min(2, 'Language must be at least 2 characters'),
  notifications: z.boolean(),
  autoSave: z.boolean(),
  sidebarCollapsed: z.boolean(),
})

// Type exports
export type User = z.infer<typeof userSchema>
export type CreateUser = z.infer<typeof createUserSchema>
export type UpdateUser = z.infer<typeof updateUserSchema>
export type Login = z.infer<typeof loginSchema>
export type Register = z.infer<typeof registerSchema>
export type Settings = z.infer<typeof settingsSchema>
