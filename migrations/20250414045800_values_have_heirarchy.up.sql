-- Add the parent_id column to personal_values
ALTER TABLE personal_values
ADD COLUMN parent_id INTEGER;

-- Add the foreign key constraint for the self-referencing relationship
ALTER TABLE personal_values
ADD CONSTRAINT fk_parent_value
FOREIGN KEY (parent_id)
REFERENCES personal_values(id)
ON DELETE SET NULL; -- Or ON DELETE CASCADE, depending on desired behavior when a parent is deleted
