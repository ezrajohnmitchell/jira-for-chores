use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use ulid::Ulid;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Default, Hash, Eq, Ord, Deserialize, Serialize,
)]
pub struct AccountId(Ulid);

impl AccountId {
    pub fn new() -> Self {
        Self(Ulid::new()) 
    }

    pub fn ulid(&self) -> Ulid {
        self.0
    }
}

impl From<Uuid> for AccountId {
    fn from(value: Uuid) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Account {
    id: AccountId,
    username: String,
    password: String,
}
