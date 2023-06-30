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
pub struct GenerateUsername;

impl From<GenerateUsername> for AppMessage {
    fn from(value: GenerateUsername) -> Self {
        value.into()
    }
}

impl Request for GenerateUsername {
    type Response = Username;
}
