import DOMPurify from 'dompurify'

/**
 * Input sanitization utilities for preventing XSS attacks
 */

// HTML sanitization using DOMPurify
export const sanitizeHtml = (input: string): string => {
  if (typeof input !== 'string') return ''
  return DOMPurify.sanitize(input, {
    ALLOWED_TAGS: ['b', 'i', 'em', 'strong', 'p', 'br'],
    ALLOWED_ATTR: [],
  })
}

// Basic text sanitization - strips HTML and dangerous characters
export const sanitizeText = (input: string): string => {
  if (typeof input !== 'string') return ''

  return input
    .replace(/[<>]/g, '') // Remove angle brackets
    .replace(/javascript:/gi, '') // Remove javascript: protocol
    .replace(/on\w+=/gi, '') // Remove event handlers like onclick=
    .trim()
}

// Email sanitization
export const sanitizeEmail = (email: string): string => {
  if (typeof email !== 'string') return ''

  return email
    .toLowerCase()
    .replace(/[<>"'&]/g, '') // Remove potentially dangerous chars
    .trim()
}

// Username sanitization - only allow alphanumeric and underscore
export const sanitizeUsername = (username: string): string => {
  if (typeof username !== 'string') return ''

  return username
    .replace(/[^a-zA-Z0-9_]/g, '') // Only allow alphanumeric and underscore
    .slice(0, 50) // Limit length
    .trim()
}

// Name fields sanitization
export const sanitizeName = (name: string): string => {
  if (typeof name !== 'string') return ''

  return name
    .replace(/[<>"'&]/g, '') // Remove HTML/XSS chars
    .replace(/\s+/g, ' ') // Normalize whitespace
    .slice(0, 100) // Limit length
    .trim()
}

// Sanitize object metadata recursively
export const sanitizeMetadata = (
  obj: Record<string, unknown>
): Record<string, unknown> => {
  const sanitized: Record<string, unknown> = {}

  for (const [key, value] of Object.entries(obj)) {
    const sanitizedKey = sanitizeText(String(key))

    if (typeof value === 'string') {
      sanitized[sanitizedKey] = sanitizeText(value)
    } else if (
      typeof value === 'object' &&
      value !== null &&
      !Array.isArray(value)
    ) {
      sanitized[sanitizedKey] = sanitizeMetadata(
        value as Record<string, unknown>
      )
    } else if (typeof value === 'number' || typeof value === 'boolean') {
      sanitized[sanitizedKey] = value
    } else {
      // Convert other types to sanitized strings
      sanitized[sanitizedKey] = sanitizeText(String(value))
    }
  }

  return sanitized
}

// Validate and sanitize log levels
export const sanitizeLogLevel = (level: string): string => {
  const allowedLevels = ['error', 'warn', 'info', 'debug', 'trace']
  const normalized = level?.toLowerCase?.() || 'info'

  return allowedLevels.includes(normalized) ? normalized : 'info'
}
