use sqlx::{postgres::PgPoolOptions, types::Uuid};

use super::{service::CatalogueRepository, task::CatalogueTask};

#[derive(Debug, Clone)]
pub struct PostgressCatalogueRepository {
    pool: sqlx::PgPool,
}

impl PostgressCatalogueRepository {
    pub async fn new(path: &str) -> anyhow::Result<PostgressCatalogueRepository> {
        let pool = PgPoolOptions::new()
            .test_before_acquire(false)
            .connect(path)
            .await?;

        Ok(Self { pool })
    }
}

impl CatalogueRepository for PostgressCatalogueRepository {
    async fn save(
        &self,
        task: &super::task::CatalogueTask,
    ) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "INSERT INTO CATALOGUE_TASK (id, organization, created_by, title, description)
            VALUES ($1, $2, $3, $4, $5)",
            Uuid::from(task.id.ulid()),
            Uuid::from(task.organization.ulid()),
            Uuid::from(task.created_by.ulid()),
            task.title,
            task.description
        ).execute(&self.pool).await?;

        Ok(())
    }

    async fn get_by_id(
        &self,
        id: &super::CatalogueTaskId,
    ) -> Result<super::task::CatalogueTask, anyhow::Error> {
        let record = sqlx::query!(
            "SELECT id, organization, created_by, title, description
            FROM CATALOGUE_TASK 
            WHERE id = $1",
            Uuid::from(id.ulid())
        ).fetch_one(&self.pool).await?;

        Ok(CatalogueTask{
            id: record.id.into(),
            organization: record.organization.into(),
            created_by: record.created_by.into(),
            title: record.title,
            description: record.description,
        })
    }

    async fn delete_by_id(
        &self,
        id: &super::CatalogueTaskId,
    ) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "DELETE FROM 
            CATALOGUE_TASK
            WHERE id = $1",
            Uuid::from(id.ulid())
        ).execute(&self.pool).await?;

        Ok(())
    }
}