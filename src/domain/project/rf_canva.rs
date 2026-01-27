use async_graphql::{SimpleObject, Union};
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::{db, graphql::EventBus};

#[derive(SimpleObject, FromRow, Debug, Clone)]
pub struct RfNode {
    pub id: Uuid,
    pub canva_id: Uuid,

    pub node_type: String,

    pub position_x: f64,
    pub position_y: f64,

    pub width: f64,
    pub height: f64,

    pub parent_id: Option<Uuid>,

    pub data: serde_json::Value,
}

#[derive(SimpleObject, FromRow, Debug, Clone)]
pub struct RfEdge {
    pub id: Uuid,
    pub canva_id: Uuid,

    pub edge_type: String,

    pub source_node_id: Uuid,
    pub target_node_id: Uuid,

    pub label: Option<String>,

    pub source_handle: Option<String>,
    pub target_handle: Option<String>,

    pub data: serde_json::Value,
}

#[derive(SimpleObject, FromRow, Debug, Clone)]
pub struct UpdateRfNode {
    pub node_type: Option<String>,

    pub position_x: Option<f64>,
    pub position_y: Option<f64>,

    pub width: Option<f64>,
    pub height: Option<f64>,

    pub parent_id: Option<Uuid>,

    pub data: Option<serde_json::Value>,
}

pub struct RfService<'a> {
    pub db: &'a PgPool,
    pub events: &'a EventBus,
}

impl<'a> RfService<'a> {
    pub async fn get_rf_nodes(&self, canva_id: Uuid) -> anyhow::Result<Vec<RfNode>> {
        let nodes = db::projects::rf_canva::get_rf_nodes(self.db, canva_id).await?;
        Ok(nodes)
    }

    pub async fn get_rf_edges(&self, canva_id: Uuid) -> anyhow::Result<Vec<RfEdge>> {
        let edges = db::projects::rf_canva::get_rf_edges(self.db, canva_id).await?;
        Ok(edges)
    }

    pub async fn insert_rf_node(
        &self,
        canva_id: Uuid,
        node_type: &str,
        position_x: f64,
        position_y: f64,
        width: f64,
        height: f64,
        parent_id: Option<Uuid>,
        data: Option<serde_json::Value>,
    ) -> anyhow::Result<RfNode> {
        let node_id = Uuid::new_v4();

        let node = RfNode {
            id: node_id,
            canva_id,
            node_type: node_type.to_string(),
            position_x,
            position_y,
            width,
            height,
            parent_id,
            data: data.unwrap_or_else(|| serde_json::json!({})),
        };

        db::projects::rf_canva::insert_rf_node(self.db, &node).await?;

        self.events.publish_rf(RfEvent::Created(RfEventCreated {
            canva_id,
            node_id,
            parent_id,
            node_type: node_type.to_string(),
        }));

        Ok(node)
    }

    pub async fn update_rf_node(
        &self,
        canva_id: Uuid,
        node_id: Uuid,
        node_type: Option<String>,
        position_x: Option<f64>,
        position_y: Option<f64>,
        width: Option<f64>,
        height: Option<f64>,
        parent_id: Option<Uuid>,
        data: Option<serde_json::Value>,
    ) -> anyhow::Result<RfNode> {
        let input = UpdateRfNode {
            node_type,
            position_x,
            position_y,
            width,
            height,
            parent_id,
            data,
        };
        let node =
            db::projects::rf_canva::update_rf_node(self.db, node_id, canva_id, input).await?;

        self.events
            .publish_rf(RfEvent::Updated(RfEventUpdated { canva_id, node_id }));

        Ok(node)
    }

    pub async fn insert_rf_edge(
        &self,
        canva_id: Uuid,
        edge_type: String,
        source_node_id: Uuid,
        target_node_id: Uuid,
        source_handle: Option<String>,
        target_handle: Option<String>,
        data: Option<serde_json::Value>,
    ) -> anyhow::Result<RfEdge> {
        let edge_id = Uuid::new_v4();

        let edge = RfEdge {
            id: edge_id,
            canva_id,
            edge_type: edge_type,
            source_node_id,
            target_node_id,
            label: None,
            source_handle,
            target_handle,
            data: data.unwrap_or_else(|| serde_json::json!({})),
        };

        db::projects::rf_canva::insert_rf_edge(self.db, &edge).await?;

        Ok(edge)
    }
}

/// Tree events for subscription
#[derive(Debug, Union, Clone)]
pub enum RfEvent {
    Created(RfEventCreated),
    Updated(RfEventUpdated),
}

impl RfEvent {
    pub fn canva_id(&self) -> Uuid {
        match self {
            RfEvent::Created(ev) => ev.canva_id,
            RfEvent::Updated(ev) => ev.canva_id,
        }
    }
}

#[derive(Debug, SimpleObject, Clone)]
pub struct RfEventCreated {
    pub canva_id: Uuid,
    pub node_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub node_type: String,
}

#[derive(Debug, SimpleObject, Clone)]
pub struct RfEventUpdated {
    pub canva_id: Uuid,
    pub node_id: Uuid,
}
