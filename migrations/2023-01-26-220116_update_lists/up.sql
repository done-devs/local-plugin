ALTER TABLE lists DROP COLUMN is_owner;
ALTER TABLE lists DROP COLUMN provider;
ALTER TABLE lists DROP COLUMN count;
ALTER TABLE lists RENAME COLUMN display_name TO name;