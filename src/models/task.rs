use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable};
use uuid::Uuid;

use proto_rust::provider::{Task, TaskImportance, TaskStatus};

use crate::schema::tasks;

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = tasks)]
pub struct QueryableTask {
    pub id_task: String,
    pub parent_list: String,
    pub title: String,
    pub body: Option<String>,
    pub importance: i32,
    pub favorite: bool,
    pub is_reminder_on: bool,
    pub status: i32,
    pub completed_on: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDateTime>,
    pub reminder_date: Option<NaiveDateTime>,
    pub created_date_time: NaiveDateTime,
    pub last_modified_date_time: NaiveDateTime,
}

impl QueryableTask {
    pub fn new(title: String, parent_list: String) -> Self {
        Self {
            id_task: Uuid::new_v4().to_string(),
            parent_list,
            title,
            body: None,
            completed_on: None,
            due_date: None,
            importance: TaskImportance::Low as i32,
            favorite: false,
            is_reminder_on: false,
            reminder_date: None,
            status: TaskStatus::NotStarted as i32,
            created_date_time: Utc::now().naive_utc(),
            last_modified_date_time: Utc::now().naive_utc(),
        }
    }
}

impl From<QueryableTask> for Task {
    fn from(value: QueryableTask) -> Self {
        Task {
            id: value.id_task,
            parent: value.parent_list,
            title: value.title,
            body: value.body,
            importance: value.importance,
            favorite: value.favorite,
            is_reminder_on: value.is_reminder_on,
            status: value.status,
            completed_on: value.completed_on.map(|d| d.timestamp()),
            due_date: value.due_date.map(|d| d.timestamp()),
            reminder_date: value.reminder_date.map(|d| d.timestamp()),
            created_date_time: value.created_date_time.timestamp(),
            last_modified_date_time: value.last_modified_date_time.timestamp(),
        }
    }
}

impl From<Task> for QueryableTask {
    fn from(task: Task) -> Self {
        Self {
            id_task: task.id,
            parent_list: task.parent,
            title: task.title,
            body: task.body,
            importance: task.importance,
            favorite: task.favorite,
            is_reminder_on: task.is_reminder_on,
            status: task.status,
            completed_on: task
                .completed_on
                .map(|d| NaiveDateTime::from_timestamp_opt(d, 0).unwrap()),
            due_date: task
                .due_date
                .map(|d| NaiveDateTime::from_timestamp_opt(d, 0).unwrap()),
            reminder_date: task
                .reminder_date
                .map(|d| NaiveDateTime::from_timestamp_opt(d, 0).unwrap()),
            created_date_time: NaiveDateTime::from_timestamp_opt(task.created_date_time, 0)
                .unwrap(),
            last_modified_date_time: NaiveDateTime::from_timestamp_opt(
                task.last_modified_date_time,
                0,
            )
            .unwrap(),
        }
    }
}
