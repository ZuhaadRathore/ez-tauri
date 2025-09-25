#!/usr/bin/env node

import { execSync } from 'node:child_process'
import { showLogo } from './brand-utils.mjs'

showLogo()

try {
  execSync('tsc', { stdio: 'inherit' })
  execSync('vite build', { stdio: 'inherit' })
  console.log('Build completed')
} catch (error) {
  process.exit(1)
}