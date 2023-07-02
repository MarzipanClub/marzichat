//! The api module.

use {
    crate::types::*,
    serde::{Deserialize, Serialize},
    std::time::Duration,
};

pub mod username;

/// The interval for sending ping websocket messages
pub const PING_INTERVAL: Duration = Duration::from_secs(2);

/// The timeout for a pong websocket message.
/// It should be greater than the ping interval.
pub const PONG_TIMEOUT: Duration = Duration::from_secs(4);

/// The payload for a ping and pong websocket messages.
pub const PING_PONG_PAYLOAD: &[u8] = b"";

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum AppMessage {
    GenerateUsername,
    CheckUsernameAvailability(Username),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum BackendMessage {
    UsernameAvailability((Username, bool)),
    GeneratedUsername(Username),
}

/// A trait to signify that a message expects at least one response.
pub trait Request: Into<AppMessage> {
    type Response: PartialEq + Eq + 'static;
}

/// A trait to signify that a message does not expect a response.
pub trait Notify: Into<AppMessage> {}
