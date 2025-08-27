mod error;
mod todo;

#[tokio::main]
async fn main() {
    init_tracing();

    let db_pool = init_dbpool().await.unwrap();
}

fn init_tracing() {
    use tracing_subscriber::{EnvFilter, filter::LevelFilter, fmt, prelude::*};

    let rust_log = std::env::var(EnvFilter::DEFAULT_ENV)
        .unwrap_or_else(|_| "sqlx=info,tower_http=debug,info".into());

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(rust_log),
        )
        .init();
}

async fn init_dbpool() -> Result<sqlx::Pool<sqlx::Sqlite>, sqlx::Error> {
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    let db_connection_str =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:db.sqlite".to_string());

    let dbpool = SqlitePoolOptions::new()
        .connect_with(SqliteConnectOptions::from_str(&db_connection_str)?.create_if_missing(true))
        .await
        .expect("can't connect to database");

    sqlx::migrate!()
        .run(&dbpool)
        .await
        .expect("database migration failed");

    Ok(dbpool)
}
