#!/usr/bin/env node

import { execSync } from 'node:child_process'
import { showLogo } from './brand-utils.mjs'

showLogo()

const testType = process.argv[2] || 'run'

switch (testType) {
  case 'ui':
    execSync('vitest --ui', { stdio: 'inherit' })
    break

  case 'coverage':
    execSync('vitest --coverage', { stdio: 'inherit' })
    break

  case 'desktop':
    execSync('npm run tauri:test:build && wdio run wdio.config.ts', { stdio: 'inherit' })
    break

  case 'e2e':
    execSync('npm run test:desktop', { stdio: 'inherit' })
    break

  default:
    execSync('vitest run', { stdio: 'inherit' })
}