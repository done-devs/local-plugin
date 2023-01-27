use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{Insertable, Queryable};
use uuid::Uuid;

use proto_rust::provider::{Priority, Status, Task};

use crate::schema::tasks;

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = tasks)]
pub struct QueryableTask {
    pub id_task: String,
    pub parent: String,
    pub title: String,
    pub favorite: bool,
    pub today: bool,
    pub status: i32,
    pub priority: i32,
    pub sub_tasks: String,
    pub tags: String,
    pub notes: Option<String>,
    pub completion_date: Option<NaiveDateTime>,
    pub deletion_date: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDateTime>,
    pub reminder_date: Option<NaiveDateTime>,
    pub recurrence: Option<String>,
    pub created_date_time: NaiveDateTime,
    pub last_modified_date_time: NaiveDateTime,
}

impl QueryableTask {
    pub fn new(title: String, parent: String) -> Self {
        let empty_vec: Vec<String> = vec![];
        Self {
            id_task: Uuid::new_v4().to_string(),
            parent,
            title,
            favorite: false,
            today: false,
            notes: None,
            status: Status::NotStarted as i32,
            priority: Priority::Low as i32,
            sub_tasks: serde_json::to_string(&empty_vec).unwrap(),
            tags: serde_json::to_string(&empty_vec).unwrap(),
            completion_date: None,
            deletion_date: None,
            due_date: None,
            reminder_date: None,
            recurrence: None,
            created_date_time: Utc::now().naive_utc(),
            last_modified_date_time: Utc::now().naive_utc(),
        }
    }
}

impl From<QueryableTask> for Task {
    fn from(value: QueryableTask) -> Self {
        Task {
            id: value.id_task,
            parent: value.parent,
            title: value.title,
            favorite: value.favorite,
            today: value.today,
            notes: value.notes,
            status: value.status,
            priority: value.priority,
            sub_tasks: serde_json::from_str(&value.sub_tasks).unwrap(),
            tags: serde_json::from_str(&value.tags).unwrap(),
            completion_date: value.completion_date.map(|date| date.into()),
            deletion_date: value.deletion_date.map(|date| date.into()),
            due_date: value.due_date.map(|date| date.into()),
            reminder_date: value.reminder_date.map(|date| date.into()),
            recurrence: value.recurrence,
            created_date_time: Some(value.created_date_time.into()),
            last_modified_date_time: Some(value.last_modified_date_time.into()),
        }
    }
}

impl From<Task> for QueryableTask {
    fn from(value: Task) -> Self {
        Self {
            id_task: value.id,
            parent: value.parent,
            title: value.title,
            favorite: value.favorite,
            today: value.today,
            notes: value.notes,
            status: value.status,
            priority: value.priority,
            sub_tasks: serde_json::to_string(&value.sub_tasks).unwrap(),
            tags: serde_json::to_string(&value.tags).unwrap(),
            completion_date: value
                .completion_date
                .map(|date| DateTime::from(date).naive_utc()),
            deletion_date: value
                .deletion_date
                .map(|date| DateTime::from(date).naive_utc()),
            due_date: value.due_date.map(|date| DateTime::from(date).naive_utc()),
            reminder_date: value
                .reminder_date
                .map(|date| DateTime::from(date).naive_utc()),
            recurrence: value.recurrence,
            created_date_time: value
                .created_date_time
                .map(|date| DateTime::from(date).naive_utc())
                .unwrap(),
            last_modified_date_time: value
                .last_modified_date_time
                .map(|date| DateTime::from(date).naive_utc())
                .unwrap(),
        }
    }
}
