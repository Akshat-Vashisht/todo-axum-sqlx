use crate::api::todos::todo_models::{CreateTodoModel, TodoModel};
use crate::AppState;
use axum::extract::Path;
use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

pub async fn get_todos(State(state): State<AppState>) -> impl IntoResponse {
    let conn = state.db_pool;

    let query_result = sqlx::query_as!(TodoModel, "SELECT * FROM todos",)
        .fetch_all(&conn)
        .await;

    if query_result.is_err() {
        return Json(json!({
            "Status": "Failed",
            "Description": "Failed to get values from the database"
        }));
    }

    let categories = query_result.unwrap();
    if categories.is_empty() {
        return Json(json!({
            "Status": "Failed",
            "Description": "No data found in database",
        }));
    }

    Json(json!(categories))
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(body): Json<CreateTodoModel>,
) -> impl IntoResponse {
    let conn = state.db_pool;

    let query_result = sqlx::query_as!(
        TodoModel,
        "INSERT INTO todos (todo_title, todo_description) VALUES ($1, $2) RETURNING *",
        body.todo_title,
        body.todo_description
    )
    .fetch_one(&conn)
    .await;

    match query_result {
        Ok(category) => Json(json!({"Status":"Success","Data":category})),

        Err(e) => Json(json!({"Status": "Failed","Description": format!("{:?}", e)})),
    }
}

pub async fn delete_todo(
    Path(title): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let conn = state.db_pool;

    let query_result = sqlx::query!("DELETE FROM todos WHERE todo_title = $1", title)
        .execute(&conn)
        .await
        .unwrap()
        .rows_affected();

    if query_result == 0 {
        return Json(json!({
            "Status": "Failed",
            "Description": format!("No todo with title: {title}")
        }));
    }

    Json(json!({
        "Status": "Success",
        "Description": format!("Successfully deleted todo with title: {title}")
    }))
}

pub async fn update_todo(
    Path(title): Path<String>,
    State(state): State<AppState>,
    Json(body): Json<CreateTodoModel>,
) -> impl IntoResponse {
    let conn = state.db_pool;

    let query_result = sqlx::query_as!(
        TodoModel,
        "SELECT * FROM todos WHERE todo_title = $1",
        title
    )
    .fetch_one(&conn)
    .await;

    if query_result.is_err() {
        return Json(json!({
            "Status": "Failed",
            "Description": format!("No todo with title: {title}")
        }));
    }

    let query_result = sqlx::query_as!(
        TodoModel,
        "UPDATE todos SET todo_description = $1 WHERE todo_title = $2 RETURNING *",
        body.todo_description,
        title
    )
    .fetch_one(&conn)
    .await;

    match query_result {
        Ok(todo) => Json(json!({"Status":"Success",
                            "Updated Data":todo})),
        Err(e) => Json(json!({
            "Status": "Failed",
            "Description": format!("Failed to update: {e}")
        })),
    }
}

pub async fn get_todo_by_id(
    Path(title): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let conn = state.db_pool;

    let query_result = sqlx::query_as!(
        TodoModel,
        "SELECT * FROM todos WHERE todo_title = $1",
        title
    )
    .fetch_one(&conn)
    .await;

    match query_result {
        Ok(category) => Json(json!(category)),
        Err(_) => Json(json!({
            "Status": "Failed",
            "Description": format!("No todo with title: {title}")
        })),
    }
}
