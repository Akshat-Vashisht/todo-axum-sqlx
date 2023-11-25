use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct TodoModel {
    pub todo_id: Uuid,
    pub todo_title: Option<String>,
    pub todo_description: Option<String>,
    pub created_at: NaiveDate,
}

#[derive(Deserialize, Serialize)]
pub struct CreateTodoModel {
    pub todo_title: Option<String>,
    pub todo_description: Option<String>,
}