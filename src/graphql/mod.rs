use async_graphql::{EmptyMutation, EmptySubscription, Schema, extensions::Tracing};

use crate::graphql::query::QueryRoot;

pub mod query;
pub mod mutation;
pub mod subscription;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .extension(Tracing)
        .finish()
}