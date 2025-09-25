#!/usr/bin/env node

import { execSync } from 'node:child_process'
import { showLogo } from './brand-utils.mjs'

showLogo()

const action = process.argv[2] || 'up'

switch (action) {
  case 'up':
    execSync('docker-compose up -d postgres redis', { stdio: 'inherit' })
    console.log('Database services started')
    break

  case 'down':
    execSync('docker-compose down', { stdio: 'inherit' })
    console.log('Database services stopped')
    break

  case 'reset':
    execSync('docker-compose down -v && docker-compose up -d postgres redis', { stdio: 'inherit' })
    console.log('Database reset completed')
    break

  case 'logs':
    execSync('docker-compose logs -f postgres', { stdio: 'inherit' })
    break

  default:
    console.log('Available actions: up, down, reset, logs')
    process.exit(1)
}