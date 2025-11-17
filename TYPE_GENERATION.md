# Type Generation with Specta/Tauri-Specta

This project uses `specta` and `tauri-specta` to automatically generate TypeScript types from Rust types, ensuring type safety between the Rust backend and TypeScript frontend.

## Overview

The type generation system:
- Automatically generates TypeScript types from Rust structs and enums
- Ensures type safety between frontend and backend
- Generates TypeScript bindings for all Tauri commands
- Keeps types in sync with Rust code

## Generated Types Location

TypeScript types are generated in: `src/types/bindings.ts`

## How to Generate Types

### Option 1: Using npm script (Recommended)

```bash
npm run generate:types
```

### Option 2: Manual generation

```bash
cd src-tauri
cargo run --bin export_types
```

## Integration in Development Workflow

### During Development

1. Make changes to your Rust types or Tauri commands
2. Run `npm run generate:types` to regenerate TypeScript types
3. Use the generated types in your TypeScript code

### Example Usage

```typescript
import { LoginRequest, LoginResponse, PublicUser } from '@/types/bindings';

// Type-safe Tauri command invocation
const loginUser = async (email: string, password: string): Promise<LoginResponse> => {
  const request: LoginRequest = { email, password, deviceInfo: null };
  return await invoke('auth_login', { request });
};
```

## What Types Are Generated?

All types used in Tauri commands are automatically generated, including:

- **Authentication Types**: LoginRequest, LoginResponse, RegisterRequest, RefreshTokenRequest, SessionInfo
- **User Types**: PublicUser, CreateUser, UpdateUser
- **System Types**: SystemInfo, WindowInfo, DatabaseStatus
- **Logging Types**: LogEntry, LogConfig, LogQueryParams, LogResponse
- **Filesystem Types**: FileInfo, DirectoryListing
- **And more...**

## Adding New Types

To add new types to the generation:

1. Define your Rust struct with `#[derive(Type)]`:
   ```rust
   use specta::Type;
   use serde::{Deserialize, Serialize};

   #[derive(Debug, Serialize, Deserialize, Type)]
   #[serde(rename_all = "camelCase")]
   #[specta(rename_all = "camelCase")]
   pub struct MyNewType {
       pub id: String,
       pub value: i32,
   }
   ```

2. Use it in a Tauri command:
   ```rust
   #[tauri::command]
   async fn my_command(data: MyNewType) -> Result<MyNewType, String> {
       // implementation
   }
   ```

3. Add the command to `src-tauri/src/bin/export_types.rs` in the `collect_commands![]` macro

4. Regenerate types: `npm run generate:types`

## Troubleshooting

### Types not updating

1. Make sure you've saved your Rust files
2. Run `npm run generate:types` again
3. Restart your TypeScript language server in your IDE

### Build errors during type generation

If you encounter build errors, make sure:
- All Rust dependencies are up to date: `cargo update`
- You're running the command from the project root
- The src-tauri project builds successfully: `cargo build`

## Benefits

- **Type Safety**: Catch type mismatches at compile time
- **Auto-completion**: Full IDE support for all types
- **Refactoring**: Rename types in Rust and TypeScript automatically knows
- **Documentation**: Types serve as documentation for the API
- **Reduced Bugs**: Eliminate runtime type errors
