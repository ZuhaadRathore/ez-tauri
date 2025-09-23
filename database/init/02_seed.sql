-- Seed data for development

-- Insert sample users (passwords are 'password123' hashed with bcrypt)
INSERT INTO users (email, username, password_hash, first_name, last_name) VALUES
    ('admin@example.com', 'admin', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/Go2b/jRWu', 'Admin', 'User'),
    ('user@example.com', 'user', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/Go2b/jRWu', 'Test', 'User'),
    ('demo@example.com', 'demo', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/Go2b/jRWu', 'Demo', 'User')
ON CONFLICT (email) DO NOTHING;

-- Insert default user settings
INSERT INTO user_settings (user_id, theme, language, settings_data)
SELECT
    u.id,
    'light',
    'en',
    '{"sidebar_collapsed": false, "auto_save": true}'::jsonb
FROM users u
ON CONFLICT (user_id) DO NOTHING;

-- Insert some sample logs
INSERT INTO app_logs (level, message, metadata, user_id)
SELECT
    'info',
    'User logged in',
    '{"ip": "127.0.0.1", "user_agent": "Tauri App"}'::jsonb,
    u.id
FROM users u
WHERE u.username = 'admin';

INSERT INTO app_logs (level, message, metadata)
VALUES
    ('info', 'Application started', '{"version": "0.1.0"}'::jsonb),
    ('debug', 'Database connection established', '{"database": "tauri_app"}'::jsonb);