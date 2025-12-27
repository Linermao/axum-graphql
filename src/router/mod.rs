use std::time::Duration;

use axum::{
    routing::get,
    Extension, Router,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use async_graphql::http::{GraphiQLSource};
use axum::response::Html;
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};

use crate::graphql::AppSchema;

pub fn build_router(schema: AppSchema) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)      // only for developing
        .allow_methods(Any)     // GET / POST / OPTIONS
        .allow_headers(Any)
        .max_age(Duration::from_secs(3600));

    Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(Extension(schema))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> Html<String> {
    Html(GraphiQLSource::build()
        .endpoint("/graphql")
        .subscription_endpoint("/ws")
        .finish())
}
