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
    pub async fn tree(&self, project_id: Uuid) -> anyhow::Result<Vec<TreeNode>> {
        let tree = db::projects::tree_nodes::fetch_tree(self.db, project_id).await?;
        Ok(tree)
    }

    pub async fn insert_tree_node(
        &self,
        project_id: Uuid,
        label: &str,
        parent_id: Option<Uuid>,
    ) -> anyhow::Result<TreeNode> {
        let node_id = Uuid::new_v4();
        let position = 0;
        let node = TreeNode {
            id: node_id,
            label: label.to_string(),
            parent_id,
            position,
        };
        db::projects::tree_nodes::insert_tree_node(self.db, project_id, &node).await?;

        self.events
            .publish_tree(TreeEvent::Created(TreeEventCreated {
                node_id,
                parent_id,
                label: label.to_string(),
            }));

        Ok(node)
    }
}

/// Tree events for subscription
#[derive(Debug, Union, Clone)]
pub enum TreeEvent {
    Created(TreeEventCreated),
}

#[derive(Debug, SimpleObject, Clone)]
pub struct TreeEventCreated {
    pub node_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub label: String,
}
