#!/usr/bin/env node

import { execSync } from 'node:child_process'
import { showLogo } from './brand-utils.mjs'

// Show logo once at the start
showLogo()

// Run vite with normal output
try {
  execSync('vite', {
    stdio: 'inherit',
    encoding: 'utf8'
  })
} catch (error) {
  process.exit(error.status || 1)
}