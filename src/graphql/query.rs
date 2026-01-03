use async_graphql::{Context, ID, Object};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::projects::{get_project_by_id, get_projects},
    domain::{project::Project, user},
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello_user(&self, id: ID, name: String) -> String {
        user::hello(id.as_str(), &name)
    }

    async fn projects(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Project>> {
        let pool = ctx.data::<PgPool>()?;
        let result = get_projects(pool).await?;
        Ok(result)
    }

    async fn project(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
    ) -> async_graphql::Result<Option<Project>> {
        let pool = ctx.data::<PgPool>()?;
        let result = get_project_by_id(pool, project_id).await?;
        Ok(result)
    }
}
