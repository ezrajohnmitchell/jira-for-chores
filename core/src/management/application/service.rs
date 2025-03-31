use thiserror::Error;

use crate::{
    management::models::{
        organization::{
            AccountType, Organization, OrganizationError, OrganizationId, TagId, TaskAssignmentType,
        },
        task::{TaskDomainError, TaskId},
    },
    shared::{account::AccountId},
};

use super::{
    commands::*,
    ports::{OrganizationRepository, TaskRepository},
};

#[derive(Debug, Clone)]
pub struct ManagementService<T, O>
where
    T: TaskRepository,
    O: OrganizationRepository,
{
    task_repo: T,
    org_repo: O,
}

impl<R, O> ManagementService<R, O>
where
    R: TaskRepository,
    O: OrganizationRepository,
{
    pub fn new(task_repo: R, org_repo: O) -> Self {
        Self {
            task_repo,
            org_repo,
        }
    }

    pub async fn create_org(
        &self,
        command: CreateOrgCommand,
    ) -> Result<OrganizationId, anyhow::Error> {
        let org = Organization::create(command.name, command.requesting_account)?;
        self.org_repo.handle_many(org.into_create_event()?).await?;
        Ok(org.id().clone())
    }

    pub async fn link_account(&self, command: AccountLinkCommand) -> Result<(), anyhow::Error> {
        let org = self.org_repo.find_org_by_id(command.orgainzation).await?;
        self.org_repo
            .handle(org.link_account(
                command.requesting_account,
                command.account,
                command.account_type,
            )?)
            .await?;
        Ok(())
    }

    pub async fn assign_tasks(&self, command: AssignTaskCommand) -> Result<(), anyhow::Error> {
        let org = self.org_repo.find_org_by_id(command.organization).await?;
        let tasks = org.assign_tasks_to_tags(
            &command.requesting_account,
            &command.tags,
            &command.tasks,
            &command.assignment_type,
        )?;
        self.task_repo
            .handle_many(tasks.iter().map(|task| task.create()).collect())
            .await?;
        Ok(())
    }

    pub async fn finish_task(&self, command: FinishTaskCommand) -> Result<(), anyhow::Error> {
        let task = self.task_repo.find_task_by_id(command.task).await?;
        self.task_repo
            .handle(task.finish(command.requesting_account)?)
            .await?;
        Ok(())
    }

    pub async fn reject_task(&self, command: FinishTaskCommand) -> Result<(), anyhow::Error> {
        let task = self.task_repo.find_task_by_id(command.task).await?;
        self.task_repo
            .handle(task.reject(command.requesting_account)?)
            .await?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ManagementError {
    #[error("invalid task")]
    TaskError(#[from] TaskDomainError),
    #[error("invalid organization")]
    OrganizationError(#[from] OrganizationError),
}
