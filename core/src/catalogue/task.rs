use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use ulid::Ulid;

use crate::{management::models::organization::OrganizationId, shared::account::AccountId};

#[derive(Debug, Clone, PartialEq, Copy, Default, Serialize, Deserialize)]
pub struct CatalogueTaskId(Ulid);

impl CatalogueTaskId {
    pub fn new() -> Self {
        Self(Ulid::new())
    }
    pub fn ulid(&self) -> Ulid {
        self.0
    }
}

impl From<Uuid> for CatalogueTaskId {
    fn from(value: Uuid) -> Self {
        CatalogueTaskId(value.into())
    }
}


#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CatalogueTask {
    pub id: CatalogueTaskId,
    pub organization: OrganizationId,
    pub created_by: AccountId,
    pub title: String,
    pub description: String,
}
