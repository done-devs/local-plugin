ALTER TABLE tasks RENAME COLUMN parent_list TO parent;
ALTER TABLE tasks RENAME COLUMN body TO notes;
ALTER TABLE tasks RENAME COLUMN importance TO priority;
ALTER TABLE tasks RENAME COLUMN completed_on TO completed_date;
ALTER TABLE tasks ADD COLUMN sub_tasks TEXT;
ALTER TABLE tasks ADD COLUMN tags TEXT;
ALTER TABLE tasks ADD COLUMN deletion_date TIMESTAMP;
ALTER TABLE tasks DROP COLUMN is_reminder_on;
