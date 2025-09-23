export type ThemeMode = 'light' | 'dark' | 'system'

type ResolvedTheme = 'light' | 'dark'

const prefersDark = (): boolean => {
  if (
    typeof window === 'undefined' ||
    typeof window.matchMedia !== 'function'
  ) {
    return false
  }

  return window.matchMedia('(prefers-color-scheme: dark)').matches
}

export const resolveTheme = (mode: ThemeMode): ResolvedTheme => {
  if (mode === 'system') {
    return prefersDark() ? 'dark' : 'light'
  }

  return mode
}

export const onSystemThemeChange = (
  callback: (theme: ResolvedTheme) => void
): (() => void) => {
  if (
    typeof window === 'undefined' ||
    typeof window.matchMedia !== 'function'
  ) {
    return () => {}
  }

  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  const handler = (event: MediaQueryListEvent) => {
    callback(event.matches ? 'dark' : 'light')
  }

  mediaQuery.addEventListener('change', handler)
  return () => mediaQuery.removeEventListener('change', handler)
}
