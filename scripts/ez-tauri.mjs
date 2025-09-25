#!/usr/bin/env node

import { execSync } from 'node:child_process'
import { showLogo } from './brand-utils.mjs'

const command = process.argv[2] || 'dev'
const args = process.argv.slice(3).join(' ')

// Show logo once at the start
showLogo()

// Run the command with normal output
const tauriCommand = `tauri ${command} ${args}`.trim()

try {
  execSync(tauriCommand, {
    stdio: 'inherit',
    encoding: 'utf8'
  })
} catch (error) {
  process.exit(error.status || 1)
}