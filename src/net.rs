use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub enum NetEvent {
    Connected(SocketAddr),
    HandshakeComplete(String),
    MessageReceived(String),
    Error(String),
    Disconnected,
}
