use async_graphql::{Context, Object};
use uuid::Uuid;

use crate::{
    domain::project::{
        Project, ProjectService,
        tree::{TreeNode, TreeService},
    },
    graphql::AppState,
};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_project(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> async_graphql::Result<Project> {
        let state = ctx.data::<AppState>()?;

        let service = ProjectService { db: &state.db };

        let project = service.insert_project(&name).await?;
        Ok(project)
    }

    async fn create_node(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
        label: String,
        parent_id: Option<Uuid>,
    ) -> async_graphql::Result<TreeNode> {
        let state = ctx.data::<AppState>()?;
        let service = TreeService {
            db: &state.db,
            events: &state.events,
        };

        let node = service
            .insert_tree_node(project_id, &label, parent_id)
            .await?;

        Ok(node)
    }
}
