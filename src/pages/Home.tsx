const Home = () => {
  return (
    <section className='space-y-8'>
      <header className='space-y-3'>
        <h1 className='text-3xl font-semibold tracking-tight'>
          Welcome aboard
        </h1>
        <p className='max-w-2xl text-sm text-slate-600 dark:text-slate-300'>
          The backend, testing, and tooling are ready; now the UI is yours to
          shape. Start by replacing this placeholder with the screens your
          product needs.
        </p>
      </header>

      <div className='grid gap-4 sm:grid-cols-2'>
        <article className='rounded-xl border border-slate-200 bg-white p-6 shadow-sm dark:border-slate-800 dark:bg-slate-900'>
          <h2 className='text-base font-medium text-slate-800 dark:text-slate-200'>
            Ready-to-use stack
          </h2>
          <ul className='mt-3 space-y-2 text-sm text-slate-600 dark:text-slate-300'>
            <li>- React 18 + TypeScript + Tailwind</li>
            <li>- Tauri commands for native capabilities</li>
            <li>- Vitest &amp; WebdriverIO for testing</li>
          </ul>
        </article>

        <article className='rounded-xl border border-slate-200 bg-white p-6 shadow-sm dark:border-slate-800 dark:bg-slate-900'>
          <h2 className='text-base font-medium text-slate-800 dark:text-slate-200'>
            Where to go next
          </h2>
          <ul className='mt-3 space-y-2 text-sm text-slate-600 dark:text-slate-300'>
            <li>- Design your navigation and core screens</li>
            <li>- Wire up state with Zustand or your preferred store</li>
            <li>- Call into Rust commands from src/api</li>
          </ul>
        </article>
      </div>
    </section>
  )
}

export default Home
