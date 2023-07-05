use {
    super::{AppMessage, Request},
    crate::types::Username,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CheckAvailability(pub Username);

impl From<CheckAvailability> for AppMessage {
    fn from(value: CheckAvailability) -> Self {
        value.into()
    }
}

impl Request for CheckAvailability {
    type Response = (Username, bool);
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SuggestUsername;

impl From<SuggestUsername> for AppMessage {
    fn from(value: SuggestUsername) -> Self {
        value.into()
    }
}

impl Request for SuggestUsername {
    type Response = Username;
}
