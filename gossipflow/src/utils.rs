// util.rs

use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;
use crate::models::Message;

/// Send a UDP message (serialized as JSON)
pub async fn send_message(socket: &UdpSocket, message: &Message, addr: &str) -> std::io::Result<()> {
    let data = serde_json::to_vec(message)?;
    socket.send_to(&data, addr).await?;
    Ok(())
}

/// Return current system time in millis since UNIX_EPOCH
pub fn current_unix_timestamp() -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    now.as_millis() as u64
}
