#!/usr/bin/env node
import { spawn } from 'node:child_process'
const npmCommand = process.platform === 'win32' ? 'npm.cmd' : 'npm'

const env = {
  ...process.env,
  TAURI_DEV_HOST: process.env.TAURI_DEV_HOST ?? '127.0.0.1',
  PLAYWRIGHT_TAURI: '1',
}

const child = spawn(npmCommand, ['run', 'tauri:dev'], {
  env,
  stdio: 'inherit',
  shell: process.platform === 'win32',
})

const shutdown = (signal) => {
  if (!child.killed) {
    child.kill(signal)
  }
}

process.on('SIGINT', () => shutdown('SIGINT'))
process.on('SIGTERM', () => shutdown('SIGTERM'))
process.on('exit', () => shutdown('SIGTERM'))

child.on('error', (error) => {
  console.error('[desktop-e2e] failed to launch `tauri dev`', error)
  process.exit(1)
})

child.on('exit', (code) => {
  process.exit(code ?? 0)
})
