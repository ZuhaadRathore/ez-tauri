#!/usr/bin/env node

import { execSync } from 'node:child_process'
import { showLogo } from './brand-utils.mjs'

showLogo()

const shouldFix = process.argv.includes('--fix')

if (shouldFix) {
  try {
    execSync('eslint . --ext ts,tsx --fix', { stdio: 'inherit' })
    execSync('prettier --write "src/**/*.{ts,tsx,js,jsx,json,css,md}"', { stdio: 'inherit' })
    console.log('Code auto-fixed')
  } catch (error) {
    process.exit(1)
  }
} else {
  try {
    execSync('eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0', { stdio: 'inherit' })
    execSync('prettier --check "src/**/*.{ts,tsx,js,jsx,json,css,md}"', { stdio: 'inherit' })
    console.log('Code quality check passed')
  } catch (error) {
    console.error('Code quality issues found. Run with --fix to auto-fix.')
    process.exit(1)
  }
}