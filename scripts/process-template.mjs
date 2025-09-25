#!/usr/bin/env node

import fs from 'fs/promises'
import path from 'path'
import { fileURLToPath } from 'url'
import { createInterface } from 'readline'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)
const PROJECT_ROOT = path.dirname(__dirname)

// Template variables configuration
const TEMPLATE_VARS = {
  PROJECT_NAME: {
    prompt: 'Project name (kebab-case, e.g., my-awesome-app):',
    validate: (value) => /^[a-z][a-z0-9-]*[a-z0-9]$/.test(value) || 'Must be valid npm package name (lowercase, hyphens allowed)',
    required: true
  },
  APP_NAME: {
    prompt: 'App display name (e.g., My Awesome App):',
    validate: (value) => value.trim().length > 0 || 'App name cannot be empty',
    derive: (vars) => vars.PROJECT_NAME.split('-').map(word =>
      word.charAt(0).toUpperCase() + word.slice(1)
    ).join(' '),
    required: true
  },
  APP_DESCRIPTION: {
    prompt: 'App description:',
    default: 'A modern desktop application built with Tauri',
    required: false
  },
  AUTHOR_NAME: {
    prompt: 'Author name:',
    validate: (value) => value.trim().length > 0 || 'Author name cannot be empty',
    required: true
  },
  AUTHOR_EMAIL: {
    prompt: 'Author email (optional):',
    validate: (value) => !value || /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value) || 'Must be valid email or empty',
    required: false
  },
  APP_IDENTIFIER: {
    prompt: 'App identifier (reverse domain notation, e.g., com.company.app):',
    validate: (value) => /^[a-z][a-z0-9]*(\.[a-z][a-z0-9-]*)*$/.test(value) || 'Must be valid reverse domain notation',
    derive: (vars) => {
      const authorSlug = vars.AUTHOR_NAME.toLowerCase().replace(/[^a-z0-9]/g, '')
      return `com.${authorSlug}.${vars.PROJECT_NAME.replace(/-/g, '')}`
    },
    required: true
  },
  RUST_LIB_NAME: {
    derive: (vars) => `${vars.PROJECT_NAME.replace(/-/g, '_')}_lib`
  },
  DATABASE_NAME: {
    derive: (vars) => vars.PROJECT_NAME.replace(/-/g, '_')
  },
  DATABASE_USER: {
    derive: (vars) => `${vars.PROJECT_NAME.replace(/-/g, '_')}_user`
  },
  DATABASE_PASSWORD: {
    derive: () => generateSecurePassword()
  },
  LOG_FILE_PREFIX: {
    derive: (vars) => vars.PROJECT_NAME
  },
  APP_IDENTIFIER_ORG: {
    derive: (vars) => vars.APP_IDENTIFIER.split('.')[0]
  },
  APP_IDENTIFIER_AUTHOR: {
    derive: (vars) => vars.APP_IDENTIFIER.split('.')[1]
  },
  APP_IDENTIFIER_NAME: {
    derive: (vars) => vars.APP_IDENTIFIER.split('.').slice(2).join('.')
  }
}

// Files to process with template variables
const TEMPLATE_FILES = [
  'package.json',
  'src-tauri/Cargo.toml',
  'src-tauri/tauri.conf.json',
  '.env.example',
  'docker-compose.yml',
  'src/layouts/Layout.tsx',
  'src-tauri/src/logging/mod.rs',
  'src-tauri/src/logging/config.rs',
  'README.md'
]

function generateSecurePassword() {
  const chars = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*'
  return Array.from({ length: 16 }, () => chars[Math.floor(Math.random() * chars.length)]).join('')
}

async function promptUser(question, defaultValue = '') {
  const rl = createInterface({
    input: process.stdin,
    output: process.stdout
  })

  return new Promise((resolve) => {
    const prompt = defaultValue ? `${question} (${defaultValue}): ` : `${question} `
    rl.question(prompt, (answer) => {
      rl.close()
      resolve(answer.trim() || defaultValue)
    })
  })
}

