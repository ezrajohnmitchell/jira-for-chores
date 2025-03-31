use std::{collections::HashSet, vec};

use chrono::{DateTime, Days, Utc};
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use thiserror::Error;
use ulid::Ulid;

use crate::{catalogue::CatalogueTaskId, shared::account::AccountId};

use super::{
    events::OrganizationEvent,
    task::{TaskDomainError, TaskId, TaskInstance, TaskStatus::Pending},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Default, Deserialize, Serialize)]
pub struct OrganizationId(pub Ulid);

impl OrganizationId {
    pub fn new() -> OrganizationId {
        OrganizationId(Ulid::new())
    }

    pub fn ulid(&self) -> Ulid {
        self.0
    }
}

impl From<Uuid> for OrganizationId {
    fn from(value: Uuid) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Organization {
    id: OrganizationId,
    name: String,
    tags: Vec<Tag>,
    linked_accounts: Vec<AccountLink>,
}

impl Organization {
    pub fn id(&self) -> &OrganizationId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new(
        id: OrganizationId,
        name: String,
        tags: Vec<Tag>,
        linked_accounts: Vec<AccountLink>,
    ) -> Result<Organization, OrganizationError> {
        Ok(Organization {
            id,
            name,
            tags,
            linked_accounts,
        })
    }

    pub fn create(name: String, account: AccountId) -> Result<Organization, OrganizationError> {
        Ok(Organization {
            id: OrganizationId::new(),
            name,
            tags: Vec::new(),
            linked_accounts: vec![AccountLink {
                account,
                account_type: AccountType::Owner,
                tasks: Vec::new(),
            }],
        })
    }

    pub fn into_create_event(&self) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        if self.linked_accounts.len() == 0 {
            return Err(OrganizationError::CannotCreate);
        }
        let mut links: Vec<OrganizationEvent> = self
            .linked_accounts
            .iter()
            .map(|link| OrganizationEvent::AccountLinked {
                account: link.account,
                account_type: link.account_type,
            })
            .collect();
        links.splice(
            0..0,
            vec![OrganizationEvent::Created {
                id: self.id,
                name: self.name.clone(),
            }],
        );
        Ok(links)
    }

    pub fn add_tag(
        &self,
        name: String,
        requesting_account: AccountId,
    ) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        self.linked_accounts
            .iter()
            .find(|link| link.account == requesting_account)
            .ok_or(OrganizationError::NotAuthorized)?;
        self.tags
            .iter()
            .find(|existing_tag| existing_tag.name == name)
            .ok_or(OrganizationError::TagAlreadyExists)?;

