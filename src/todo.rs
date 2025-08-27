use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, query, query_as};

use crate::error::Error;

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct Todo {
    id: i64,
    body: String,
    completed: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    body: String,
}

impl CreateTodo {
    pub fn body(&self) -> &str {
        self.body.as_ref()
    }
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    body: String,
    completed: bool,
}

impl UpdateTodo {
    pub fn body(&self) -> &str {
        self.body.as_ref()
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}
impl Todo {
    pub async fn list(dbpool: SqlitePool) -> Result<Vec<Todo>, Error> {
        query_as(
            r#"
            SELECT * FROM todos
            "#,
        )
        .fetch_all(&dbpool)
        .await
        .map_err(Into::into)
    }

    pub async fn read(dbpool: SqlitePool, id: i64) -> Result<Todo, Error> {
        query_as(
            r#"
            SELECT * FROM todos WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&dbpool)
        .await
        .map_err(Into::into)
    }

    pub async fn create(dbpool: SqlitePool, new_todo: CreateTodo) -> Result<Todo, Error> {
        query_as(
            r#"
            INSERT INTO todos (body) VALUES (?)
            RETURNING *
            "#,
        )
        .bind(new_todo.body())
        .fetch_one(&dbpool)
        .await
        .map_err(Into::into)
    }

    pub async fn update(
        dbpool: SqlitePool,
        id: i64,
        update_todo: UpdateTodo,
    ) -> Result<Todo, Error> {
        query_as(
            r#"
            UPDATE todos SET body = ?, completed = ?, updated_at = datetime('now') WHERE id = ?
            RETURNING *
            "#,
        )
        .bind(update_todo.body())
        .bind(update_todo.completed())
        .bind(id)
        .fetch_one(&dbpool)
        .await
        .map_err(Into::into)
    }

    pub async fn delete(dbpool: SqlitePool, id: i64) -> Result<(), Error> {
        query(
            r#"
            DELETE FROM todos WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&dbpool)
        .await?;

        Ok(())
    }
}