async function collectVariables(args = {}) {
  const variables = { ...args }

  console.log('🚀 EZ Tauri Template Processor')
  console.log('======================================\\n')

  // Collect required variables first
  for (const [key, config] of Object.entries(TEMPLATE_VARS)) {
    if (config.derive || variables[key] !== undefined) continue

    let value = variables[key]
    let isValid = false

    while (!isValid) {
      if (!value) {
        value = await promptUser(config.prompt, config.default)
      }

      if (!config.required && !value) {
        isValid = true
      } else if (config.validate) {
        const validation = config.validate(value)
        if (validation === true) {
          isValid = true
        } else {
          console.log(`❌ ${validation}`)
          value = null
        }
      } else {
        isValid = true
      }
    }

    variables[key] = value || ''
  }

  // Derive dependent variables
  for (const [key, config] of Object.entries(TEMPLATE_VARS)) {
    if (config.derive && variables[key] === undefined) {
      variables[key] = config.derive(variables)
    }
  }

  return variables
}

async function processFile(filePath, variables) {
  try {
    const fullPath = path.join(PROJECT_ROOT, filePath)
    let content = await fs.readFile(fullPath, 'utf8')

    // Replace template variables
    for (const [key, value] of Object.entries(variables)) {
      const placeholder = `{{${key}}}`
      content = content.replaceAll(placeholder, value)
    }

    await fs.writeFile(fullPath, content, 'utf8')
    console.log(`✅ Processed: ${filePath}`)
  } catch (error) {
    console.error(`❌ Error processing ${filePath}:`, error.message)
  }
}

async function validateProject() {
  try {
    const packageJson = path.join(PROJECT_ROOT, 'package.json')
    const cargoToml = path.join(PROJECT_ROOT, 'src-tauri', 'Cargo.toml')

    await fs.access(packageJson)
    await fs.access(cargoToml)

    return true
  } catch {
    console.error('❌ This does not appear to be an EZ Tauri project')
    console.error('   Make sure you are in the project root directory')
    return false
  }
}

async function main() {
  console.clear()

  if (!await validateProject()) {
    process.exit(1)
  }

  try {
    // Parse command line arguments
    const args = {}
    for (let i = 2; i < process.argv.length; i += 2) {
      const key = process.argv[i]?.replace(/^--/, '').replace(/-/g, '_').toUpperCase()
      const value = process.argv[i + 1]
      if (key && TEMPLATE_VARS[key]) {
        args[key] = value
      }
    }

    // Collect all variables
    const variables = await collectVariables(args)

    console.log('\\n📋 Configuration Summary:')
    console.log('========================')
    for (const [key, value] of Object.entries(variables)) {
      if (key.includes('PASSWORD')) {
        console.log(`${key}: ${'*'.repeat(value.length)}`)
      } else {
        console.log(`${key}: ${value}`)
      }
    }

    const proceed = await promptUser('\\n🚀 Proceed with template processing? (y/N)', 'n')
    if (proceed.toLowerCase() !== 'y') {
      console.log('❌ Template processing cancelled')
      process.exit(0)
    }

    console.log('\\n🔄 Processing template files...')

    for (const file of TEMPLATE_FILES) {
      await processFile(file, variables)
    }

    console.log('\\n✅ Template processing complete!')
    console.log('\\n📝 Next steps:')
    console.log('   1. Review the generated files')
    console.log('   2. Run npm install to install dependencies')
    console.log('   3. Copy .env.example to .env and adjust if needed')
    console.log('   4. Start development with npm run dev')

  } catch (error) {
    console.error('❌ Error during template processing:', error.message)
    process.exit(1)
  }
}

// Handle CLI usage
if (process.argv[1] === __filename) {
  main().catch(console.error)
}

export { processFile, collectVariables, TEMPLATE_VARS }