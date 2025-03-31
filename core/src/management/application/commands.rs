use std::collections::HashSet;

use serde::Deserialize;

use crate::{
    catalogue::CatalogueTaskId,
    management::models::{
        organization::{AccountType, OrganizationId, TagId, TaskAssignmentType},
        task::TaskId,
    },
    shared::account::AccountId,
};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateOrgCommand {
    pub name: String,
    pub requesting_account: AccountId,
}

pub struct AccountLinkCommand {
    pub orgainzation: OrganizationId,
    pub requesting_account: AccountId,
    pub account: AccountId,
    pub account_type: AccountType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FinishTaskCommand {
    pub task: TaskId,
    pub requesting_account: AccountId,
}

#[derive(Debug, Clone)]
pub struct AssignTaskCommand {
    pub organization: OrganizationId,
    pub tasks: Vec<CatalogueTaskId>,
    pub requesting_account: AccountId,
    pub assignment_type: TaskAssignmentType,
    pub tags: HashSet<TagId>,
}
