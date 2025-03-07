# MineChat protocol

[![Rust](https://github.com/walker84837/minechat-protocol/actions/workflows/rust.yml/badge.svg)](https://github.com/walker84837/minechat-protocol/actions/workflows/rust.yml)

MineChat is a Rust library designed to facilitate communication with a Minecraft chat server. It provides asynchronous functions to send and receive messages, handle authentication, and manage connections.

## Features

- Asynchronous message sending and receiving.
- Error handling with detailed error messages.
- UUID generation for client identification.
- Support for various message types (Auth, AuthAck, Chat, Broadcast, Disconnect).
- Integration with `tokio` for asynchronous I/O operations.
- Logging for debugging and monitoring.

## Usage

### Sending a Message

To send a message to the server, use the `send_message` function:

```rust
use minechat_protocol::{send_message, MineChatMessage};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let server_addr = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(server_addr).await.unwrap();
    let (mut reader, mut writer) = stream.split();

    let message = MineChatMessage::Chat {
        payload: ChatPayload {
            message: "Hello, server!".to_string(),
        },
    };

    if let Err(e) = send_message(&mut writer, &message).await {
        eprintln!("Failed to send message: {}", e);
    }
}
```

### Receiving a Message

To receive a message from the server, use the `receive_message` function:

```rust
use minechat_protocol::receive_message;
use tokio::net::TcpStream;
use tokio::io::BufReader;

#[tokio::main]
async fn main() {
    let server_addr = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(server_addr).await.unwrap();
    let (reader, _) = stream.split();
    let mut reader = BufReader::new(reader);

    match receive_message(&mut reader).await {
        Ok(message) => println!("Received message: {:?}", message),
        Err(e) => eprintln!("Failed to receive message: {}", e),
    }
}
```

### Handling Linking

To handle linking with the server, use the `handle_link` function:

```rust
use minechat_protocol::handle_link;

#[tokio::main]
async fn main() {
    let server_addr = "127.0.0.1:8080";
    let link_code = "your_link_code";

    match handle_link(server_addr, link_code).await {
        Ok((server, client_uuid)) => {
            println!("Linked successfully to server {} with client UUID {}", server, client_uuid);
        }
        Err(e) => eprintln!("Failed to link: {}", e),
    }
}
```

## License

This project is licensed under the MPL-2.0 License. See the [LICENSE](LICENSE) file for more details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

### Roadmap

- [ ] Expose a synchronous API.

## Contact

For any questions or support, please open an issue on the [GitHub repository](https://github.com/walker84837/minechat-protocol).
