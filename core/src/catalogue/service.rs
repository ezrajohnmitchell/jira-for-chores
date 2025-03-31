use std::future::Future;

use crate::{management::models::organization::OrganizationId, shared::account::AccountId};

use super::{task::CatalogueTask, CatalogueTaskId};

pub trait CatalogueRepository: Send + Sync + Clone + 'static {
    fn save(&self, task: &CatalogueTask) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
    fn get_by_id(
        &self,
        id: &CatalogueTaskId,
    ) -> impl Future<Output = Result<CatalogueTask, anyhow::Error>> + Send;
    fn delete_by_id(
        &self,
        id: &CatalogueTaskId,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}

pub struct CatalogueService<R>
where
    R: CatalogueRepository,
{
    repo: R,
}

impl<R> CatalogueService<R>
where
    R: CatalogueRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_task(
        &self,
        command: CreateTaskCommand,
    ) -> Result<CatalogueTaskId, anyhow::Error> {
        let id = CatalogueTaskId::new();
        let task = CatalogueTask {
            id,
            organization: command.organization,
            created_by: command.created_by,
            title: command.title,
            description: command.description,
        };

        self.repo.save(&task).await?;
        Ok(id)
    }

    pub async fn get_task(&self, id: CatalogueTaskId) -> Result<CatalogueTask, anyhow::Error> {
        Ok(self.repo.get_by_id(&id).await?)
    }

    pub async fn task_exists(&self, id: CatalogueTaskId) -> Result<bool, anyhow::Error> {
        Ok(self.repo.get_by_id(&id).await.is_ok())
    }

    pub async fn delete_task(&self, id: CatalogueTaskId) -> Result<(), anyhow::Error> {
        Ok(self.repo.delete_by_id(&id).await?)
    }
}

pub struct CreateTaskCommand {
    pub organization: OrganizationId,
    pub created_by: AccountId,
    pub title: String,
    pub description: String,
}
