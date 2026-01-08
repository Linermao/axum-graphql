use async_graphql::{Context, ID, Object};
use uuid::Uuid;

use crate::{
    domain::{
        project::{
            Project, ProjectService,
            tree::{TreeNode, TreeService},
        },
        user,
    },
    graphql::AppState,
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// test query
    async fn hello_user(&self, id: ID, name: String) -> String {
        user::hello(id.as_str(), &name)
    }

    async fn projects(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Project>> {
        let state = ctx.data::<AppState>()?;
        let service = ProjectService { db: &state.db };
        let result = service.get_projects().await?;
        Ok(result)
    }

    async fn project(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
    ) -> async_graphql::Result<Option<Project>> {
        let state = ctx.data::<AppState>()?;
        let service = ProjectService { db: &state.db };
        let result = service.get_project_by_id(project_id).await?;
        Ok(result)
    }

    async fn tree(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
    ) -> async_graphql::Result<Vec<TreeNode>> {
        let state = ctx.data::<AppState>()?;
        let service = TreeService {
            db: &state.db,
            events: &state.events,
        };
        let nodes = service.tree(project_id).await?;
        Ok(nodes)
    }
}
