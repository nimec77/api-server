pub async fn create_router(dbpool: sqlx::Pool<sqlx::Sqlite>) -> axum::Router {
    use crate::api::{ping, todo_create, todo_delete, todo_list, todo_read, todo_update};
    use axum::{
        Router,
        routing::{delete, get, post, put},
    };
    use tower_http::cors::{Any, CorsLayer};
    use tower_http::trace::TraceLayer;

    Router::new()
        .route("/alive", get(|| async { "OK" }))
        .route("/ready", get(ping))
        .nest(
            "/v1",
            Router::new()
                .route("/todos", get(todo_list))
                .route("/todos", post(todo_create))
                .route("/todos/:id", get(todo_read))
                .route("/todos/:id", put(todo_update))
                .route("/todos/:id", delete(todo_delete)),
        )
        .with_state(dbpool)
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(TraceLayer::new_for_http())
}
