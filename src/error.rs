use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

#[derive(Debug)]
pub enum Error {
    Sqlx(StatusCode, String),
    NotFound,
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::Sqlx(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::Sqlx(status, message) => (status, message),
            Error::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
