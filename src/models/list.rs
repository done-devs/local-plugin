use diesel::{Insertable, Queryable};
use proto_rust::provider::List;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::lists;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = lists)]
pub struct QueryableList {
    pub id_list: String,
    pub name: String,
    pub is_owner: bool,
    pub icon_name: Option<String>,
    pub provider: String,
}

impl QueryableList {
    pub fn new(display_name: &str, icon_name: Option<String>, list_provider: String) -> Self {
        Self {
            id_list: Uuid::new_v4().to_string(),
            name: display_name.to_string(),
            is_owner: true,
            icon_name,
            provider: list_provider,
        }
    }
}

impl From<QueryableList> for List {
    fn from(value: QueryableList) -> Self {
        List {
            id: value.id_list,
            name: value.name,
            is_owner: value.is_owner,
            icon: value.icon_name,
            provider: value.provider,
        }
    }
}

impl From<List> for QueryableList {
    fn from(task: List) -> Self {
        Self {
            id_list: task.id,
            name: task.name,
            is_owner: task.is_owner,
            icon_name: task.icon,
            provider: task.provider,
        }
    }
}
