# Authentication System Documentation

## Overview

EZ Tauri includes a comprehensive JWT-based authentication system with the following features:

- **User Registration & Login** - Secure account creation and authentication
- **JWT Tokens** - Access and refresh token pair for stateless authentication
- **Session Management** - Track and manage user sessions across devices
- **Token Refresh** - Automatic token renewal without re-authentication
- **Multi-Device Support** - Users can be logged in on multiple devices
- **Session Revocation** - Logout from single device or all devices at once
- **Security** - Bcrypt password hashing, input validation, and SQL injection protection

## Architecture

### Backend (Rust)

The authentication system is organized into modules:

```
src-tauri/src/modules/auth/
├── mod.rs           # Module exports and configuration
├── jwt.rs           # JWT token generation and validation
├── session.rs       # Session management and database operations
├── handlers.rs      # Tauri command handlers
└── models.rs        # Data models
```

### Frontend (TypeScript/React)

```
src/
├── api/auth.ts           # Authentication API functions
└── hooks/useAuth.ts      # React hook for auth state management
```

## Setup

### 1. Environment Variables

Add to your `.env` file:

```bash
# JWT Secret (IMPORTANT: Change in production!)
# Generate with: openssl rand -base64 32
JWT_SECRET=your-secret-key-here-change-in-production-min-32-chars

# Token expiration times
JWT_ACCESS_TOKEN_HOURS=24    # Access token lifetime (24 hours)
JWT_REFRESH_TOKEN_DAYS=30    # Refresh token lifetime (30 days)
```

### 2. Database Migration

The sessions table is automatically created when you run migrations:

```bash
npm run db:up        # Start PostgreSQL
npm run tauri:dev    # Migrations run automatically on startup
```

### 3. Verify Installation

Check that auth commands are registered in `src-tauri/src/lib.rs`:

```rust
modules::auth::auth_register,
modules::auth::auth_login,
modules::auth::auth_logout,
// ... other auth commands
```

## Usage

### Frontend Integration

#### Using the React Hook

The easiest way to use authentication in React components:

```tsx
import { useAuth } from './hooks'

function LoginForm() {
  const { login, register, logout, user, isAuthenticated, isLoading, error } = useAuth()

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault()
    try {
      await login('user@example.com', 'password123')
      // User is now logged in, navigate to dashboard
    } catch (err) {
      // Error is automatically set in the hook
      console.error('Login failed:', err)
    }
  }

  if (isLoading) return <div>Loading...</div>
  if (isAuthenticated) return <div>Welcome, {user?.email}!</div>

  return (
    <form onSubmit={handleLogin}>
      <input type="email" required />
      <input type="password" required />
      <button type="submit">Login</button>
      {error && <div className="error">{error}</div>}
    </form>
  )
}
```

#### Using the Auth Service Directly

For more control, use `AuthService`:

```typescript
import { AuthService } from './api/auth'

// Register a new user
try {
  const user = await AuthService.register(
    'user@example.com',
    'username',
    'password123',
    'John',
    'Doe'
  )
  console.log('Registered:', user)
} catch (error) {
  console.error('Registration failed:', error)
}

// Login
try {
  const user = await AuthService.login('user@example.com', 'password123')
  console.log('Logged in:', user)
} catch (error) {
  console.error('Login failed:', error)
}

// Check if authenticated
if (AuthService.isAuthenticated()) {
  console.log('User is logged in')
}

// Get current user
const user = await AuthService.getCurrentUser()
if (user) {
  console.log('Current user:', user)
}

// Logout
await AuthService.logout()

// Logout from all devices
await AuthService.logoutAll()
```

### Session Management

```typescript
import { useAuth } from './hooks'

function SessionManager() {
  const { getSessions, revokeSession } = useAuth()
  const [sessions, setSessions] = useState([])

  useEffect(() => {
    loadSessions()
  }, [])

  const loadSessions = async () => {
    const userSessions = await getSessions()
    setSessions(userSessions)
  }

  const handleRevoke = async (sessionId: string) => {
    await revokeSession(sessionId)
    await loadSessions() // Reload sessions
  }

  return (
    <div>
      <h2>Active Sessions</h2>
      {sessions.map(session => (
        <div key={session.id}>
          <p>Device: {session.deviceInfo}</p>
          <p>Last used: {new Date(session.lastUsedAt).toLocaleString()}</p>
          <button onClick={() => handleRevoke(session.id)}>Revoke</button>
        </div>
      ))}
    </div>
  )
}
```

### Direct API Calls

If you need to call the Tauri commands directly:

```typescript
import { invoke } from '@tauri-apps/api/core'

// Login
const response = await invoke('auth_login', {
  request: {
    email: 'user@example.com',
    password: 'password123',
    deviceInfo: 'Web Browser'
  }
})

// Refresh token
const refreshed = await invoke('auth_refresh_token', {
  request: {
    refreshToken: 'your_refresh_token_here'
  }
})

// Verify token
const user = await invoke('auth_verify_token', {
  accessToken: 'your_access_token_here'
})
```

## Backend API Reference

### Commands

#### `auth_register`

Register a new user account.

**Request:**
```rust
RegisterRequest {
    email: String,
    username: String,
    password: String,
    first_name: Option<String>,
    last_name: Option<String>,
}
```

**Response:**
```rust
LoginResponse {
    access_token: String,
    refresh_token: String,
    user: PublicUser,
    expires_in: i64,  // seconds
}
```

#### `auth_login`

Authenticate a user.

**Request:**
```rust
LoginRequest {
    email: String,
    password: String,
    device_info: Option<String>,
}
```

