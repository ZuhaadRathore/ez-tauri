const Home = () => {
  return (
    <div className='space-y-16'>
      <section className='text-center space-y-6'>
        <div className='space-y-4'>
          <h1 className='text-4xl font-bold tracking-tight text-slate-900 dark:text-slate-100 sm:text-5xl'>
            Modern Desktop App Boilerplate
          </h1>
          <p className='mx-auto max-w-2xl text-lg text-slate-600 dark:text-slate-300'>
            Get started building cross-platform desktop applications with React,
            TypeScript, and Tauri. Everything you need is already configured and
            ready to go.
          </p>
        </div>

        <div className='flex flex-col sm:flex-row gap-4 justify-center items-center'>
          <button className='px-6 py-3 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 transition-colors'>
            Get Started
          </button>
          <button className='px-6 py-3 border border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 font-medium rounded-lg hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors'>
            View Documentation
          </button>
        </div>
      </section>

      <section className='grid gap-6 sm:grid-cols-2 lg:grid-cols-4'>
        <div className='text-center space-y-3'>
          <div className='mx-auto w-12 h-12 bg-blue-100 dark:bg-blue-900 rounded-lg flex items-center justify-center'>
            <svg
              className='w-6 h-6 text-blue-600 dark:text-blue-400'
              fill='none'
              stroke='currentColor'
              viewBox='0 0 24 24'
            >
              <path
                strokeLinecap='round'
                strokeLinejoin='round'
                strokeWidth={2}
                d='M13 10V3L4 14h7v7l9-11h-7z'
              />
            </svg>
          </div>
          <h3 className='font-semibold text-slate-900 dark:text-slate-100'>
            Lightning Fast
          </h3>
          <p className='text-sm text-slate-600 dark:text-slate-300'>
            Built with Tauri for native performance and small bundle sizes
          </p>
        </div>

        <div className='text-center space-y-3'>
          <div className='mx-auto w-12 h-12 bg-green-100 dark:bg-green-900 rounded-lg flex items-center justify-center'>
            <svg
              className='w-6 h-6 text-green-600 dark:text-green-400'
              fill='none'
              stroke='currentColor'
              viewBox='0 0 24 24'
            >
              <path
                strokeLinecap='round'
                strokeLinejoin='round'
                strokeWidth={2}
                d='M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z'
              />
            </svg>
          </div>
          <h3 className='font-semibold text-slate-900 dark:text-slate-100'>
            Type Safe
          </h3>
          <p className='text-sm text-slate-600 dark:text-slate-300'>
            Full TypeScript support with strict type checking
          </p>
        </div>

        <div className='text-center space-y-3'>
          <div className='mx-auto w-12 h-12 bg-purple-100 dark:bg-purple-900 rounded-lg flex items-center justify-center'>
            <svg
              className='w-6 h-6 text-purple-600 dark:text-purple-400'
              fill='none'
              stroke='currentColor'
              viewBox='0 0 24 24'
            >
              <path
                strokeLinecap='round'
                strokeLinejoin='round'
                strokeWidth={2}
                d='M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zM21 5a2 2 0 00-2-2h-4a2 2 0 00-2 2v12a4 4 0 004 4h4a2 2 0 002-2V5z'
              />
            </svg>
          </div>
          <h3 className='font-semibold text-slate-900 dark:text-slate-100'>
            Cross Platform
          </h3>
          <p className='text-sm text-slate-600 dark:text-slate-300'>
            Deploy to Windows, macOS, and Linux from a single codebase
          </p>
        </div>

        <div className='text-center space-y-3'>
          <div className='mx-auto w-12 h-12 bg-orange-100 dark:bg-orange-900 rounded-lg flex items-center justify-center'>
            <svg
              className='w-6 h-6 text-orange-600 dark:text-orange-400'
              fill='none'
              stroke='currentColor'
              viewBox='0 0 24 24'
            >
              <path
                strokeLinecap='round'
                strokeLinejoin='round'
                strokeWidth={2}
                d='M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z'
              />
            </svg>
          </div>
          <h3 className='font-semibold text-slate-900 dark:text-slate-100'>
            Modern Tooling
          </h3>
          <p className='text-sm text-slate-600 dark:text-slate-300'>
            Vite, ESLint, Prettier, and comprehensive testing setup
          </p>
        </div>
      </section>

      <section className='bg-slate-50 dark:bg-slate-900 rounded-2xl p-8 text-center space-y-6'>
        <div className='space-y-3'>
          <h2 className='text-2xl font-semibold text-slate-900 dark:text-slate-100'>
            Ready to Build Something Amazing?
          </h2>
          <p className='text-slate-600 dark:text-slate-300 max-w-2xl mx-auto'>
            This boilerplate includes everything you need: React 18, TypeScript,
            Tailwind CSS, comprehensive testing with Vitest and WebdriverIO, and
            seamless Rust integration through Tauri.
          </p>
        </div>

        <div className='space-y-4'>
          <div className='text-left bg-slate-900 dark:bg-slate-800 rounded-lg p-4 font-mono text-sm text-slate-100'>
            <div className='text-slate-400'># Clone and start developing</div>
            <div className='text-green-400'>npm install</div>
            <div className='text-green-400'>npm run dev</div>
          </div>

          <p className='text-sm text-slate-500 dark:text-slate-400'>
            Start building your next desktop application in seconds, not hours.
          </p>
        </div>
      </section>
    </div>
  )
}

export default Home
