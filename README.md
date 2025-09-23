# 🚀 Tauri Boilerplate

A comprehensive, production-ready boilerplate for building modern desktop applications with **Tauri v2**, **React 18**, **TypeScript**, and **Tailwind CSS**.

## ✨ Features

### 🎨 Frontend

- **React 18** with TypeScript for type-safe development
- **Tailwind CSS v3** for utility-first styling (no component library)
- **React Router v6** for client-side routing
- **React Hook Form + Zod** for robust form handling and validation
- **Zustand** for lightweight state management with persistence
- **Advanced theming** with light/dark/system modes and CSS custom properties

### 🦀 Backend

- **Tauri v2** for secure, fast desktop applications
- **Rust** backend with comprehensive Tauri features:
  - File system operations
  - Window management
  - Native notifications
  - System information access
  - Command execution

> **Filesystem sandbox**: All filesystem commands operate inside the application data directory (`%APPDATA%/tavuc-boilerplate` on Windows, `~/Library/Application Support/com.tavuc.tavuc-boilerplate` on macOS, etc.). Provide paths relative to that root — absolute paths or traversal attempts (for example, using `../`) are rejected.

### 🗄️ Database

- **PostgreSQL** integration with SQLx for type-safe queries
- **Docker Compose** setup for local development
- Database migrations and seeding
- Connection pooling and async operations

### 🧪 Testing

- **Vitest** for unit testing with React Testing Library
- **WebdriverIO + tauri-driver** for desktop end-to-end testing
- Comprehensive test setup and examples

### 🛠️ Development Tools

- **ESLint + Prettier** for code quality and formatting
- **Husky** for Git hooks
- **TypeScript** for type safety across the stack
- **Hot reload** for both frontend and backend development

### 🚀 CI/CD

- **GitHub Actions** workflows for:
  - Continuous Integration (testing, linting, building)
  - Automated releases for multiple platforms
  - Dependency updates
  - Security auditing

## 📋 Prerequisites

- **Node.js** 18+ and npm
- **Rust** 1.70+ and Cargo
- **Docker** and Docker Compose (for database)
- **Git** for version control

### Platform-specific Requirements

#### Windows

- **Microsoft Visual Studio C++ Build Tools**
- **WebView2** (usually pre-installed on Windows 10/11)

#### macOS

- **Xcode Command Line Tools**: `xcode-select --install`

#### Linux (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

## 🚀 Quick Start

### 1. Clone and Install

```bash
git clone <your-repo-url>
cd tauri-boilerplate
npm install
```

### 2. Start Database

```bash
docker-compose up -d
```

### 3. Setup Environment

```bash
cp .env.example .env
# Edit .env with your configuration
```

### 4. Initialize Database

The Docker setup runs the SQL init scripts in `database/init` the first time you launch the services.
If you ever need to rebuild the data from scratch, use:

```bash
npm run db:reset
```

### 5. Start Development

```bash
npm run dev
```

This starts both the Vite dev server and Tauri in development mode.

## 📁 Project Structure

```
├── src/                      # React frontend source
│   ├── api/                  # API layer for Tauri commands
│   ├── components/           # Reusable React components
│   ├── config/               # Configuration files (theme, etc.)
│   ├── hooks/                # Custom React hooks
│   ├── layouts/              # Layout components
│   ├── pages/                # Page components
│   ├── stores/               # Zustand state stores
│   ├── types/                # TypeScript type definitions
│   └── utils/                # Utility functions
├── src-tauri/                # Rust backend source
│   ├── src/
│   │   ├── handlers/         # Tauri command handlers
│   │   ├── database/         # Database operations
│   │   ├── models/           # Data models
│   │   └── lib.rs           # Main application entry
│   ├── migrations/           # SQL database migrations
│   └── Cargo.toml          # Rust dependencies
├── tests/                    # Test files
│   ├── e2e/                  # Playwright E2E tests
│   └── unit/                 # Unit tests
├── database/                 # Database setup
│   └── init/                 # SQL initialization scripts
├── .github/workflows/        # GitHub Actions CI/CD
└── docker-compose.yml        # Docker services
```

## 🔧 Available Scripts

### Development

- `npm run dev` - Start the Vite + Tauri dev workflow
- `npm run tauri:dev` - Run only the Tauri development shell
- `npm run db:up` - Boot PostgreSQL and Redis containers
- `npm run db:down` - Stop the database stack
- `npm run db:logs` - Tail PostgreSQL logs

