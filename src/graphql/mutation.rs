use async_graphql::{Context, Object};
use sqlx::PgPool;

use crate::{db::projects::insert_project, domain::project::Project};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_project(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> async_graphql::Result<Project> {
        let pool = ctx.data::<PgPool>()?;
        let project = insert_project(pool, &name).await?;
        Ok(project)
    }
}