        let id = TagId::new();
        Ok(vec![
            OrganizationEvent::TagAdded {
                name,
                organization_id: self.id,
                tag_id: id,
            },
            OrganizationEvent::EditorAddedToTag {
                tag_id: id,
                account: requesting_account,
            },
        ])
    }

    pub fn add_worker_to_tag(
        &self,
        tag_id: TagId,
        requesting_account: AccountId,
        worker: AccountId,
    ) -> Result<OrganizationEvent, OrganizationError> {
        let tag = self
            .tags
            .iter()
            .find(|&existing_tag| existing_tag.id == tag_id)
            .ok_or(OrganizationError::TagDoesNotExist)?;
        tag.authorized_editors
            .iter()
            .find(|&&editor| editor == requesting_account)
            .ok_or(OrganizationError::NotAuthorized)?;

        Ok(OrganizationEvent::WorkerAddedToTag {
            tag_id: tag_id,
            account: worker,
        })
    }

    pub fn add_editor_to_tag(
        &self,
        tag_id: TagId,
        requesting_account: AccountId,
        editor: AccountId,
    ) -> Result<OrganizationEvent, OrganizationError> {
        let tag = self
            .tags
            .iter()
            .find(|&existing_tag| existing_tag.id == tag_id)
            .ok_or(OrganizationError::TagDoesNotExist)?;
        tag.authorized_editors
            .iter()
            .find(|&&editor| editor == requesting_account)
            .ok_or(OrganizationError::NotAuthorized)?;

        Ok(OrganizationEvent::EditorAddedToTag {
            tag_id: tag_id,
            account: editor,
        })
    }

    pub fn link_account(
        &self,
        requesting_account: AccountId,
        worker: AccountId,
        link_type: AccountType,
    ) -> Result<OrganizationEvent, OrganizationError> {
        self.linked_accounts
            .iter()
            .find(|link| {
                link.account == requesting_account && link.account_type != AccountType::Worker
            })
            .ok_or(OrganizationError::NotAuthorized)?;
        if link_type == AccountType::Owner {
            return Err(OrganizationError::NotAuthorized);
        }

        Ok(OrganizationEvent::AccountLinked {
            account: worker,
            account_type: link_type,
        })
    }

    pub fn transfer_ownership(
        &self,
        requesting_account: AccountId,
        new_owner: AccountId,
    ) -> Result<OrganizationEvent, OrganizationError> {
        todo!();
    }

    pub fn assign_tasks_to_tags(
        &self,
        requesting_account: &AccountId,
        tags: &HashSet<TagId>,
        tasks: &Vec<CatalogueTaskId>,
        assignment_type: &TaskAssignmentType,
    ) -> Result<Vec<TaskInstance>, OrganizationError> {
        //verify requesting account is an editor for all groups requested
        let tags: Vec<&Tag> = self
            .tags
            .iter()
            .filter(|tag| tags.contains(&tag.id))
            .collect();
        let link = self
            .linked_accounts
            .iter()
            .find(|link| link.account == *requesting_account)
            .ok_or(OrganizationError::NotInOrg)?;

        match link.account_type {
            AccountType::Worker | AccountType::Admin => {
                let is_authorized = tags
                    .iter()
                    .map(|tag| &tag.authorized_editors)
                    .all(|editors| editors.contains(requesting_account));

                if is_authorized {
                    Ok(())
                } else {
                    Err(OrganizationError::NotAuthorized)
                }
            }
            AccountType::Owner => Ok(()),
        }?;

        //get workers that exist in all groups
        let sets: Vec<&HashSet<AccountId>> = tags.iter().map(|tag| &tag.workers).collect();

        let workers: Vec<AccountId> = Vec::from_iter(match sets.len() {
            0 => HashSet::new(),
            _ => sets[1..].iter().fold(sets[0].clone(), |mut acc, set| {
                acc.retain(|account| set.contains(account));
                acc
            }),
        });

        if workers.len() == 0 {
            return Err(OrganizationError::NoWorkers);
        }

        match assignment_type {
            TaskAssignmentType::Random => {
                let mut rng = rand::rng();
                let out: Result<Vec<TaskInstance>, TaskDomainError> = tasks
                    .iter()
                    .map(|task| {
                        let worker = workers.choose(&mut rng).unwrap();
                        TaskInstance::new(
                            TaskId::new(),
                            *worker,
                            *requesting_account,
                            None,
                            task.clone(),
                            Pending,
                        )
                    })
                    .collect();

                Ok(out?)
            }
            TaskAssignmentType::LowestTasks => {
                let mut out: Vec<TaskInstance> = Vec::new();
                let mut workers_with_tasks: Vec<(&AccountId, usize)> = workers
                    .iter()
                    .map(|worker| {
                        (
                            worker,
                            self.linked_accounts
                                .iter()
                                .find(|link| link.account == *worker)
                                .map(|link| link.tasks.len())
                                .unwrap(),
                        )
                    })
                    .collect();

                for task in tasks {
                    let min = workers_with_tasks.iter_mut().min_by_key(|worker| worker.1);
                    match min {
                        Some(worker) => {
                            let task_newd = TaskInstance::new(
                                TaskId::new(),
                                worker.0.clone(),
                                *requesting_account,
                                None,
                                *task,
                                Pending,
                            )?;
                            out.push(task_newd);
                            worker.1 += 1;
                        }
                        None => return Err(OrganizationError::NoWorkers),
                    };
                }

                Ok(out)
            }
            TaskAssignmentType::HighestTasks => {
                let mut out: Vec<TaskInstance> = Vec::new();
                let mut workers_with_tasks: Vec<(&AccountId, usize)> = workers
                    .iter()
                    .map(|worker| {
                        (
                            worker,
                            self.linked_accounts
                                .iter()
                                .find(|link| link.account == *worker)
                                .map(|link| link.tasks.len())
                                .unwrap(),
                        )
                    })
                    .collect();

                for task in tasks {
                    let max = workers_with_tasks.iter_mut().max_by_key(|worker| worker.1);
                    match max {
                        Some(worker) => {
                            let task_newd = TaskInstance::new(
                                TaskId::new(),
                                worker.0.clone(),
                                *requesting_account,
                                None,
                                *task,
                                Pending,
                            )?;
                            out.push(task_newd);
                            worker.1 += 1;
                        }
                        None => return Err(OrganizationError::NoWorkers),
                    };
                }

                Ok(out)
            }
            TaskAssignmentType::Copy => {
                let output: Result<Vec<TaskInstance>, TaskDomainError> = tasks
                    .iter()
                    .flat_map(|&task| {
                        let mut out = Vec::new();
                        for worker in &workers {
                            out.push(TaskInstance::new(
                                TaskId::new(),
                                *worker,
                                *requesting_account,
                                None,
                                task.clone(),
                                Pending,
                            ));
                        }
                        out
                    })
                    .collect();

                Ok(output?)
            }
            TaskAssignmentType::ToAccount { account } => {
                match workers.iter().find(|&worker| *worker == *account) {
                    Some(_) => {
                        let output: Result<Vec<TaskInstance>, TaskDomainError> = tasks
                            .iter()
                            .map(|task| {
                                TaskInstance::new(
                                    TaskId::new(),
                                    *account,
                                    *requesting_account,
                                    None,
                                    task.clone(),
                                    Pending,
                                )
                            })
                            .collect();

                        Ok(output?)
                    }
                    None => Err(OrganizationError::NoWorkers),
                }
            }
        }
    }

    pub fn assign_tasks_to_account(
        &self,
        requesting_account: AccountId,
        worker: AccountId,
        tasks: &Vec<CatalogueTaskId>,
    ) -> Result<Vec<TaskInstance>, OrganizationError> {
        let link = self
            .linked_accounts
            .iter()
            .find(|&link| link.account == requesting_account)
            .ok_or(OrganizationError::NotInOrg)?;

        if worker == requesting_account || link.account_type != AccountType::Worker {
            let out: Result<Vec<TaskInstance>, TaskDomainError> = tasks
                .iter()
                .map(|task| {
                    TaskInstance::new(
                        TaskId::new(),
                        worker,
                        requesting_account,
                        None,
                        *task,
                        Pending,
                    )
                })
                .collect();
            return Ok(out?);
        }

        Err(OrganizationError::NotAuthorized)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct TagId(pub Ulid);

impl TagId {
    pub fn new() -> TagId {
        TagId(Ulid::new())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    id: TagId,
    name: String,
    authorized_editors: HashSet<AccountId>,
    workers: HashSet<AccountId>,
}

impl Tag {
    pub fn new(
        id: TagId,
        name: String,
        authorized_editors: HashSet<AccountId>,
        workers: HashSet<AccountId>,
    ) -> Tag {
        Self {
            id,
            name,
            authorized_editors,
            workers,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AccountLink {
    account: AccountId,
    account_type: AccountType,
    tasks: Vec<TaskId>,
}

impl AccountLink {
    pub fn new(account: AccountId, account_type: AccountType, tasks: Vec<TaskId>) -> AccountLink {
        Self {
            account,
            account_type,
            tasks,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum AccountType {
    Worker,
    Admin,
    Owner,
}

#[derive(Debug, Clone)]
pub struct RepeatingTask {
    id: Ulid,
    last_assigned: DateTime<Utc>,
    requesting_account: AccountId,
    period: Days,
    assigned_to: AssignmentType,
    tasks: Vec<CatalogueTaskId>,
}

impl PartialEq for RepeatingTask {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone)]
enum AssignmentType {
    Account(AccountId),
    Tags {
        tags: HashSet<TagId>,
        assignment_type: TaskAssignmentType,
    },
}

#[derive(Error, Debug)]
pub enum OrganizationError {
    #[error("cannot create organization")]
    CannotCreate,
    #[error("tag already exists")]
    TagAlreadyExists,
    #[error("tag does not exist")]
    TagDoesNotExist,
    #[error("no accounts available to assign in tags")]
    NoWorkers,
    #[error("error creating task")]
    TaskError(#[from] TaskDomainError),
    #[error("not authorized for tags")]
    NotAuthorized,
    #[error("requesting account is not part of this organization")]
    NotInOrg,
    #[error("repeating task date is invalid")]
    InvalidRepeatingTask,
}

#[derive(Debug, Clone)]
pub enum TaskAssignmentType {
    Random,
    Copy,
    LowestTasks,
    HighestTasks,
    ToAccount { account: AccountId },
}
