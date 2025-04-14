-- Create the personal_values table
CREATE TABLE IF NOT EXISTS personal_values (
    id SERIAL PRIMARY KEY,                     -- Auto-incrementing integer ID
    name VARCHAR(255) UNIQUE NOT NULL,         -- Name of the value (e.g., Honesty), must be unique
    description TEXT                           -- Optional description of the value
);
