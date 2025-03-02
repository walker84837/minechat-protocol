use log::error;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io;

#[derive(Debug, Error, Diagnostic)]
pub enum MineChatError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Server not linked")]
    ServerNotLinked,

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Disconnected")]
    Disconnected,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MineChatMessage {
    #[serde(rename = "AUTH")]
    Auth { payload: AuthPayload },
    #[serde(rename = "AUTH_ACK")]
    AuthAck { payload: AuthAckPayload },
    #[serde(rename = "CHAT")]
    Chat { payload: ChatPayload },
    #[serde(rename = "BROADCAST")]
    Broadcast { payload: BroadcastPayload },
    #[serde(rename = "DISCONNECT")]
    Disconnect { payload: DisconnectPayload },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    pub client_uuid: String,
    pub link_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAckPayload {
    pub status: String,
    pub message: String,
    pub minecraft_uuid: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatPayload {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastPayload {
    pub from: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisconnectPayload {
    pub reason: String,
}
