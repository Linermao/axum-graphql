use crate::domain::project::Project;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_project(pool: &PgPool, name: &str) -> Result<Project, sqlx::Error> {
    let project_id = Uuid::new_v4();
    sqlx::query("INSERT INTO projects (project_id, name) VALUES ($1, $2)")
        .bind(project_id)
        .bind(name)
        .execute(pool)
        .await?;

    Ok(Project {
        project_id,
        name: name.to_string(),
    })
}

pub async fn get_projects(pool: &PgPool) -> Result<Vec<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>("SELECT project_id, name FROM projects")
        .fetch_all(pool)
        .await
}

pub async fn get_project_by_id(
    pool: &PgPool,
    project_id: Uuid,
) -> Result<Option<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>("SELECT project_id, name FROM projects WHERE project_id = $1")
        .bind(project_id)
        .fetch_optional(pool)
        .await
}
