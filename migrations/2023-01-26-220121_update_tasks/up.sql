ALTER TABLE tasks RENAME COLUMN id_list TO parent;
ALTER TABLE tasks RENAME COLUMN body TO notes;
ALTER TABLE tasks RENAME COLUMN importance TO priority;
ALTER TABLE tasks RENAME COLUMN completed_on TO completion_date;
ALTER TABLE tasks ADD COLUMN sub_tasks TEXT DEFAULT "[]" NOT NULL;
ALTER TABLE tasks ADD COLUMN tags TEXT DEFAULT "[]" NOT NULL;
ALTER TABLE tasks ADD COLUMN today BOOLEAN DEFAULT false NOT NULL;
ALTER TABLE tasks ADD COLUMN deletion_date TIMESTAMP;
ALTER TABLE tasks ADD COLUMN recurrence TEXT;
ALTER TABLE tasks DROP COLUMN is_reminder_on;

DROP TRIGGER IF EXISTS save_task_count_new;
DROP TRIGGER IF EXISTS update_task_count;

CREATE TRIGGER remove_tasks_on_list_delete
    BEFORE DELETE ON lists
BEGIN
    DELETE FROM tasks WHERE tasks.parent = old.id_list;
END;