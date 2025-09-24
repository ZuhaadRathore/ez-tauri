#!/usr/bin/env node
import { execSync } from 'node:child_process'
import { readFileSync, writeFileSync, existsSync } from 'node:fs'
import { join } from 'node:path'

const workspaceRoot = process.cwd()
const changelogPath = join(workspaceRoot, 'CHANGELOG.md')
const packageJsonPath = join(workspaceRoot, 'package.json')

// Get command line arguments
const args = process.argv.slice(2)
const isCheckMode = args.includes('--check')
const forceGenerate = args.includes('--force')

// Conventional commit types and their descriptions
const COMMIT_TYPES = {
  feat: { title: 'Features', description: 'New features' },
  fix: { title: 'Bug Fixes', description: 'Bug fixes' },
  docs: { title: 'Documentation', description: 'Documentation changes' },
  style: { title: 'Styles', description: 'Code style changes (formatting, etc)' },
  refactor: { title: 'Code Refactoring', description: 'Code refactoring' },
  perf: { title: 'Performance Improvements', description: 'Performance improvements' },
  test: { title: 'Tests', description: 'Test changes' },
  build: { title: 'Build System', description: 'Build system changes' },
  ci: { title: 'Continuous Integration', description: 'CI configuration changes' },
  chore: { title: 'Chores', description: 'Other changes' },
  revert: { title: 'Reverts', description: 'Reverted changes' }
}

/**
 * Execute git command and return output
 */
function gitCommand(command) {
  try {
    return execSync(command, { encoding: 'utf8' }).trim()
  } catch (error) {
    console.error(`Git command failed: ${command}`)
    console.error(error.message)
    return ''
  }
}

/**
 * Get the current version from package.json
 */
function getCurrentVersion() {
  try {
    const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8'))
    return packageJson.version
  } catch (error) {
    console.error('Failed to read package.json:', error.message)
    process.exit(1)
  }
}

/**
 * Get the latest git tag
 */
function getLatestTag() {
  const tags = gitCommand('git tag --sort=-version:refname')
  return tags.split('\n')[0] || null
}

/**
 * Get commits since the last tag or all commits if no tags exist
 */
function getCommitsSinceTag(tag) {
  const range = tag ? `${tag}..HEAD` : ''
  const command = `git log ${range} --pretty=format:"%H|%s|%an|%ad" --date=short`
  const output = gitCommand(command)

  if (!output) return []

  return output.split('\n').map(line => {
    const [hash, message, author, date] = line.split('|')
    return { hash, message, author, date }
  })
}

/**
 * Parse conventional commit message
 */
function parseConventionalCommit(message) {
  // Regex to match conventional commits: type(scope): description
  const conventionalRegex = /^(\w+)(?:\(([^)]+)\))?: (.+)$/
  const match = message.match(conventionalRegex)

  if (match) {
    const [, type, scope, description] = match
    return {
      type: type.toLowerCase(),
      scope: scope || null,
      description,
      isBreaking: message.includes('BREAKING CHANGE') || message.includes('!'),
      isConventional: true
    }
  }

  // If not conventional, try to categorize by keywords
  const lowerMessage = message.toLowerCase()
  let type = 'chore'

  if (lowerMessage.includes('feat') || lowerMessage.includes('add') || lowerMessage.includes('implement')) {
    type = 'feat'
  } else if (lowerMessage.includes('fix') || lowerMessage.includes('bug')) {
    type = 'fix'
  } else if (lowerMessage.includes('doc')) {
    type = 'docs'
  } else if (lowerMessage.includes('test')) {
    type = 'test'
  } else if (lowerMessage.includes('refactor')) {
    type = 'refactor'
  } else if (lowerMessage.includes('style')) {
    type = 'style'
  } else if (lowerMessage.includes('perf')) {
    type = 'perf'
  } else if (lowerMessage.includes('build') || lowerMessage.includes('ci')) {
    type = 'build'
  }

  return {
    type,
    scope: null,
    description: message,
    isBreaking: false,
    isConventional: false
  }
}

/**
 * Group commits by type
 */
function groupCommitsByType(commits) {
  const groups = {}

  commits.forEach(commit => {
    const parsed = parseConventionalCommit(commit.message)
    const type = parsed.type

    if (!groups[type]) {
      groups[type] = []
    }

    groups[type].push({
      ...commit,
      ...parsed
    })
  })

  return groups
}

/**
 * Generate markdown for a version section
 */
