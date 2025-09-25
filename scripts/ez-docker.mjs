#!/usr/bin/env node

import { execSync } from 'node:child_process'
import { showLogo } from './brand-utils.mjs'

showLogo()

const action = process.argv[2] || 'up'

switch (action) {
  case 'up':
    execSync('docker-compose up -d', { stdio: 'inherit' })
    console.log('All containers started')
    break

  case 'down':
    execSync('docker-compose down', { stdio: 'inherit' })
    console.log('All containers stopped')
    break

  case 'logs':
    execSync('docker-compose logs -f', { stdio: 'inherit' })
    break

  case 'build':
    execSync('docker-compose build', { stdio: 'inherit' })
    console.log('Containers built successfully')
    break

  case 'restart':
    execSync('docker-compose restart', { stdio: 'inherit' })
    console.log('All containers restarted')
    break

  default:
    console.log('Available actions: up, down, logs, build, restart')
    process.exit(1)
}