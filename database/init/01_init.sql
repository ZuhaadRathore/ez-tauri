-- =====================================================================
-- Initial Database Schema for Tauri Boilerplate Application
-- =====================================================================
--
-- This script creates the foundational database structure including:
-- - User management with authentication
-- - User preferences and settings storage
-- - Application logging infrastructure
-- - Performance indexes for common queries
-- - Automatic timestamp updates via triggers
--
-- Dependencies: PostgreSQL with uuid-ossp extension
-- =====================================================================

-- Enable UUID extension for generating unique identifiers
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =====================================================================
-- USERS TABLE
-- =====================================================================
-- Core user authentication and profile information
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- Unique identifier
    email VARCHAR(255) UNIQUE NOT NULL,               -- Login email (unique)
    username VARCHAR(100) UNIQUE NOT NULL,            -- Display username (unique)
    password_hash VARCHAR(255) NOT NULL,              -- Bcrypt hashed password
    first_name VARCHAR(100),                          -- Optional first name
    last_name VARCHAR(100),                           -- Optional last name
    is_active BOOLEAN DEFAULT true,                   -- Account active status
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,  -- Creation time
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP   -- Last update time
);

-- =====================================================================
-- USER SETTINGS TABLE
-- =====================================================================
-- User-specific preferences and application settings
CREATE TABLE IF NOT EXISTS user_settings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),                     -- Unique identifier
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,       -- Foreign key to users
    theme VARCHAR(20) DEFAULT 'light',                                  -- UI theme preference
    language VARCHAR(10) DEFAULT 'en',                                  -- Language preference
    notifications_enabled BOOLEAN DEFAULT true,                         -- Notification setting
    settings_data JSONB DEFAULT '{}',                                   -- Additional JSON settings
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,      -- Creation time
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,      -- Last update time
    UNIQUE(user_id)                                                     -- One setting per user
);

-- =====================================================================
-- APPLICATION LOGS TABLE
-- =====================================================================
-- Centralized logging for application events and errors
CREATE TABLE IF NOT EXISTS app_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),              -- Unique identifier
    level VARCHAR(20) NOT NULL,                                  -- Log level (info, warn, error, debug)
    message TEXT NOT NULL,                                       -- Log message content
    metadata JSONB DEFAULT '{}',                                 -- Additional structured data
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,       -- Optional user context
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP -- Log timestamp
);

-- =====================================================================
-- PERFORMANCE INDEXES
-- =====================================================================
-- Indexes for common query patterns and foreign key relationships
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);
CREATE INDEX IF NOT EXISTS idx_user_settings_user_id ON user_settings(user_id);
CREATE INDEX IF NOT EXISTS idx_app_logs_level ON app_logs(level);
CREATE INDEX IF NOT EXISTS idx_app_logs_created_at ON app_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_app_logs_user_id ON app_logs(user_id);

-- =====================================================================
-- AUTOMATIC TIMESTAMP UPDATES
-- =====================================================================
-- Trigger function to automatically update 'updated_at' columns
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply triggers to tables with 'updated_at' columns
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_user_settings_updated_at BEFORE UPDATE ON user_settings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();