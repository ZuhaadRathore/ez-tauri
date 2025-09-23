import { useEffect, useMemo } from 'react'
import { useAppStore } from '../stores'
import {
  onSystemThemeChange,
  resolveTheme,
  type ThemeMode,
} from '../config/theme'

export const useTheme = () => {
  const { settings, setTheme } = useAppStore()
  const theme = settings.theme

  const appliedTheme = useMemo(() => resolveTheme(theme), [theme])

  useEffect(() => {
    const root = window.document.documentElement

    root.classList.remove('light', 'dark')
    root.classList.add(appliedTheme)
    root.dataset.theme = appliedTheme
  }, [appliedTheme])

  useEffect(() => {
    if (theme !== 'system') {
      return
    }

    return onSystemThemeChange(nextTheme => {
      const root = window.document.documentElement
      root.classList.remove('light', 'dark')
      root.classList.add(nextTheme)
      root.dataset.theme = nextTheme
    })
  }, [theme])

  const toggleTheme = () => {
    const nextTheme: ThemeMode = appliedTheme === 'dark' ? 'light' : 'dark'
    setTheme(nextTheme)
  }

  const setThemeMode = (mode: ThemeMode) => {
    setTheme(mode)
  }

  return {
    theme,
    isDark: appliedTheme === 'dark',
    setTheme: setThemeMode,
    toggleTheme,
  }
}
