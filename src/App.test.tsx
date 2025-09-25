import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { beforeEach, describe, expect, it } from 'vitest'
import App from './App'
import { useAppStore } from './stores'

const resetStore = () => {
  useAppStore.setState({
    user: null,
    isAuthenticated: false,
    settings: {
      theme: 'system',
      language: 'en',
      sidebarCollapsed: false,
      autoSave: true,
      notifications: true,
    },
    isLoading: false,
    error: null,
  })
}

beforeEach(() => {
  resetStore()
})

describe('App shell', () => {
  it('renders the welcome message', async () => {
    render(<App />)

    await waitFor(() =>
      expect(
        screen.getByRole('heading', { name: /modern desktop app boilerplate/i })
      ).toBeInTheDocument()
    )
  })

  it('toggles between light and dark themes', async () => {
    const user = userEvent.setup()

    render(<App />)

    await waitFor(() =>
      expect(document.documentElement.classList.contains('light')).toBe(true)
    )

    const toggle = screen.getByRole('button', { name: /toggle theme/i })
    await user.click(toggle)

    await waitFor(() =>
      expect(document.documentElement.classList.contains('dark')).toBe(true)
    )
  })
})
