-- =====================================================================
-- Development Seed Data for Tauri Boilerplate Application
-- =====================================================================
--
-- This script populates the database with sample data for development:
-- - Test user accounts with known passwords
-- - Default user settings and preferences
-- - Sample application logs for testing
--
-- SECURITY WARNING: This seed data is for development only!
-- Do NOT use in production environments.
--
-- Default password for all test users: 'password123'
-- =====================================================================

-- =====================================================================
-- SAMPLE USERS
-- =====================================================================
-- Test user accounts with bcrypt-hashed passwords
INSERT INTO users (email, username, password_hash, first_name, last_name) VALUES
    ('admin@example.com', 'admin', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/Go2b/jRWu', 'Admin', 'User'),   -- Administrator account
    ('user@example.com', 'user', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/Go2b/jRWu', 'Test', 'User'),    -- Standard test user
    ('demo@example.com', 'demo', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/Go2b/jRWu', 'Demo', 'User')     -- Demo user for presentations
ON CONFLICT (email) DO NOTHING;  -- Skip if users already exist

-- =====================================================================
-- DEFAULT USER SETTINGS
-- =====================================================================
-- Create default preferences for all users
INSERT INTO user_settings (user_id, theme, language, settings_data)
SELECT
    u.id,                                                              -- Link to user
    'light',                                                           -- Default to light theme
    'en',                                                              -- English language
    '{"sidebar_collapsed": false, "auto_save": true}'::jsonb            -- Default app preferences
FROM users u
ON CONFLICT (user_id) DO NOTHING;  -- Skip if settings already exist

-- =====================================================================
-- SAMPLE APPLICATION LOGS
-- =====================================================================
-- Test log entries for development and testing
-- User-specific log entry
INSERT INTO app_logs (level, message, metadata, user_id)
SELECT
    'info',                                                            -- Info level log
    'User logged in',                                                  -- Login event
    '{"ip": "127.0.0.1", "user_agent": "Tauri App"}'::jsonb,             -- Context metadata
    u.id                                                               -- Admin user ID
FROM users u
WHERE u.username = 'admin';

-- System-level log entries (no specific user)
INSERT INTO app_logs (level, message, metadata)
VALUES
    ('info', 'Application started', '{"version": "0.1.0"}'::jsonb),                        -- App startup
    ('debug', 'Database connection established', '{"database": "tauri_app"}'::jsonb);       -- DB connection