### Building

- `npm run build` - Build optimized frontend assets
- `npm run preview` - Serve the production build locally
- `npm run tauri:build` - Bundle the desktop application

### Testing

- `npm run test` - Run unit tests
- `npm run test:ui` - Run tests with UI
- `npm run test:desktop` - Run E2E tests
- `npm run test:desktop:ui` - Run E2E tests with UI

### Code Quality

- `npm run lint` - Run ESLint
- `npm run lint:fix` - Fix ESLint issues
- `npm run format` - Format code with Prettier
- `npm run typecheck` - Run TypeScript type checking

### Database

- `npm run db:up` - Start the database services
- `npm run db:down` - Stop and remove containers
- `npm run db:logs` - Follow database logs
- `npm run db:reset` - Reset volumes and start fresh

### Logging

- Logs are automatically managed and rotated
- View logs in the application at `/logs`
- Configure logging via environment variables

## 🎨 Theming

The application supports advanced theming with:

### Theme Modes

- **Light Mode** - Clean, bright interface
- **Dark Mode** - Easy on the eyes
- **System Mode** - Follows OS preference automatically

### CSS Custom Properties

All theme colors are defined as CSS variables, making customization easy:

```typescript
// src/config/theme.ts
export const lightTheme: ThemeColors = {
  primary: {
    /* ... */
  },
  background: {
    /* ... */
  },
  // ...
}
```

### Using Themes in Components

```tsx
import { useTheme } from '../hooks'

const MyComponent = () => {
  const { theme, isDark, toggleTheme } = useTheme()

  return (
    <div className='bg-theme-bg-primary text-theme-text-primary'>
      Current theme: {theme}
    </div>
  )
}
```

## 📝 Logging System

The application includes a comprehensive logging system with both frontend and backend logging capabilities.

### Features

- **Structured Logging**: JSON and plain text formats
- **Multiple Log Levels**: Error, Warn, Info, Debug, Trace
- **Automatic Rotation**: Daily, hourly, or custom rotation
- **File Management**: Automatic cleanup of old log files
- **Context Enrichment**: Add custom context to log entries
- **Performance Timing**: Built-in operation timing
- **Error Boundaries**: Automatic error logging from React
- **Web Interface**: View and search logs in the application

### Configuration

Configure logging via environment variables in your `.env` file:

```env
# Logging Configuration
LOG_LEVEL=info                 # error, warn, info, debug, trace
LOG_CONSOLE_ENABLED=true       # Enable console logging
LOG_CONSOLE_FORMAT=pretty      # pretty, compact, full, json
LOG_CONSOLE_COLORS=true        # Use ANSI colors in console output
LOG_FILE_ENABLED=true          # Enable file logging
LOG_FILE_PREFIX=tavuc-boilerplate # File name prefix for rotated logs
LOG_JSON=false                 # Force JSON output (overrides console format)
LOG_DIRECTORY=logs             # Override log directory (relative or absolute)
LOG_ROTATION=daily             # never, minutely, hourly, daily, weekly
LOG_MAX_FILES=30               # Number of log files to keep
LOG_MAX_SIZE_MB=100            # Optional per-file size limit
```

Leaving `LOG_DIRECTORY` blank (or set to `logs`) keeps output under the platform-specific app data directory. Set `LOG_JSON=true` to emit structured JSON regardless of the console format value. Changes take effect the next time the application starts.

### Usage Examples

#### Frontend (TypeScript)

```typescript
import { useLogger } from '../utils/logger'

const MyComponent = () => {
  const logger = useLogger('MyComponent')

  const handleAction = async () => {
    await logger.info('User clicked button', { userId: 123, action: 'click' })

    try {
      await logger.timeOperation('api-call', async () => {
        return await api.fetchData()
      })
    } catch (error) {
      await logger.error('API call failed', { error: error.message })
    }
  }
}
```

#### Backend (Rust)

```rust
use tracing::{info, error, warn};
use crate::log_with_context;

#[tauri::command]
async fn my_command() -> Result<String, String> {
    info!("Command started");

    log_with_context!(
        LogLevel::Info,
        "Processing user request",
        "user_id" => 123,
        "timestamp" => chrono::Utc::now()
    );

    Ok("Success".to_string())
}
```

