use tracing_subscriber::EnvFilter;

use crate::config::AppConfig;
use prelude::*;

mod config;
mod db;
mod domain;
mod graphql;
mod prelude;
mod router;

#[tokio::main]
async fn main() {
    let config = AppConfig::from_env();
    let directives = format!(
        "tower_http={},async_graphql={},{}",
        config.log_level, config.log_level, config.log_level,
    );

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(directives))
        .init();

    let pool = db::postgres::init_postgres(&config.database_url)
        .await
        .unwrap();

    let schema = graphql::build_schema(pool);
    let app: axum::Router = router::build_router(schema);

    let listener = tokio::net::TcpListener::bind(config.listen_url.clone())
        .await
        .unwrap();
    info!("Server running at {}", config.listen_url);

    axum::serve(listener, app).await.unwrap();
}
