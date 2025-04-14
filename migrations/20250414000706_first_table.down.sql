-- Add down migration script here
CREATE TABLE IF NOT EXISTS first_table (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);