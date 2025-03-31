use std::future::Future;

use crate::management::models::{
        events::{OrganizationEvent, TaskEvent},
        organization::{Organization, OrganizationId},
        task::{TaskId, TaskInstance},
    };

pub trait TaskRepository: Send + Sync + Clone + 'static {
    fn handle(&self, event: TaskEvent) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
    fn handle_many(
        &self,
        events: Vec<TaskEvent>,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
    fn publish(&self, event: TaskEvent) -> Result<(), anyhow::Error>;
    fn query_for_expired_tasks(
        &self,
    ) -> impl Future<Output = Result<Vec<TaskInstance>, anyhow::Error>> + Send;
    fn find_task_by_id(
        &self,
        id: TaskId,
    ) -> impl Future<Output = Result<TaskInstance, anyhow::Error>> + Send;
}

pub trait OrganizationRepository: Send + Sync + Clone + 'static {
    fn handle(
        &self,
        event: OrganizationEvent,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
    fn handle_many(
        &self,
        events: Vec<OrganizationEvent>,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
    fn publish(&self, event: OrganizationEvent);
    fn query_for_pending_task_repeats(&self) -> impl Future<Output = Vec<Organization>> + Send;
    fn find_org_by_id(
        &self,
        id: OrganizationId,
    ) -> impl Future<Output = Result<Organization, anyhow::Error>> + Send;
}
