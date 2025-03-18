use ulid::Ulid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Hash, Eq, Ord)]
pub struct AccountId(Ulid);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Account {
    id: AccountId,
    username: String,
    password: String,
}
