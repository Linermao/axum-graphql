use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::prelude::*;
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

    // initial table if empty
    if let Err(e) = init_table(&pool).await {
        error!("Failed to initialize database tables: {:#?}", e);
        return None;
    }

    Some(pool)
}

/// Initialize required tables if they do not exist
async fn init_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS projects (
        project_id UUID PRIMARY KEY,
        name TEXT NOT NULL
    );
    "#;

    sqlx::query(query)
        .execute(pool)
        .await?;

    Ok(())
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