use std::future::Future;

use super::{events::TaskEvent, organization::OrganizationError, task::TaskError};

pub trait TaskRepository: Send + Sync + 'static {
    fn handle(&self, event: TaskEvent) -> impl Future<Output = Result<(), TaskError>> + Send;
    fn publish(&self, event: TaskEvent) -> Result<(), TaskError>;
}

pub trait OrganizationRepository: Send + Sync + 'static {
    fn handle(&self, event: TaskEvent) -> impl Future<Output = Result<(), TaskError>> + Send;
    fn publish(&self, event: TaskEvent) -> Result<(), OrganizationError>;
}
