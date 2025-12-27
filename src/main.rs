use tracing::info;
use tracing_subscriber::EnvFilter;

mod graphql;
mod domain;
mod router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::new("tower_http=debug,async_graphql=debug,debug")
        )
        .init();

    let schema = graphql::build_schema();
    let app: axum::Router = router::build_router(schema);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}