-- Create sessions table for JWT refresh token management
-- This table stores user sessions with refresh tokens for authentication

CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    refresh_token TEXT NOT NULL UNIQUE,
    device_info TEXT,
    ip_address TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked BOOLEAN NOT NULL DEFAULT FALSE,

    -- Add foreign key if users table exists
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_refresh_token ON sessions(refresh_token);
CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions(expires_at);
CREATE INDEX IF NOT EXISTS idx_sessions_revoked ON sessions(revoked) WHERE revoked = false;

-- Create index for cleanup queries
CREATE INDEX IF NOT EXISTS idx_sessions_cleanup ON sessions(expires_at, last_used_at) WHERE revoked = true;

-- Add comment for documentation
COMMENT ON TABLE sessions IS 'Stores user authentication sessions with refresh tokens';
COMMENT ON COLUMN sessions.refresh_token IS 'JWT refresh token for obtaining new access tokens';
COMMENT ON COLUMN sessions.device_info IS 'Information about the device/client that created this session';
COMMENT ON COLUMN sessions.revoked IS 'Whether this session has been manually revoked';
