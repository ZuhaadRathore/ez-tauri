# 📝 Changelog Generation Guide

This project uses an automated changelog generation system based on conventional commits. This guide explains how to use it effectively.

## 🎯 Overview

Our changelog system:

- ✅ Automatically generates changelogs from git commits
- ✅ Follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format
- ✅ Uses [Conventional Commits](https://www.conventionalcommits.org/) standard
- ✅ Integrates with GitHub release workflow
- ✅ Supports semantic versioning

## 🚀 Quick Start

### Generate Changelog

```bash
npm run changelog                 # Generate/update changelog
npm run changelog:check          # Check if changelog is up to date
```

### Version Management

```bash
npm run version:patch            # 0.1.0 → 0.1.1 (bug fixes)
npm run version:minor            # 0.1.0 → 0.2.0 (new features)
npm run version:major            # 0.1.0 → 1.0.0 (breaking changes)
```

## 📋 Conventional Commit Format

Use this format for all commits to ensure proper changelog generation:

```
type(scope): description

[optional body]

[optional footer]
```

### Commit Types

| Type       | Icon | Description              | Changelog Section |
| ---------- | ---- | ------------------------ | ----------------- |
| `feat`     | 🎉   | New features             | Features          |
| `fix`      | 🐛   | Bug fixes                | Bug Fixes         |
| `docs`     | 📚   | Documentation            | Documentation     |
| `style`    | 💅   | Code formatting          | Styles            |
| `refactor` | ♻️   | Code refactoring         | Code Refactoring  |
| `perf`     | ⚡   | Performance improvements | Performance       |
| `test`     | 🧪   | Test changes             | Tests             |
| `build`    | 🏗️   | Build system             | Build System      |
| `ci`       | 🤖   | CI configuration         | CI                |
| `chore`    | 🔧   | Maintenance              | Chores            |
| `revert`   | ⏪   | Reverted changes         | Reverts           |

### Scopes (Optional)

Common scopes in this project:

- `frontend` - React components and UI
- `backend` - Rust/Tauri handlers
- `database` - Database operations
- `ci` - GitHub Actions workflows
- `docs` - Documentation
- `deps` - Dependencies
- `config` - Configuration files

### Examples

```bash
# Features
git commit -m "feat(frontend): add dark mode toggle to settings"
git commit -m "feat(backend): implement user authentication endpoints"

# Bug fixes
git commit -m "fix(database): resolve connection pool timeout issue"
git commit -m "fix: prevent memory leak in file watcher"

# Documentation
git commit -m "docs: add API documentation for user endpoints"
git commit -m "docs(readme): update installation instructions"

# Breaking changes
git commit -m "feat(backend)!: migrate to new authentication system

BREAKING CHANGE: Auth tokens now use JWT format instead of session cookies"
```

## 🔄 Release Workflow

### 1. Development

Write commits following conventional commit format:

```bash
git commit -m "feat(frontend): add user profile page"
git commit -m "fix(backend): resolve database timeout"
git commit -m "docs: update API documentation"
```

### 2. Prepare Release

Choose version bump type and prepare:

```bash
# For patch releases (bug fixes)
npm run version:patch

# For minor releases (new features)
npm run version:minor

# For major releases (breaking changes)
npm run version:major
```

This will:

- Bump version in `package.json` and `Cargo.toml`
- Generate/update `CHANGELOG.md`
- Create a version commit
- Create a version tag

### 3. Release

Push the tag to trigger automated release:

```bash
git push origin main --tags
```

GitHub Actions will:

- Build for all platforms (Windows, macOS, Linux)
- Generate release notes from changelog
- Create GitHub release with binaries
- Publish the release

## 📁 Generated Files

### CHANGELOG.md

Automatically generated changelog with sections:

```markdown
# Changelog

## [1.2.0] - 2025-01-15

### 🎉 Features

- **frontend**: add dark mode toggle ([abc123](../../commit/abc123))

### 🐛 Bug Fixes

- **backend**: resolve database timeout issue ([def456](../../commit/def456))

### 📚 Documentation

- update API documentation ([ghi789](../../commit/ghi789))
```

### GitHub Release Notes

Generated from template with:

- Version information
- Changelog content
- Installation instructions
- System requirements
- Support links

## ⚙️ Configuration

### .changelogrc.json

Customize commit types, emojis, and scopes:

```json
{
  "types": {
    "feat": {
      "title": "🎉 Features",
      "description": "New features"
    }
  },
  "scopes": {
    "frontend": "Frontend/React components"
  }
}
```

### .github/release-notes-template.md

Customize GitHub release notes template with placeholders:

- `{{VERSION}}` - Version number
- `{{GENERATED_NOTES}}` - Changelog content

## 🛠️ Manual Operations

### Force Regenerate Changelog

```bash
npm run changelog -- --force
```

### Check Specific Version

```bash
node scripts/generate-changelog.mjs --check
```

### Custom Date Range

Edit `scripts/generate-changelog.mjs` to customize git log range.

## 🎯 Best Practices

### Commit Messages

- Use imperative mood: "add feature" not "added feature"
- Keep first line under 50 characters
- Use body for detailed explanations
- Reference issues: "closes #123"

### Versioning

- **Patch** (0.1.1): Bug fixes, documentation
- **Minor** (0.2.0): New features, backward compatible
- **Major** (1.0.0): Breaking changes

### Release Frequency

- Patch releases: As needed for critical fixes
- Minor releases: Weekly/bi-weekly for features
- Major releases: When introducing breaking changes

## 🐛 Troubleshooting

### "No commits found"

- Ensure you have commits since the last tag
- Check git log output
- Use `--force` to regenerate anyway

### "Version mismatch"

- Ensure `package.json` and `Cargo.toml` versions match
- Run version bump scripts to sync versions

### "Template not found"

- Ensure `.github/release-notes-template.md` exists
- Check file permissions

### Duplicate entries

- Don't manually edit changelog after generation
- Use `--force` flag to regenerate sections

## 📚 References

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
- [Semantic Versioning](https://semver.org/)
- [GitHub Release API](https://docs.github.com/en/rest/releases)
