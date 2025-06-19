-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    anonymous BOOLEAN NOT NULL,
    username VARCHAR(256) NOT NULL
);

-- Create user_permissions table
CREATE TABLE IF NOT EXISTS user_permissions (
    user_id INTEGER NOT NULL REFERENCES users(id),
    token VARCHAR(256) NOT NULL
);

-- Insert default users
INSERT INTO users (id, anonymous, username) VALUES (1, true, 'Guest')
ON CONFLICT(id) DO UPDATE SET
    anonymous = EXCLUDED.anonymous,
    username = EXCLUDED.username;

INSERT INTO users (id, anonymous, username) VALUES (2, false, 'Test')
ON CONFLICT(id) DO UPDATE SET
    anonymous = EXCLUDED.anonymous,
    username = EXCLUDED.username;

-- Insert default permissions
INSERT INTO user_permissions (user_id, token) VALUES (2, 'Category::View')
ON CONFLICT DO NOTHING;

-- Reset the sequence to start from 3 (after our manual inserts)
SELECT setval('users_id_seq', GREATEST(3, (SELECT MAX(id) + 1 FROM users)));
