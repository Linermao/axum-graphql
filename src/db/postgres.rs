use crate::prelude::*;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

/// Initialize PostgreSQL: create connection pool and ensure required tables exist.
/// Returns Some(PgPool) if successful, None if failed (prints error details)
pub async fn init_postgres(database_url: &str) -> Option<PgPool> {
    // create post pool.
    let pool = match PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
    {
        Ok(pool) => {
            info!("PostgreSQL connected successfully");
            pool
        }
        Err(e) => {
            error!("Failed to connect to PostgreSQL");
            print_sqlx_error(&e);
            return None;
        }
    };

    Some(pool)
}

/// Detailed error printer for sqlx errors
fn print_sqlx_error(err: &sqlx::Error) {
    error!("Error: {}", err);

    match err {
        sqlx::Error::Database(db_err) => {
            error!("Database error:");
            error!("  message: {}", db_err.message());
            if let Some(code) = db_err.code() {
                error!("  code: {}", code);
            }
        }
        sqlx::Error::Io(io_err) => error!("I/O error: {}", io_err),
        sqlx::Error::PoolTimedOut => error!("Connection pool timed out"),
        other => error!("Other error: {:?}", other),
    }
}
