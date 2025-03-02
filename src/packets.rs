use crate::protocol::*;
use log::{error, info};
use miette::Diagnostic;
use thiserror::Error;
use tokio::{
    io::{self, AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use uuid::Uuid;

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

pub async fn send_message<W>(writer: &mut W, msg: &MineChatMessage) -> Result<(), MineChatError>
where
    W: AsyncWrite + Unpin,
{
    info!("Serializing message {:?}", msg);
    let json = serde_json::to_string(msg)? + "\n";
    info!("Sending message to server");
    writer.write_all(json.as_bytes()).await?;
    Ok(())
}

pub async fn receive_message<R>(reader: &mut R) -> Result<MineChatMessage, MineChatError>
where
    R: AsyncBufRead + Unpin,
{
    let mut line = String::new();
    reader.read_line(&mut line).await?;
    Ok(serde_json::from_str(&line)?)
}

pub async fn handle_link(server_addr: &str, code: &str) -> Result<(String, String), MineChatError> {
    let client_uuid = Uuid::new_v4().to_string();
    info!("Connecting to server {}", server_addr);

    let mut stream = TcpStream::connect(server_addr).await?;
    let (reader, mut writer) = stream.split();

    info!("Connected to server");

    let mut reader = BufReader::new(reader);

    info!("Sending message to server {}", server_addr);
    send_message(
        &mut writer,
        &MineChatMessage::Auth {
            payload: AuthPayload {
                client_uuid: client_uuid.clone(),
                link_code: code.to_string(),
            },
        },
    )
    .await?;

    match receive_message(&mut reader).await? {
        MineChatMessage::AuthAck { payload } => {
            if payload.status != "success" {
                return Err(MineChatError::AuthFailed(payload.message));
            }
            info!("Linked successfully: {}", payload.message);
            Ok((server_addr.to_string(), client_uuid))
        }
        _ => Err(MineChatError::AuthFailed("Unexpected response".into())),
    }
}
