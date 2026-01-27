pub mod rf_canva;
pub mod tree_nodes;

use crate::domain::project::Project;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_projects(pool: &PgPool) -> Result<Vec<Project>, sqlx::Error> {
    sqlx::query_as!(
        Project,
        "SELECT project_id, name, default_rf_canva_id FROM projects"
    )
    .fetch_all(pool)
    .await
}

pub async fn get_project_by_id(
    pool: &PgPool,
    project_id: Uuid,
) -> Result<Option<Project>, sqlx::Error> {
    sqlx::query_as!(
        Project,
        "SELECT project_id, name, default_rf_canva_id FROM projects WHERE project_id = $1",
        project_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn insert_project(
    project_id: Uuid,
    name: &str,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO projects (project_id, name) VALUES ($1, $2)",
        project_id,
        name,
    )
    .execute(pool)
    .await?;

    Ok(())
}
