use async_graphql::{Schema, extensions::{ApolloTracing, Tracing}};

use crate::graphql::{mutation::MutationRoot, query::QueryRoot, subscription::SubscriptionRoot};

pub mod query;
pub mod mutation;
pub mod subscription;

pub type AppSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub fn build_schema() -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .extension(Tracing)
        .extension(ApolloTracing)
        .finish()
}