# Contributing to Tauri Boilerplate

Thank you for your interest in contributing to the Tauri Boilerplate! This guide will help you get started.

## 📋 Prerequisites

- Node.js 18+
- Rust 1.70+
- Docker and Docker Compose (for database)
- Git

## 🔄 Development Workflow

1. **Fork and clone the repository**

   ```bash
   git clone https://github.com/YOUR_USERNAME/tauri-boilerplate.git
   cd tauri-boilerplate
   ```

2. **Install dependencies**

   ```bash
   npm install
   ```

3. **Start the development environment**

   ```bash
   # Start database services
   npm run db:up

   # Start the development server
   npm run dev
   ```

4. **Make your changes and test**

   ```bash
   # Run tests
   npm test
   npm run test:desktop
   cargo test --manifest-path src-tauri/Cargo.toml

   # Check code quality
   npm run lint
   npm run typecheck
   ```

## 📝 Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/) for automatic changelog generation:

- `feat: add new feature` - New features
- `fix: resolve bug in component` - Bug fixes
- `docs: update README` - Documentation changes
- `style: fix formatting` - Code style changes
- `refactor: improve code structure` - Code refactoring
- `perf: optimize database queries` - Performance improvements
- `test: add unit tests` - Test additions/changes
- `build: update dependencies` - Build system changes
- `ci: update workflow` - CI configuration changes
- `chore: update tooling` - Other changes

### Examples:

```bash
git commit -m "feat: add dark mode toggle to settings"
git commit -m "fix: resolve authentication redirect issue"
git commit -m "docs: add API documentation for users endpoint"
git commit -m "test: add integration tests for payment flow"
```

## 🚀 Release Process

### For Contributors

1. Create a feature branch from `main`
2. Make your changes following our conventions
3. Submit a pull request with a clear description

### For Maintainers

1. **Create a release** (automated changelog generation):

   ```bash
   # For patch releases (0.1.0 -> 0.1.1)
   npm run version:patch

   # For minor releases (0.1.0 -> 0.2.0)
   npm run version:minor

   # For major releases (0.1.0 -> 1.0.0)
   npm run version:major
   ```

2. **Push the version tag**:

   ```bash
   git push origin main --tags
   ```

3. **GitHub Actions will automatically**:
   - Build the application for all platforms
   - Generate release notes from the changelog
   - Create a GitHub release with downloadable artifacts

## 🧪 Testing

### Backend Tests

```bash
cd src-tauri
cargo test
```

### Frontend Tests

```bash
npm test                    # Unit tests
npm run test:desktop        # Desktop E2E tests
npm run test:coverage       # Coverage report
```

### Manual Testing

```bash
npm run tauri:build        # Test production build
```

## 📖 Documentation

- Update relevant documentation when adding features
- Follow JSDoc conventions for TypeScript functions
- Add Rust documentation for public APIs
- Update README.md if needed

## 🐛 Bug Reports

Use our [bug report template](https://github.com/YOUR_USERNAME/YOUR_REPO/issues/new?template=bug_report.md) and include:

- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment information
- Screenshots if applicable

## 💡 Feature Requests

Use our [feature request template](https://github.com/YOUR_USERNAME/YOUR_REPO/issues/new?template=feature_request.md) and describe:

- The problem you're solving
- Proposed solution
- Alternative solutions considered
- Additional context

## 📏 Code Standards

### TypeScript/React

- Use TypeScript strict mode
- Follow React hooks conventions
- Use Zod for runtime validation
- Prefer function components over class components

### Rust

- Follow Rust naming conventions
- Use `anyhow` for error handling
- Write comprehensive tests for handlers
- Document public APIs

### General

- Write descriptive commit messages
- Keep functions small and focused
- Add tests for new functionality
- Ensure code passes all lints

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## 🙏 Questions?

- Create a [Discussion](https://github.com/YOUR_USERNAME/YOUR_REPO/discussions)
- Join our community chat (if applicable)
- Ask in GitHub issues for bug-related questions

Thank you for contributing! 🎉