**Response:** Same as `auth_register`

#### `auth_logout`

Logout from current session.

**Parameters:**
- `refresh_token: String`

**Response:** `()`

#### `auth_logout_all`

Logout from all devices.

**Parameters:**
- `access_token: String`

**Response:** `()`

#### `auth_refresh_token`

Refresh access token.

**Request:**
```rust
RefreshTokenRequest {
    refresh_token: String,
}
```

**Response:**
```rust
RefreshTokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}
```

#### `auth_verify_token`

Verify access token and get user info.

**Parameters:**
- `access_token: String`

**Response:** `PublicUser`

#### `auth_get_sessions`

Get all active sessions for the user.

**Parameters:**
- `access_token: String`

**Response:** `Vec<SessionInfo>`

#### `auth_revoke_session`

Revoke a specific session.

**Parameters:**
- `access_token: String`
- `session_id: String`

**Response:** `()`

## Security Features

### Password Security

- **Bcrypt Hashing**: Passwords are hashed using bcrypt with default cost factor (10)
- **Minimum Length**: Passwords must be at least 8 characters
- **No Plain Text**: Passwords are never stored in plain text

### Input Validation

All user input is validated on both frontend and backend:

- **Email**: RFC 5322 compliant format
- **Username**: 3-50 alphanumeric characters and underscores
- **Names**: Letters, spaces, apostrophes, and hyphens only
- **XSS Protection**: DOMPurify sanitization + backend validation

### Token Security

- **HS256 Algorithm**: Industry-standard HMAC with SHA-256
- **Short Access Token Lifetime**: Default 24 hours
- **Long Refresh Token Lifetime**: Default 30 days
- **Token Type Validation**: Separate access and refresh token types
- **Session Tracking**: All refresh tokens are tracked in database

### Session Management

- **Database-Backed**: All sessions stored in PostgreSQL
- **Revocation Support**: Sessions can be revoked instantly
- **Expiry Tracking**: Automatic cleanup of expired sessions
- **Device Tracking**: Optional device information for each session

## Error Handling

The auth system uses comprehensive error handling:

```typescript
try {
  await AuthService.login(email, password)
} catch (error) {
  // Error types:
  // - ValidationError: Invalid input
  // - DatabaseError: Database operation failed
  // - Unauthorized: Invalid credentials
  // - Forbidden: Account inactive
  // - TokenExpired: Token has expired
  console.error(error)
}
```

## Best Practices

### 1. Environment Variables

```bash
# ALWAYS change JWT_SECRET in production
# Generate a secure random secret:
openssl rand -base64 32
```

### 2. Token Refresh

Implement automatic token refresh:

```typescript
// Refresh token before it expires
setInterval(async () => {
  if (AuthService.isAuthenticated()) {
    await AuthService.refreshAccessToken()
  }
}, 20 * 60 * 1000) // Every 20 minutes (for 24-hour tokens)
```

### 3. Protected Routes

```tsx
function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const { isAuthenticated, isLoading } = useAuth()
  const navigate = useNavigate()

  useEffect(() => {
    if (!isLoading && !isAuthenticated) {
      navigate('/login')
    }
  }, [isAuthenticated, isLoading])

  if (isLoading) return <div>Loading...</div>
  if (!isAuthenticated) return null

  return <>{children}</>
}
```

### 4. Logout on Tab Close

```typescript
useEffect(() => {
  const handleBeforeUnload = () => {
    // Optionally logout on tab close
    // AuthService.logout()
  }

  window.addEventListener('beforeunload', handleBeforeUnload)
  return () => window.removeEventListener('beforeunload', handleBeforeUnload)
}, [])
```

## Testing

### Unit Tests

The JWT and session modules include comprehensive unit tests:

```bash
cd src-tauri
cargo test --lib auth
```

### Integration Testing

Test the full auth flow:

```typescript
describe('Authentication', () => {
  it('should register and login', async () => {
    const user = await AuthService.register(
      'test@example.com',
      'testuser',
      'password123'
    )
    expect(user).toBeDefined()
    expect(AuthService.isAuthenticated()).toBe(true)

    await AuthService.logout()
    expect(AuthService.isAuthenticated()).toBe(false)
  })
})
```

## Database Schema

```sql
CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    refresh_token TEXT NOT NULL UNIQUE,
    device_info TEXT,
    ip_address TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    last_used_at TIMESTAMPTZ NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
```

## Troubleshooting

### "Invalid JWT secret" Error

Make sure `JWT_SECRET` is set in your `.env` file and is at least 32 characters long.

### "Session not found" Error

This usually means:
1. The refresh token expired
2. The session was revoked
3. The database connection failed

Solution: Re-login to create a new session.

### Token Refresh Fails

- Check that the refresh token is being stored correctly
- Verify the token hasn't expired
- Ensure the session exists in the database

### Database Migration Fails

```bash
# Reset database and rerun migrations
npm run db:reset
npm run tauri:dev
```

## Advanced Topics

### Custom Token Expiry

Modify in `.env`:

```bash
JWT_ACCESS_TOKEN_HOURS=48     # 2 days
JWT_REFRESH_TOKEN_DAYS=90     # 3 months
```

### Adding Email Verification

Extend the `User` model and add email verification logic in handlers.

### OAuth Integration

The current system can be extended to support OAuth providers (Google, GitHub, etc.) by adding provider-specific handlers.

### Role-Based Access Control (RBAC)

Add a `roles` table and modify the JWT claims to include user roles.

## License

This authentication system is part of EZ Tauri and is licensed under the MIT License.
