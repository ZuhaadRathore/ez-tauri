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

// Contact form schema
export const contactSchema = z.object({
  name: z.string().min(1, 'Name is required').max(100, 'Name is too long'),
  email: z.string().email('Invalid email address'),
  subject: z
    .string()
    .min(1, 'Subject is required')
    .max(200, 'Subject is too long'),
  message: z
    .string()
    .min(10, 'Message must be at least 10 characters')
    .max(1000, 'Message is too long'),
  category: z.enum(['general', 'support', 'bug', 'feature'], {
    message: 'Please select a valid category',
  }),
})

// Settings schema
export const settingsSchema = z.object({
  theme: z.enum(['light', 'dark', 'system']),
  language: z.string().min(2, 'Language must be at least 2 characters'),
  notifications: z.boolean(),
  autoSave: z.boolean(),
  sidebarCollapsed: z.boolean(),
})

// Greeting schema (for the demo form)
export const greetingSchema = z.object({
  name: z.string().min(1, 'Name is required').max(50, 'Name is too long'),
  greeting: z.enum(['hello', 'hi', 'hey', 'greetings'], {
    message: 'Please select a greeting type',
  }),
  includeTime: z.boolean(),
})

// Database connection schema
export const dbConnectionSchema = z.object({
  host: z.string().min(1, 'Host is required'),
  port: z
    .number()
    .min(1, 'Port must be a positive number')
    .max(65535, 'Port must be less than 65536'),
  database: z.string().min(1, 'Database name is required'),
  username: z.string().min(1, 'Username is required'),
  password: z.string().min(1, 'Password is required'),
  ssl: z.boolean().default(false),
})

// Type exports
export type User = z.infer<typeof userSchema>
export type CreateUser = z.infer<typeof createUserSchema>
export type UpdateUser = z.infer<typeof updateUserSchema>
export type Login = z.infer<typeof loginSchema>
export type Register = z.infer<typeof registerSchema>
export type Contact = z.infer<typeof contactSchema>
export type Settings = z.infer<typeof settingsSchema>
export type Greeting = z.infer<typeof greetingSchema>
export type DbConnection = z.infer<typeof dbConnectionSchema>
