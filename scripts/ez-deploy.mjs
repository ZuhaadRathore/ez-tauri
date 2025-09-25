#!/usr/bin/env node

import { execSync } from 'child_process'
import { existsSync } from 'fs'
import { join } from 'path'

const environment = process.argv[2] || 'staging'

console.log(`🚀 Deploying to ${environment}...`)

// Validate environment
if (!['staging', 'production'].includes(environment)) {
  console.error('❌ Environment must be either "staging" or "production"')
  process.exit(1)
}

// Check if environment file exists
const envFile = `.env.${environment}`
if (!existsSync(envFile)) {
  console.error(`❌ Environment file ${envFile} not found`)
  process.exit(1)
}

try {
  // Pre-deployment checks
  console.log('🔍 Running pre-deployment checks...')

  // Type check
  console.log('📝 Type checking...')
  execSync('npm run typecheck', { stdio: 'inherit' })

  // Lint
  console.log('🔍 Linting...')
  execSync('npm run lint', { stdio: 'inherit' })

  // Test
  console.log('🧪 Running tests...')
  execSync('npm run test:run', { stdio: 'inherit' })

  // Build for specific environment
  console.log(`🏗️  Building for ${environment}...`)
  execSync(`npm run build:${environment}`, { stdio: 'inherit' })

  // Build Tauri app
  console.log('📦 Building Tauri application...')
  execSync('npm run tauri:build', { stdio: 'inherit' })

  if (environment === 'production') {
    console.log('🐳 Building production Docker image...')
    execSync('npm run docker:prod:build', { stdio: 'inherit' })

    console.log('⚠️  Remember to:')
    console.log('1. Set production environment variables in your deployment platform')
    console.log('2. Configure your production database and Redis')
    console.log('3. Set up SSL certificates')
    console.log('4. Configure monitoring and logging')
    console.log('5. Run: npm run docker:prod:up')
  }

  console.log(`✅ Successfully deployed to ${environment}!`)

} catch (error) {
  console.error(`❌ Deployment to ${environment} failed:`, error.message)
  process.exit(1)
}