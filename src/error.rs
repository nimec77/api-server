use axum::http::StatusCode;

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
