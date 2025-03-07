use crate::protocol::*;
use log::trace;
use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use uuid::Uuid;

/// Sends a message to the server.
///
/// # Arguments
///
/// * `writer` - A mutable reference to an asynchronous writer.
/// * `msg` - A reference to the message to be sent.
///
/// # Returns
///
/// * `Result<(), MineChatError>` - Returns `Ok(())` if the message is sent successfully, otherwise
///   returns an error.
pub async fn send_message<W>(writer: &mut W, msg: &MineChatMessage) -> Result<(), MineChatError>
where
    W: AsyncWrite + Unpin,
{
    trace!("Serializing message {:?}", msg);
    let json = serde_json::to_string(msg)? + "\n";
    trace!("Sending message to server");
    writer.write_all(json.as_bytes()).await?;
    Ok(())
}

/// Receives a message from the server.
///
/// # Arguments
///
/// * `reader` - A mutable reference to an asynchronous reader.
///
/// # Returns
///
/// * `Result<MineChatMessage, MineChatError>` - Returns the received message if successful,
///   otherwise returns an error.
pub async fn receive_message<R>(reader: &mut R) -> Result<MineChatMessage, MineChatError>
where
    R: AsyncBufRead + Unpin,
{
    let mut line = String::new();
    reader.read_line(&mut line).await?;
    Ok(serde_json::from_str(&line)?)
}

/// Handles linking with the server.
///
/// # Arguments
///
/// * `server_addr` - The address of the server to connect to.
/// * `code` - The link code to authenticate with the server.
///
/// # Returns
/// * `Result<(String, String), MineChatError>` - Returns a tuple containing the client UUID and
///   server address if linking is successful, otherwise returns an error.
pub async fn link_with_server(
    server_addr: impl AsRef<str>,
    code: impl AsRef<str>,
) -> Result<(String, String), MineChatError> {
    let addr = server_addr.as_ref();
    let link_code = code.as_ref();

    handle_link(addr, link_code).await
}

/// Handles linking with the server. The same as link_with_server.
///
/// Deprecated, use link_with_server instead, as it has a more descriptive and self-explaining name.
#[deprecated(since = "0.1.1", note = "use link_with_server instead")]
pub async fn handle_link(server_addr: &str, code: &str) -> Result<(String, String), MineChatError> {
    let client_uuid = Uuid::new_v4().to_string();
    trace!("Connecting to server {}", server_addr);

    let mut stream = TcpStream::connect(server_addr).await?;
    let (reader, mut writer) = stream.split();

    trace!("Connected to server");

    let mut reader = BufReader::new(reader);

    trace!("Sending message to server {}", server_addr);
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
            trace!("Linked successfully: {}", payload.message);
            Ok((client_uuid, server_addr.to_string()))
        }
        _ => Err(MineChatError::AuthFailed("Unexpected response".into())),
    }
}
