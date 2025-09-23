import { useTheme } from '../hooks'

interface ThemeToggleProps {
  className?: string
}

const ThemeToggle = ({ className = '' }: ThemeToggleProps) => {
  const { isDark, toggleTheme } = useTheme()

  return (
    <button
      type='button'
      onClick={toggleTheme}
      aria-pressed={isDark}
      className={`flex items-center gap-2 rounded-full border border-slate-300 px-3 py-1 text-sm font-medium transition-colors hover:border-blue-500 hover:text-blue-600 dark:border-slate-700 dark:text-slate-200 dark:hover:border-blue-400 dark:hover:text-blue-300 ${className}`}
      aria-label='Toggle theme'
    >
      <span className='font-mono text-xs uppercase tracking-wide'>
        {isDark ? 'dark' : 'light'}
      </span>
      <span className='hidden sm:inline text-slate-600 dark:text-slate-300'>
        mode
      </span>
    </button>
  )
}

export default ThemeToggle
