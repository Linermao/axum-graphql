use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::project::rf_canva::{RfEdge, RfNode, UpdateRfNode};

pub async fn get_rf_nodes(pool: &PgPool, canva_id: Uuid) -> Result<Vec<RfNode>, sqlx::Error> {
    sqlx::query_as!(
        RfNode,
        r#"
        SELECT
            id,
            canva_id,
            node_type,
            position_x,
            position_y,
            width,
            height,
            parent_id,
            data
        FROM rf_nodes
        WHERE canva_id = $1
        ORDER BY z_index ASC, created_at ASC
        "#,
        canva_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_rf_edges(pool: &PgPool, canva_id: Uuid) -> Result<Vec<RfEdge>, sqlx::Error> {
    sqlx::query_as!(
        RfEdge,
        r#"
        SELECT
            id,
            canva_id,
            edge_type,
            source_node_id,
            target_node_id,
            label,
            source_handle,
            target_handle,
            data
        FROM rf_edges
        WHERE canva_id = $1
        ORDER BY created_at ASC
        "#,
        canva_id
    )
    .fetch_all(pool)
    .await
}

pub async fn insert_rf_node(pool: &PgPool, node: &RfNode) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO rf_nodes (
            id,
            canva_id,
            node_type,
            position_x,
            position_y,
            width,
            height,
            parent_id,
            data
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        node.id,
        node.canva_id,
        node.node_type,
        node.position_x,
        node.position_y,
        node.width,
        node.height,
        node.parent_id,
        node.data,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_rf_node(
    pool: &PgPool,
    node_id: Uuid,
    canva_id: Uuid,
    input: UpdateRfNode,
) -> Result<RfNode, sqlx::Error> {
    let mut query = sqlx::QueryBuilder::new("UPDATE rf_nodes SET ");
    let mut separated = query.separated(", ");

    let mut updated = false;

    if let Some(node_type) = &input.node_type {
        separated
            .push("node_type = ")
            .push_bind_unseparated(node_type);
        updated = true;
    }
    if let Some(x) = input.position_x {
        separated.push("position_x = ").push_bind_unseparated(x);
        updated = true;
    }
    if let Some(y) = input.position_y {
        separated.push("position_y = ").push_bind_unseparated(y);
        updated = true;
    }
    if let Some(w) = input.width {
        separated.push("width = ").push_bind_unseparated(w);
        updated = true;
    }
    if let Some(h) = input.height {
        separated.push("height = ").push_bind_unseparated(h);
        updated = true;
    }
    if let Some(pid) = input.parent_id {
        separated.push("parent_id = ").push_bind_unseparated(pid);
        updated = true;
    }
    if let Some(data) = &input.data {
        separated.push("data = ").push_bind_unseparated(data);
        updated = true;
    }

    if !updated {
        return Err(sqlx::Error::Protocol(
            "update_rf_node called with no fields to update".into(),
        ));
    }

    query
        .push(" WHERE id = ")
        .push_bind(node_id)
        .push(" AND canva_id = ")
        .push_bind(canva_id)
        .push(" RETURNING *");

    query.build_query_as::<RfNode>().fetch_one(pool).await
}

pub async fn insert_rf_edge(pool: &PgPool, edge: &RfEdge) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO rf_edges (
            id,
            canva_id,
            source_node_id,
            target_node_id,
            edge_type,
            source_handle,
            target_handle
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        edge.id,
        edge.canva_id,
        edge.source_node_id,
        edge.target_node_id,
        edge.edge_type,
        edge.source_handle,
        edge.target_handle
    )
    .execute(pool)
    .await?;

    Ok(())
}
