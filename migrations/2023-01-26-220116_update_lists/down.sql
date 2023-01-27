ALTER TABLE lists ADD COLUMN is_owner;
ALTER TABLE lists ADD COLUMN provider;
ALTER TABLE lists ADD COLUMN count;
ALTER TABLE lists RENAME COLUMN name TO display_name;