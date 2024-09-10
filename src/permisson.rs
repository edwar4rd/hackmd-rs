use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RWPermission {
    #[serde(rename = "guest")]
    Guest,
    #[serde(rename = "signed_in")]
    SignedIn,
    #[serde(rename = "owner")]
    Owner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CommentPermisson {
    #[serde(rename = "everyone")]
    Everyone,
    #[serde(rename = "signed_in_users")]
    SignedIn,
    #[serde(rename = "owners")]
    Owners,
    #[serde(rename = "forbidden")]
    Forbidden,
    #[serde(rename = "disabled")]
    Disabled,
}
