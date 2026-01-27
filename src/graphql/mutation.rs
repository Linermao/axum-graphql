use async_graphql::{Context, Object};
use uuid::Uuid;

use crate::{
    domain::project::{
        Project, ProjectService,
        rf_canva::{RfEdge, RfNode, RfService},
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

    async fn create_tree_node(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
        label: String,
        parent_id: Option<Uuid>,
        position: i32,
    ) -> async_graphql::Result<TreeNode> {
        let state = ctx.data::<AppState>()?;
        let service = TreeService {
            db: &state.db,
            events: &state.events,
        };

        let node = service
            .insert_tree_node(project_id, &label, parent_id, position)
            .await?;

        Ok(node)
    }

    async fn delete_tree_node(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
        node_id: Uuid,
    ) -> async_graphql::Result<bool> {
        let state = ctx.data::<AppState>()?;
        let service = TreeService {
            db: &state.db,
            events: &state.events,
        };

        let result = service.delete_tree_node(project_id, node_id).await?;

        Ok(result)
    }

    async fn create_rf_node(
        &self,
        ctx: &Context<'_>,
        canva_id: Uuid,
        node_type: String,
        position_x: f64,
        position_y: f64,
        width: f64,
        height: f64,
        parent_id: Option<Uuid>,
        data: Option<serde_json::Value>,
    ) -> async_graphql::Result<RfNode> {
        let state = ctx.data::<AppState>()?;
        let service = RfService {
            db: &state.db,
            events: &state.events,
        };

        let node = service
            .insert_rf_node(
                canva_id, &node_type, position_x, position_y, width, height, parent_id, data,
            )
            .await?;

        Ok(node)
    }

    async fn update_rf_node(
        &self,
        ctx: &Context<'_>,
        canva_id: Uuid,
        node_id: Uuid,
        node_type: Option<String>,
        position_x: Option<f64>,
        position_y: Option<f64>,
        width: Option<f64>,
        height: Option<f64>,
        parent_id: Option<Uuid>,
        data: Option<serde_json::Value>,
    ) -> async_graphql::Result<RfNode> {
        let state = ctx.data::<AppState>()?;
        let service = RfService {
            db: &state.db,
            events: &state.events,
        };

        let node = service
            .update_rf_node(
                canva_id, node_id, node_type, position_x, position_y, width, height, parent_id,
                data,
            )
            .await?;

        Ok(node)
    }

    async fn create_rf_edge(
        &self,
        ctx: &Context<'_>,
        canva_id: Uuid,
        edge_type: String,
        source_node_id: Uuid,
        target_node_id: Uuid,
        source_handle: Option<String>,
        target_handle: Option<String>,
        data: Option<serde_json::Value>,
    ) -> async_graphql::Result<RfEdge> {
        let state = ctx.data::<AppState>()?;
        let service = RfService {
            db: &state.db,
            events: &state.events,
        };

        let edge = service
            .insert_rf_edge(
                canva_id,
                edge_type,
                source_node_id,
                target_node_id,
                source_handle,
                target_handle,
                data,
            )
            .await?;

        Ok(edge)
    }
}
