use serde_json::Value;
use socketioxide::extract::{SocketRef, Data};
use tracing::info;

use crate::socket::listeners::{on_message, on_message_with_ack};

pub fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    
    socket.emit("auth", &data).ok();

    socket.on("message", on_message);
    socket.on("message-with-ack", on_message_with_ack);
}
