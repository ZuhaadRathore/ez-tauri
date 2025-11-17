-- Migration to implement hashed refresh tokens
-- This migration clears all existing sessions because we're switching from plaintext to hashed tokens
-- Users will need to log in again after this migration

-- Clear all existing sessions since they contain unhashed tokens
-- that are incompatible with the new hashing implementation
TRUNCATE TABLE sessions;

-- Update the comment to reflect the new hashing implementation
COMMENT ON COLUMN sessions.refresh_token IS 'SHA-256 hash of JWT refresh token (not stored in plaintext for security)';
