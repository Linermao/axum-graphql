use async_graphql::{SimpleObject, Union};
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::{db, graphql::EventBus};

#[derive(SimpleObject, FromRow, Debug, Clone)]
pub struct TreeNode {
    pub id: Uuid,
    pub label: String,
    pub parent_id: Option<Uuid>,
    pub position: i32,
}

pub struct TreeService<'a> {
    pub db: &'a PgPool,
    pub events: &'a EventBus,
}

impl<'a> TreeService<'a> {
    pub async fn get_tree_nodes(&self, project_id: Uuid) -> anyhow::Result<Vec<TreeNode>> {
        let tree = db::projects::tree_nodes::get_tree_nodes(self.db, project_id).await?;
        Ok(tree)
    }

    pub async fn insert_tree_node(
        &self,
        project_id: Uuid,
        label: &str,
        parent_id: Option<Uuid>,
        position: i32,
    ) -> anyhow::Result<TreeNode> {
        let node_id = Uuid::new_v4();
        let node = TreeNode {
            id: node_id,
            label: label.to_string(),
            parent_id,
            position,
        };
        db::projects::tree_nodes::insert_tree_node(self.db, project_id, &node).await?;

        self.events
            .publish_tree(TreeEvent::Created(TreeEventCreated {
                project_id,
                node_id,
                parent_id,
                label: label.to_string(),
            }));

        Ok(node)
    }

    pub async fn delete_tree_node(&self, project_id: Uuid, node_id: Uuid) -> anyhow::Result<bool> {
        db::projects::tree_nodes::delete_tree_node(self.db, project_id, node_id).await?;

        self.events
            .publish_tree(TreeEvent::Deleted(TreeEventDeleted {
                project_id,
                node_id,
            }));

        Ok(true)
    }
}

/// Tree events for subscription
#[derive(Debug, Union, Clone)]
pub enum TreeEvent {
    Created(TreeEventCreated),
    Deleted(TreeEventDeleted),
}

impl TreeEvent {
    pub fn project_id(&self) -> Uuid {
        match self {
            TreeEvent::Created(ev) => ev.project_id,
            TreeEvent::Deleted(ev) => ev.project_id,
        }
    }
}

#[derive(Debug, SimpleObject, Clone)]
pub struct TreeEventCreated {
    pub project_id: Uuid,
    pub node_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub label: String,
}

#[derive(Debug, SimpleObject, Clone)]
pub struct TreeEventDeleted {
    pub project_id: Uuid,
    pub node_id: Uuid,
}
