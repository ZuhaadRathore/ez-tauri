import { NavLink, Outlet } from 'react-router-dom'
import { ThemeToggle } from '../components'

const navigation = [{ name: 'Home', to: '/' }]

const Layout = () => {
  return (
    <div className='min-h-screen bg-white text-slate-900 transition-colors dark:bg-slate-950 dark:text-slate-100'>
      <header className='border-b border-slate-200 dark:border-slate-800'>
        <div className='mx-auto flex max-w-4xl items-center justify-between px-4 py-5'>
          <NavLink to='/' className='text-lg font-semibold tracking-tight'>
            tavuc-boilerplate
          </NavLink>

          <nav aria-label='Primary navigation'>
            <ul className='flex items-center gap-6 text-sm font-medium'>
              {navigation.map(item => (
                <li key={item.to}>
                  <NavLink
                    to={item.to}
                    end
                    className={({ isActive }) =>
                      `transition-colors hover:text-blue-600 ${
                        isActive
                          ? 'text-blue-600'
                          : 'text-slate-600 dark:text-slate-300'
                      }`
                    }
                  >
                    {item.name}
                  </NavLink>
                </li>
              ))}
            </ul>
          </nav>

          <ThemeToggle className='ml-6' />
        </div>
      </header>

      <main className='mx-auto max-w-4xl px-4 py-10'>
        <Outlet />
      </main>
    </div>
  )
}

export default Layout
