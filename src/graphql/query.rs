use async_graphql::{Context, ID, Object};
use uuid::Uuid;

use crate::{
    domain::{
        project::{
            Project, ProjectService,
            rf_canva::{RfEdge, RfNode, RfService},
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

    async fn get_projects(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Project>> {
        let state = ctx.data::<AppState>()?;
        let service = ProjectService { db: &state.db };
        let result = service.get_projects().await?;
        Ok(result)
    }

    async fn get_project_by_id(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
    ) -> async_graphql::Result<Option<Project>> {
        let state = ctx.data::<AppState>()?;
        let service = ProjectService { db: &state.db };
        let result = service.get_project_by_id(project_id).await?;
        Ok(result)
    }

    async fn get_tree_nodes(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
    ) -> async_graphql::Result<Vec<TreeNode>> {
        let state = ctx.data::<AppState>()?;
        let service = TreeService {
            db: &state.db,
            events: &state.events,
        };
        let nodes = service.get_tree_nodes(project_id).await?;
        Ok(nodes)
    }

    async fn get_rf_nodes(
        &self,
        ctx: &Context<'_>,
        canva_id: Uuid,
    ) -> async_graphql::Result<Vec<RfNode>> {
        let state = ctx.data::<AppState>()?;
        let service = RfService {
            db: &state.db,
            events: &state.events,
        };
        let nodes = service.get_rf_nodes(canva_id).await?;
        Ok(nodes)
    }

    async fn get_rf_edges(
        &self,
        ctx: &Context<'_>,
        canva_id: Uuid,
    ) -> async_graphql::Result<Vec<RfEdge>> {
        let state = ctx.data::<AppState>()?;
        let service = RfService {
            db: &state.db,
            events: &state.events,
        };
        let edges = service.get_rf_edges(canva_id).await?;
        Ok(edges)
    }
}
