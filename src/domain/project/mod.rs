use async_graphql::SimpleObject;
use sqlx::prelude::FromRow;
use uuid::Uuid;

/// Domain model for Project
#[derive(SimpleObject, FromRow, Debug, Clone)]
pub struct Project {
    pub project_id: Uuid,
    pub name: String,
}