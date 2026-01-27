pub mod rf_canva;
pub mod tree;

use async_graphql::SimpleObject;
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::db;

/// Domain model for Project
#[derive(SimpleObject, FromRow, Debug, Clone)]
pub struct Project {
    pub project_id: Uuid,
    pub name: String,

    pub default_rf_canva_id: Option<Uuid>,
}

pub struct ProjectService<'a> {
    pub db: &'a PgPool,
}

impl<'a> ProjectService<'a> {
    pub async fn get_projects(&self) -> anyhow::Result<Vec<Project>> {
        let projects = db::projects::get_projects(self.db).await?;
        Ok(projects)
    }

    pub async fn get_project_by_id(&self, project_id: Uuid) -> anyhow::Result<Option<Project>> {
        let project = db::projects::get_project_by_id(self.db, project_id).await?;
        Ok(project)
    }

    pub async fn insert_project(&self, name: &str) -> anyhow::Result<Project> {
        let project_id = Uuid::new_v4();
        db::projects::insert_project(project_id, name, self.db).await?;

        Ok(Project {
            project_id,
            name: name.to_string(),
            default_rf_canva_id: None,
        })
    }
}
