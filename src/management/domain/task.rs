use chrono::{DateTime, Duration, TimeDelta, Utc};
use thiserror::Error;
use ulid::Ulid;

use super::{account::AccountId, events::TaskEvent};

#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub struct CatalogueTaskId;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct TaskId(pub Ulid);

impl TaskId {
    pub fn new() -> Self {
        TaskId(Ulid::new())
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TaskInstance {
    id: TaskId,
    catalogue_id: CatalogueTaskId,
    assigned_to: AccountId,
    assigned_by: AccountId,
    expires: Option<DateTime<Utc>>,
    status: TaskStatus,
}

impl TaskInstance {
    pub fn new(
        id: TaskId,
        assigned_to: AccountId,
        assigned_by: AccountId,
        expires: Option<DateTime<Utc>>,
        task: CatalogueTaskId,
        status: TaskStatus,
    ) -> Result<TaskInstance, TaskError> {
        Ok(TaskInstance {
            id,
            assigned_to,
            assigned_by,
            expires,
            catalogue_id: task,
            status,
        })
    }

    pub fn into_create_event(&self) -> TaskEvent {
        TaskEvent::Assigned {
            id: self.id,
            assigned_to: self.assigned_to,
            assigned_by: self.assigned_by,
            task: self.catalogue_id,
            expires: self.expires,
        }
    }

    pub fn finish(&self) -> Result<TaskEvent, TaskError> {
        match self.status {
            TaskStatus::Pending => Ok(TaskEvent::Finished { task_id: self.id }),
            TaskStatus::Finished | TaskStatus::Rejected | TaskStatus::Expired => {
                Err(TaskError::StatusNotApplicable)
            }
        }
    }

    pub fn reject(&self) -> Result<TaskEvent, TaskError> {
        match self.status {
            TaskStatus::Pending => Ok(TaskEvent::Rejected {
                task_id: self.id,
                assigned_by: self.assigned_by,
            }),
            TaskStatus::Finished | TaskStatus::Rejected | TaskStatus::Expired => {
                Err(TaskError::StatusNotApplicable)
            }
        }
    }

    pub fn expire(&self) -> Result<TaskEvent, TaskError> {
        match self.status {
            TaskStatus::Pending => Ok(TaskEvent::Expired {
                task_id: self.id,
                assigned_by: self.assigned_by,
            }),
            TaskStatus::Finished | TaskStatus::Rejected | TaskStatus::Expired => {
                Err(TaskError::StatusNotApplicable)
            }
        }
    }

    pub fn add_time(&self, time: Duration) -> Result<TaskEvent, TaskError> {
        match self.status {
            TaskStatus::Expired => Err(TaskError::StatusNotApplicable),
            _ => match self.expires {
                Some(_) => Ok(TaskEvent::TimeAdded {
                    task_id: self.id,
                    duration: time,
                }),
                None => Err(TaskError::TaskDoesNotExpire),
            },
        }
    }

    pub fn apply(mut self, event: &TaskEvent) -> Self {
        match event {
            TaskEvent::Assigned {
                id,
                assigned_to,
                assigned_by,
                task,
                expires,
            } => {
                self.id = id.clone();
                self.assigned_to = assigned_to.clone();
                self.assigned_by = assigned_by.clone();
                self.catalogue_id = task.clone();
                self.expires = *expires;
            }
            TaskEvent::Finished { task_id: _ } => self.status = TaskStatus::Finished,
            TaskEvent::TimeAdded {
                task_id: _,
                duration,
            } => {
                if let Some(mut expiration_time) = self.expires {
                    expiration_time += *duration;
                }
            }
            TaskEvent::Rejected {
                task_id: _,
                assigned_by: _,
            } => self.status = TaskStatus::Rejected,
            TaskEvent::Expired {
                task_id: _,
                assigned_by: _,
            } => self.status = TaskStatus::Expired,
        };

        self
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TaskStatus {
    #[default]
    Pending,
    Finished,
    Rejected,
    Expired,
}

#[derive(Debug, Clone)]
pub enum TimeRequestAction {
    Approve { add: TimeDelta },
    Deny,
}

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("action cannot be performed on task status")]
    StatusNotApplicable,
    #[error("task does not have an expiration")]
    TaskDoesNotExpire,
}
