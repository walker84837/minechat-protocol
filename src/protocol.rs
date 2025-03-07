use log::error;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io;

/// Errors that can occur during the operation of the MineChat protocol.
#[derive(Debug, Error, Diagnostic)]
pub enum MineChatError {
    /// I/O error. Contains the underlying error.
    #[error("I/O error: {0}")]
    #[diagnostic(code(minechat::io))]
    Io(#[from] io::Error),

    /// Serde error. Contains the underlying JSON error.
    #[error("Serde error: {0}")]
    #[diagnostic(code(minechat::serde))]
    Serde(#[from] serde_json::Error),

    /// Server not linked.
    #[error("Server not linked")]
    ServerNotLinked,

    /// Configuration error.
    #[error("Config error: {0}")]
    #[diagnostic(code(minechat::config_error), help = "Check your configuration file")]
    ConfigError(String),

    /// Authentication failed.
    #[error("Authentication failed: {0}")]
    #[diagnostic(
        code(minechat::auth_failed),
        help = "Try logging in again with valid credentials"
    )]
    AuthFailed(String),

    /// UUID error.
    #[error("UUID error: {0}")]
    #[diagnostic(code(minechat::uuid))]
    Uuid(#[from] uuid::Error),

    /// Disconnected.
    #[error("Disconnected")]
    #[diagnostic(
        code(minechat::disconnected),
        help = "If this is unexpected, try reconnecting"
    )]
    Disconnected,
}

/// The different types of messages that can be sent and received in the MineChat protocol.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MineChatMessage {
    /// An authentication message, containing the client's UUID and link code.
    #[serde(rename = "AUTH")]
    Auth { payload: AuthPayload },

    /// An acknowledgment of a successful authentication, containing the server's response.
    #[serde(rename = "AUTH_ACK")]
    AuthAck { payload: AuthAckPayload },

    /// A chat message, containing the message text.
    #[serde(rename = "CHAT")]
    Chat { payload: ChatPayload },

    /// A broadcast message, containing the message text and the sender's name.
    #[serde(rename = "BROADCAST")]
    Broadcast { payload: BroadcastPayload },

    /// A disconnect message, containing the reason for the disconnection.
    #[serde(rename = "DISCONNECT")]
    Disconnect { payload: DisconnectPayload },
}

/// The payload for an authentication message.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    /// The client's UUID.
    pub client_uuid: String,
    /// The link code used to authenticate with the server.
    pub link_code: String,
}

/// The payload for an authentication acknowledgment message.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAckPayload {
    /// The status of the authentication (either "success" or "failure").
    pub status: String,
    /// A message describing the authentication status.
    pub message: String,
    /// The client's Minecraft UUID, if available.
    pub minecraft_uuid: Option<String>,
    /// The client's Minecraft username, if available.
    pub username: Option<String>,
}

/// The payload for a chat message.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatPayload {
    /// The text of the chat message.
    pub message: String,
}

/// The payload for a broadcast message.
#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastPayload {
    /// The name of the sender.
    pub from: String,
    /// The text of the broadcast message.
    pub message: String,
}

/// The payload for a disconnect message.
#[derive(Debug, Serialize, Deserialize)]
pub struct DisconnectPayload {
    /// The reason for the disconnection.
    pub reason: String,
}
