# Template Variables

This document defines all available template variables for the Tauri boilerplate.

## Variable Format

All template variables use the format: `{{VARIABLE_NAME}}`

## Available Variables

### Core Project Variables

- **`{{PROJECT_NAME}}`** - The package/project name (kebab-case)
  - Used in: package.json, Cargo.toml, file names
  - Example: `my-awesome-app`
  - Validation: Must be valid npm package name

- **`{{APP_NAME}}`** - The display name of the application
  - Used in: Window titles, README, descriptions
  - Example: `My Awesome App`
  - Validation: Any string

- **`{{APP_DESCRIPTION}}`** - Short description of the application
  - Used in: package.json, Cargo.toml, README
  - Example: `A modern desktop application built with Tauri`
  - Validation: Any string

### Author Variables

- **`{{AUTHOR_NAME}}`** - The author's name
  - Used in: package.json, Cargo.toml, LICENSE
  - Example: `John Doe`
  - Validation: Any string

- **`{{AUTHOR_EMAIL}}`** - The author's email (optional)
  - Used in: package.json, git config
  - Example: `john@example.com`
  - Validation: Valid email format or empty

### App Identity Variables

- **`{{APP_IDENTIFIER}}`** - Unique app identifier for Tauri
  - Used in: tauri.conf.json
  - Example: `com.johndoe.my-awesome-app`
  - Validation: Reverse domain notation

- **`{{RUST_LIB_NAME}}`** - Rust library name (snake_case)
  - Used in: Cargo.toml lib.name
  - Example: `my_awesome_app_lib`
  - Validation: Valid Rust identifier

### Database Variables

- **`{{DATABASE_NAME}}`** - PostgreSQL database name
  - Used in: docker-compose.yml, .env files
  - Example: `my_awesome_app`
  - Default: `{{PROJECT_NAME}}` with hyphens replaced by underscores

- **`{{DATABASE_USER}}`** - PostgreSQL username
  - Used in: docker-compose.yml, .env files
  - Example: `app_user`
  - Default: `{{PROJECT_NAME}}_user`

- **`{{DATABASE_PASSWORD}}`** - PostgreSQL password
  - Used in: docker-compose.yml, .env files
  - Example: `secure_password_123`
  - Default: Randomly generated

### Logging Variables

- **`{{LOG_FILE_PREFIX}}`** - Prefix for log files
  - Used in: .env files, logging configuration
  - Example: `my-awesome-app`
  - Default: `{{PROJECT_NAME}}`

## Files That Use Template Variables

### Core Configuration

- `package.json` - PROJECT_NAME, APP_DESCRIPTION, AUTHOR_NAME, AUTHOR_EMAIL
- `src-tauri/Cargo.toml` - PROJECT_NAME, APP_DESCRIPTION, AUTHOR_NAME, RUST_LIB_NAME
- `src-tauri/tauri.conf.json` - APP_NAME, APP_IDENTIFIER

### Environment & Docker

- `.env.example` - LOG_FILE_PREFIX, DATABASE_NAME, DATABASE_USER, DATABASE_PASSWORD
- `docker-compose.yml` - DATABASE_NAME, DATABASE_USER, DATABASE_PASSWORD

### Documentation

- `README.md` - PROJECT_NAME, APP_NAME, APP_DESCRIPTION
- `.github/release-notes-template.md` - PROJECT_NAME

### Source Code

- `src/layouts/Layout.tsx` - APP_NAME (in title)
- `src-tauri/src/logging/config.rs` - LOG_FILE_PREFIX
- `src-tauri/src/logging/mod.rs` - LOG_FILE_PREFIX

## Variable Dependencies

Some variables are derived from others if not explicitly provided:

```
APP_IDENTIFIER = com.${AUTHOR_SLUG}.${PROJECT_NAME}
RUST_LIB_NAME = ${PROJECT_NAME}_lib (with hyphens → underscores)
DATABASE_NAME = ${PROJECT_NAME} (with hyphens → underscores)
DATABASE_USER = ${PROJECT_NAME}_user
LOG_FILE_PREFIX = ${PROJECT_NAME}
```

Where `AUTHOR_SLUG` is the author name converted to lowercase with spaces/special chars removed.

## Validation Rules

- **PROJECT_NAME**: Must match npm package name rules (lowercase, hyphens allowed, no spaces)
- **APP_IDENTIFIER**: Must be valid reverse domain notation (e.g., com.company.app)
- **AUTHOR_EMAIL**: Must be valid email format or empty string
- **DATABASE_NAME**: Must be valid PostgreSQL database name (alphanumeric + underscores)
- **RUST_LIB_NAME**: Must be valid Rust identifier (snake_case)

## How to Use the Template System

### Interactive Mode

Run the template processor interactively to be prompted for all values:

```bash
npm run template:process
```

### Non-Interactive Mode

Pass values as command-line arguments:

```bash
npm run template:process -- \
  --project-name my-awesome-app \
  --app-name "My Awesome App" \
  --author-name "John Doe" \
  --author-email "john@example.com"
```

### What the Script Does

1. **Validates Project**: Ensures you're in a valid EZ Tauri directory
2. **Collects Variables**: Prompts for required values or uses provided arguments
3. **Derives Values**: Automatically generates dependent variables (like database names)
4. **Processes Files**: Replaces all `{{VARIABLE_NAME}}` placeholders in the specified files
5. **Confirms Changes**: Shows a summary before processing

### Processed Files

The script automatically processes these files:

- `package.json` - Project metadata and scripts
- `src-tauri/Cargo.toml` - Rust package configuration
- `src-tauri/tauri.conf.json` - Tauri app configuration
- `.env.example` - Environment variables template
- `docker-compose.yml` - Database service configuration
- `src/layouts/Layout.tsx` - React app title
- `src-tauri/src/logging/mod.rs` - Logging configuration
- `src-tauri/src/logging/config.rs` - Logging defaults
- `README.md` - Project documentation

### Example Usage

```bash
$ npm run template:process

🚀 EZ Tauri Template Processor
======================================

Project name (kebab-case, e.g., my-awesome-app): todo-app
App display name (e.g., My Awesome App): Todo App
App description: A simple todo application built with Tauri
Author name: Jane Smith
Author email (optional): jane@example.com
App identifier (reverse domain notation, e.g., com.company.app): com.janesmith.todoapp

📋 Configuration Summary:
========================
PROJECT_NAME: todo-app
APP_NAME: Todo App
APP_DESCRIPTION: A simple todo application built with Tauri
AUTHOR_NAME: Jane Smith
AUTHOR_EMAIL: jane@example.com
APP_IDENTIFIER: com.janesmith.todoapp
RUST_LIB_NAME: todo_app_lib
DATABASE_NAME: todo_app
DATABASE_USER: todo_app_user
DATABASE_PASSWORD: ****************
LOG_FILE_PREFIX: todo-app

🚀 Proceed with template processing? (y/N): y

🔄 Processing template files...
✅ Processed: package.json
✅ Processed: src-tauri/Cargo.toml
... etc
```