function generateVersionSection(version, commits, date = new Date().toISOString().split('T')[0]) {
  if (commits.length === 0) {
    return `## [${version}] - ${date}\n\nNo significant changes.\n`
  }

  const grouped = groupCommitsByType(commits)
  let markdown = `## [${version}] - ${date}\n\n`

  // Handle breaking changes first
  const breakingChanges = commits.filter(commit =>
    parseConventionalCommit(commit.message).isBreaking
  )

  if (breakingChanges.length > 0) {
    markdown += '### ⚠ BREAKING CHANGES\n\n'
    breakingChanges.forEach(commit => {
      const parsed = parseConventionalCommit(commit.message)
      const scope = parsed.scope ? `**${parsed.scope}**: ` : ''
      markdown += `- ${scope}${parsed.description} ([${commit.hash.substring(0, 7)}](../../commit/${commit.hash}))\n`
    })
    markdown += '\n'
  }

  // Process each commit type in order of importance
  const typeOrder = ['feat', 'fix', 'perf', 'refactor', 'docs', 'test', 'build', 'ci', 'style', 'chore', 'revert']

  typeOrder.forEach(type => {
    if (grouped[type] && grouped[type].length > 0) {
      const typeInfo = COMMIT_TYPES[type] || { title: type.charAt(0).toUpperCase() + type.slice(1) }
      markdown += `### ${typeInfo.title}\n\n`

      grouped[type].forEach(commit => {
        const scope = commit.scope ? `**${commit.scope}**: ` : ''
        const breakingBadge = commit.isBreaking ? ' ⚠️' : ''
        markdown += `- ${scope}${commit.description}${breakingBadge} ([${commit.hash.substring(0, 7)}](../../commit/${commit.hash}))\n`
      })
      markdown += '\n'
    }
  })

  return markdown
}

/**
 * Read existing changelog
 */
function readExistingChangelog() {
  if (!existsSync(changelogPath)) {
    return ''
  }
  return readFileSync(changelogPath, 'utf8')
}

/**
 * Generate initial changelog header
 */
function generateChangelogHeader() {
  return `# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

`
}

/**
 * Update or create changelog
 */
function updateChangelog(version, commits) {
  const existingChangelog = readExistingChangelog()
  const newSection = generateVersionSection(version, commits)

  let updatedChangelog

  if (!existingChangelog) {
    // Create new changelog
    updatedChangelog = generateChangelogHeader() + newSection
  } else {
    // Check if version already exists
    if (existingChangelog.includes(`## [${version}]`)) {
      if (!forceGenerate) {
        console.log(`Version ${version} already exists in changelog. Use --force to regenerate.`)
        return false
      }

      // Replace existing version section (more precise regex)
      const versionRegex = new RegExp(`## \\[${version}\\][\\s\\S]*?(?=## \\[|$)`, 'm')
      updatedChangelog = existingChangelog.replace(versionRegex, newSection)
    } else {
      // Insert new version at the top (after header)
      const headerMatch = existingChangelog.match(/^# Changelog[\s\S]*?(?=##)/m)
      if (headerMatch) {
        const header = headerMatch[0]
        const rest = existingChangelog.substring(header.length)
        updatedChangelog = header + newSection + rest
      } else {
        // Fallback: prepend to existing content
        updatedChangelog = generateChangelogHeader() + newSection + existingChangelog
      }
    }
  }

  writeFileSync(changelogPath, updatedChangelog)
  return true
}

/**
 * Check if changelog needs to be updated
 */
function checkChangelog() {
  const version = getCurrentVersion()
  const latestTag = getLatestTag()
  const commits = getCommitsSinceTag(latestTag)

  console.log(`Current version: ${version}`)
  console.log(`Latest tag: ${latestTag || 'none'}`)
  console.log(`Commits since last tag: ${commits.length}`)

  if (commits.length === 0) {
    console.log('✅ No new commits to add to changelog')
    return true
  }

  const existingChangelog = readExistingChangelog()
  if (!existingChangelog.includes(`## [${version}]`)) {
    console.log(`❌ Changelog is missing entry for version ${version}`)
    console.log('Run `npm run changelog` to generate it')
    return false
  }

  console.log('✅ Changelog is up to date')
  return true
}

/**
 * Main function
 */
function main() {
  console.log('🔍 Generating changelog...\n')

  if (isCheckMode) {
    const isValid = checkChangelog()
    process.exit(isValid ? 0 : 1)
  }

  const version = getCurrentVersion()
  const latestTag = getLatestTag()
  const commits = getCommitsSinceTag(latestTag)

  console.log(`Current version: ${version}`)
  console.log(`Latest tag: ${latestTag || 'none'}`)
  console.log(`Commits to include: ${commits.length}`)

  if (commits.length === 0 && !forceGenerate) {
    console.log('\n✅ No new commits found. Use --force to regenerate anyway.')
    return
  }

  // Show commit summary
  if (commits.length > 0) {
    console.log('\nCommits to include:')
    const grouped = groupCommitsByType(commits)
    Object.keys(grouped).forEach(type => {
      const typeInfo = COMMIT_TYPES[type] || { title: type }
      console.log(`  ${typeInfo.title}: ${grouped[type].length}`)
    })
  }

  const success = updateChangelog(version, commits)
  if (success) {
    console.log(`\n✅ Changelog updated for version ${version}`)
    console.log(`📝 Check ${changelogPath}`)
  }
}

// Run the script
main()