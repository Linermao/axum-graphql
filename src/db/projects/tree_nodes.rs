use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::project::tree::TreeNode;

pub async fn fetch_tree(pool: &PgPool, project_id: Uuid) -> Result<Vec<TreeNode>, sqlx::Error> {
    sqlx::query_as!(
        TreeNode,
        r#"
        SELECT id, label, parent_id, position
        FROM project_tree_nodes
        WHERE project_id = $1
        ORDER BY parent_id NULLS FIRST, position
        "#,
        project_id
    )
    .fetch_all(pool)
    .await
}

pub async fn insert_tree_node(
    pool: &PgPool,
    project_id: Uuid,
    node: &TreeNode,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO project_tree_nodes (id, project_id, label, parent_id, position)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        node.id,
        project_id,
        node.label,
        node.parent_id,
        node.position
    )
    .execute(pool)
    .await?;
    Ok(())
}
