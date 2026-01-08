use include_dir::include_dir;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://user_name:passwd@localhost:5432/postgres".to_string());

    let pool = PgPool::connect(&database_url).await?;

    run_migrations(&pool).await?;

    println!("Database in itialized successfully");
    Ok(())
}

async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    let migrations_path = include_dir!("$CARGO_MANIFEST_DIR/src/tables/");

    let mut files = migrations_path.files().collect::<Vec<_>>();

    files.sort_by_key(|f| f.path());

    for file in files {
        let sql = file
            .contents_utf8()
            .expect("Migration file must be valid UTF-8");

        sqlx::query(sql).execute(pool).await?;
    }

    Ok(())
}
