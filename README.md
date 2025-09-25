# EZ Tauri

A production-ready Tauri + React boilerplate for building cross-platform desktop applications with web technologies.

## Overview

EZ Tauri provides a streamlined foundation for desktop application development, combining React's familiar web development experience with Rust's performance and Tauri's lightweight runtime. This boilerplate eliminates common setup friction while maintaining flexibility for complex applications.

### Key Features

- **Minimal footprint**: ~10MB distributable size compared to 100MB+ for Electron alternatives
- **Native performance**: Rust backend with direct OS integration
- **Type-safe architecture**: TypeScript frontend with Rust backend communication
- **Database-ready**: PostgreSQL integration with secure connection management
- **Production-tested**: Comprehensive testing setup with CI/CD pipelines

### Technology Stack

| Layer        | Technologies                                              |
| ------------ | --------------------------------------------------------- |
| **Frontend** | React 18, TypeScript, Tailwind CSS, Zustand, React Router |
| **Runtime**  | Tauri v2, Rust                                            |
| **Database** | PostgreSQL, SQLx, Docker                                  |
| **Testing**  | Vitest, WebdriverIO                                       |
| **DevOps**   | GitHub Actions, ESLint, Prettier                          |

## Prerequisites

### Required Software

- **Node.js** 18 or higher ([Download](https://nodejs.org/) or use [fnm](https://github.com/Schniz/fnm))
- **Rust** 1.70 or higher ([Install via rustup](https://rustup.rs/))
- **Docker** ([Download](https://www.docker.com/get-started))

### Platform-Specific Requirements

#### Windows

Install Visual Studio Build Tools with C++ support. WebView2 runtime is typically pre-installed on Windows 10/11.

#### macOS

```bash
xcode-select --install
```

#### Linux (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
                 libwebkit2gtk-4.1-dev \
                 libappindicator3-dev \
                 librsvg2-dev \
                 patchelf
```

## Quick Start

### Installation

Using the template generator:

```bash
npm create ez-tauri my-app
cd my-app
npm install
```

Or clone directly:

```bash
git clone https://github.com/ZuhaadRathore/ez-tauri.git
cd ez-tauri
npm install
```

### Initial Setup

1. **Start the database**

   ```bash
   docker-compose up -d
   ```

2. **Configure environment variables**

   ```bash
   cp .env.example .env
   # Update .env with your configuration
   ```

3. **Launch development server**
   ```bash
   npm run dev
   ```

The application will open automatically with hot reload enabled for both frontend and backend changes.

## Project Structure

```
ez-tauri/
├── src/                    # React frontend application
│   ├── components/         # React components
│   ├── pages/             # Route pages
│   ├── stores/            # Zustand state management
│   └── utils/             # Utility functions
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── handlers/      # Tauri command handlers
│   │   ├── db/           # Database models and queries
│   │   └── main.rs       # Application entry point
│   └── migrations/        # SQL migration files
├── tests/                 # Test suites
│   ├── unit/             # Unit tests
│   └── e2e/              # End-to-end tests
├── database/              # Database initialization
└── .github/workflows/     # CI/CD configuration
```

## Development

### Available Commands

| Command               | Description                              |
| --------------------- | ---------------------------------------- |
| `npm run dev`         | Start development server with hot reload |
| `npm run build`       | Build production frontend                |
| `npm run tauri:build` | Package native desktop application       |
| `npm run test`        | Execute test suite                       |
| `npm run lint:fix`    | Auto-fix linting issues                  |
| `npm run db:up`       | Start PostgreSQL container               |
| `npm run db:reset`    | Reset database to initial state          |

### Frontend-Backend Communication

The architecture uses Tauri's IPC (Inter-Process Communication) for type-safe communication between React and Rust.

#### Backend Handler (Rust)

```rust
// src-tauri/src/handlers/files.rs
#[tauri::command]
pub async fn read_config_file() -> Result<String, String> {
    let contents = std::fs::read_to_string("config.json")
        .map_err(|e| e.to_string())?;
    Ok(contents)
}
```

#### Frontend Usage (React)

```tsx
// src/components/Settings.tsx
import { invoke } from '@tauri-apps/api/core'

const Settings: React.FC = () => {
  const [config, setConfig] = useState<string>('')

  useEffect(() => {
    invoke<string>('read_config_file').then(setConfig).catch(console.error)
  }, [])

  return <pre>{config}</pre>
}
```

## Database Integration

The boilerplate includes PostgreSQL with automatic migration management and secure credential storage.

### Configuration

Database connection is managed through environment variables with automatic encryption via Tauri's Stronghold after first use.

### Migrations

SQL migrations are automatically applied from `src-tauri/migrations/`:

```sql
-- 001_create_users.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

## Building for Production

### Generate Distributables

```bash
npm run build
npm run tauri:build
```

This produces platform-specific installers:

- **Windows**: `.msi` installer
- **macOS**: `.dmg` disk image
- **Linux**: `.deb` package and `.AppImage`

### Security Considerations

- Content Security Policy (CSP) configured
- Command allowlisting enforced
- Input validation using Zod schemas
- SQL injection protection via parameterized queries
- Credentials encrypted with Stronghold

Regular security audits:

```bash
npm audit
cd src-tauri && cargo audit
```

## Included Features

The boilerplate demonstrates essential desktop application patterns:

- **UI/UX**: Theme switching with system preference detection
- **File System**: Secure file operations in app data directory
- **Database**: User management with PostgreSQL
- **Window Management**: Native window controls
- **Notifications**: OS-native notification system
- **System Integration**: Hardware and OS information access
- **Logging**: Rotating file logs with configurable levels
- **Error Handling**: Comprehensive error boundaries and recovery

## Performance Comparison

| Metric             | Tauri             | Electron            |
| ------------------ | ----------------- | ------------------- |
| **Bundle Size**    | ~10MB             | ~100MB+             |
| **Memory Usage**   | ~50MB             | ~500MB+             |
| **Startup Time**   | <1s               | 2-3s                |
| **Security Model** | Isolated contexts | Full Node.js access |

## Troubleshooting

### Common Issues

**Build failures**

- Verify Node.js and Rust versions meet requirements
- Ensure platform-specific dependencies are installed
- Clear build caches: `rm -rf node_modules target/ && npm install`

**Database connection errors**

- Confirm Docker daemon is running: `docker ps`
- Reset database state: `npm run db:reset`
- Check `.env` configuration

**Test failures**

- Ensure application is not running during test execution
- WebdriverIO tests require exclusive access to the application

## Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please ensure all tests pass and add appropriate test coverage for new features.

## Support

- **Documentation**: [Full documentation](https://github.com/ZuhaadRathore/ez-tauri/wiki)
- **Issues**: [GitHub Issues](https://github.com/ZuhaadRathore/ez-tauri/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ZuhaadRathore/ez-tauri/discussions)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Built with a focus on developer experience and production readiness.
