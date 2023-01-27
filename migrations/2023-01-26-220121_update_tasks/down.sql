ALTER TABLE tasks RENAME COLUMN parent TO id_list;
ALTER TABLE tasks RENAME COLUMN notes TO body;
ALTER TABLE tasks RENAME COLUMN priority TO importance;
ALTER TABLE tasks RENAME COLUMN completed_date TO completed_on;
ALTER TABLE tasks DROP COLUMN sub_tasks;
ALTER TABLE tasks DROP COLUMN tags;
ALTER TABLE tasks DROP COLUMN deletion_date;
ALTER TABLE tasks ADD COLUMN is_reminder_on;
