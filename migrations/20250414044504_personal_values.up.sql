-- Create the users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,                     -- Auto-incrementing integer ID
    email VARCHAR(255) UNIQUE NOT NULL         -- User's email address, must be unique
);

-- Create the personal_values table, linked to users
CREATE TABLE IF NOT EXISTS personal_values (
    id SERIAL PRIMARY KEY,                     -- Auto-incrementing integer ID
    user_id INTEGER NOT NULL,                  -- Foreign key referencing the users table
    name VARCHAR(255) NOT NULL,                -- Name of the value (e.g., Honesty)
    description TEXT,                          -- Optional description of the value
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE, -- Ensure referential integrity and cascade deletes
    UNIQUE (user_id, name)                     -- Ensure value names are unique per user
);
