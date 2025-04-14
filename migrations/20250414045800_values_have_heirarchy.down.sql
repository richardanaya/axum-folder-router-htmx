-- Remove the foreign key constraint first
ALTER TABLE personal_values
DROP CONSTRAINT IF EXISTS fk_parent_value;

-- Remove the parent_id column
ALTER TABLE personal_values
DROP COLUMN IF EXISTS parent_id;
