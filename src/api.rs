use axum::{extract::{Path, State}, Json, http::StatusCode};
use sqlx::SqlitePool;

use crate::{error::Error, todo::{CreateTodo, Todo, UpdateTodo}};

pub async fn ping(State(dbpool): State<SqlitePool>) -> Result<String, Error> {
    todo!()
}

pub async fn todo_list(State(dbpool): State<SqlitePool>) -> Result<Json<Vec<Todo>>, Error> {
    todo!()
}

pub async fn todo_read(
    State(dbpool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, Error> {
    todo!()
}

pub async fn todo_create(
    State(dbpool): State<SqlitePool>,
    Json(new_todo): Json<CreateTodo>,
) -> Result<Json<Todo>, Error> {
    todo!()
}

pub async fn todo_update(
    State(dbpool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(update_todo): Json<UpdateTodo>,
) -> Result<Json<Todo>, Error> {
    todo!()
}

pub async fn todo_delete(
    State(dbpool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, Error> {
    todo!()
}
