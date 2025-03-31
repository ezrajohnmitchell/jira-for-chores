use chrono::{DateTime, Duration, Utc};

use crate::catalogue::CatalogueTaskId;
use crate::shared::account::AccountId;

use super::organization::{AccountType, OrganizationId, TagId};

use super::task::TaskId;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum TaskEvent {
    Assigned {
        id: TaskId,
        assigned_to: AccountId,
        assigned_by: AccountId,
        task: CatalogueTaskId,
        expires: Option<DateTime<Utc>>,
    },
    Finished {
        task_id: TaskId,
    },
    TimeAdded {
        task_id: TaskId,
        duration: Duration,
    },
    Rejected {
        task_id: TaskId,
        assigned_by: AccountId,
    },
    Expired {
        task_id: TaskId,
        assigned_by: AccountId,
    },
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum OrganizationEvent {
    Created {
        id: OrganizationId,
        name: String,
    },
    TagAdded {
        organization_id: OrganizationId,
        tag_id: TagId,
        name: String,
    },
    EditorAddedToTag {
        tag_id: TagId,
        account: AccountId,
    },
    WorkerAddedToTag {
        tag_id: TagId,
        account: AccountId,
    },
    TagRemoverd {
        tag: TagId,
    },
    AccountLinked {
        account: AccountId,
        account_type: AccountType,
    },
}
