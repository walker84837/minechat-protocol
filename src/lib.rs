//! # MineChat Protocol
//!
//! This library implements my own MineChat protocol, which is designed to enable you to chat with a Minecraft server
//! from external clients, without going through the hassle of using platforms like Discord.
//! It provides asynchronous functions to send and receive messages, handle authentication, and
//! manage connections, although a synchronous API is going to be developed soon if you feel like
//! using one.
//!
//! ## Features
//!
//! - Asynchronous message sending and receiving via generic readers/writers.
//! - UUID generation for client identification for a more unique identification for your client.
//! - Right now there is just an async API to use packets, with `tokio`.
//! - This crate uses the `log` crate for logging for debugging and monitoring, so you can log what
//!   happens in your application.
//!
//! ## Modules
//!
//! - `protocol`: Defines the message types and payload structures used in the MineChat protocol.
//! - `packets`: Contains functions for sending and receiving messages, as well as handling server linking.

#[allow(dead_code)]
pub mod packets;
pub mod protocol;