### Log Management

Access the logs interface at `/logs` in the application to:

- **View Recent Logs**: Filter by level, time range, and content
- **Search Logs**: Full-text search across log messages
- **Export Logs**: Download logs for external analysis
- **Clear Old Logs**: Remove logs older than specified days
- **View Statistics**: Log file counts, sizes, and dates
- **Test Logging**: Create test log entries at different levels

### Performance Monitoring

The logging system includes built-in performance monitoring:

```typescript
// Automatic timing
await logger.timeOperation('database-query', async () => {
  return await database.query(sql)
})

// Manual timing
const timer = new Timer('custom-operation')
// ... do work ...
timer.finish()
```

### Log Rotation and Cleanup

- **Automatic Rotation**: Logs rotate based on your configuration (daily by default)
- **Size Management**: Optional maximum file size limits
- **Retention Policy**: Automatically remove logs older than specified days
- **Space Monitoring**: Track total log directory size

## 🗄️ Database

### Configuration

Update your `.env` file with database credentials:

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/tauri_app
```

### Migrations

Create new migrations in `src-tauri/migrations/`:

```sql
-- 001_create_users.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
```

### Models

Define Rust models in `src-tauri/src/models/`:

```rust
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
}
```

## 🧪 Testing

### Unit Tests

Create tests alongside your components:

```typescript
// src/components/__tests__/Button.test.tsx
import { render, screen } from '@testing-library/react'
import { Button } from '../Button'

test('renders button with text', () => {
  render(<Button>Click me</Button>)
  expect(screen.getByRole('button')).toHaveTextContent('Click me')
})
```

### E2E Tests

Test complete user workflows:

```typescript
// tests/e2e/app.spec.ts
import { test, expect } from '@playwright/test'

test('user can navigate to settings', async ({ page }) => {
  await page.goto('/')
  await page.click('text=Settings')
  await expect(page).toHaveURL('/settings')
})
```

## 🚀 Deployment

### Building for Production

```bash
npm run build
npm run tauri:build
```

### Release Process

1. Update version in `package.json` and `src-tauri/Cargo.toml`
2. Create and push a git tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
3. GitHub Actions will automatically build and create a release

### Supported Platforms

- **Windows** - `.msi` installer
- **macOS** - `.dmg` installer
- **Linux** - `.AppImage` and `.deb` packages

## 🔒 Security

### Best Practices Implemented

- **CSP (Content Security Policy)** configured
- **API allowlist** - only specific Tauri commands exposed
- **Input validation** with Zod schemas
- **SQL injection protection** with SQLx
- **Dependency auditing** in CI/CD

### Security Auditing

```bash
npm audit
cd src-tauri && cargo audit
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Ensure all tests pass: `npm run test && npm run test:desktop`
5. Commit your changes: `git commit -m 'feat: add amazing feature'`
6. Push to the branch: `git push origin feature/amazing-feature`
7. Open a pull request

### Development Guidelines

- Follow the existing code style
- Write tests for new features
- Update documentation as needed
- Ensure CI/CD passes

## 📚 Documentation

### Key Resources

- [Tauri Documentation](https://tauri.app/)
- [React Documentation](https://react.dev/)
- [Tailwind CSS Documentation](https://tailwindcss.com/)
- [TypeScript Documentation](https://www.typescriptlang.org/)

### API Reference

Detailed API documentation is available in the `/docs` folder (coming soon).

## 🐛 Troubleshooting

### Common Issues

#### Build Failures

- Ensure all system dependencies are installed
- Check that Rust and Node.js versions meet requirements
- Clear caches: `npm run clean && cargo clean`

#### Database Connection Issues

- Verify Docker is running: `docker ps`
- Check database credentials in `.env`
- Restart database: `npm run db:down && npm run db:up`

#### Theme Issues

- Clear browser cache and local storage
- Check if system theme preference is conflicting
- Verify CSS custom properties are loading

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri Team](https://github.com/tauri-apps/tauri) for the amazing framework
- [React Team](https://github.com/facebook/react) for the UI library
- [Tailwind CSS](https://github.com/tailwindlabs/tailwindcss) for the styling system
- All the open-source contributors who made this possible

---

**Happy Coding! 🎉**

If you find this boilerplate helpful, please consider giving it a star ⭐

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
