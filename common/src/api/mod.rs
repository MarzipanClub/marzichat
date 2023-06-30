//! The api module.

use {
    crate::types::*,
    serde::{Deserialize, Serialize},
    std::time::Duration,
};

pub mod username;

/// The heartbeat interval for app to server messages.
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(3);

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum AppMessage {
    Ping,
    Heartbeat,
    GenerateUsername,
    CheckUsernameAvailability(Username),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum BackendMessage {
    Pong,
    UsernameAvailability((Username, bool)),
    GeneratedUsername(Username),
}

/// A trait to signify that a message expects at least one response.
pub trait Request: Into<AppMessage> {
    type Response: PartialEq + Eq + 'static;
}

/// A trait to signify that a message does not expect a response.
pub trait Notify: Into<AppMessage> {}
