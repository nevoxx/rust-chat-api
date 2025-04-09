use std::sync::Arc;
use serde_json::Value;
use socketioxide::extract::{SocketRef, Data};
use tracing::{info, warn};
use crate::AppState;
use crate::socket::connection::ConnectionInfo;

pub fn send_chat_message(socket: &SocketRef, Data(msg): Data<Value>, app_state: Arc<AppState>) {
    info!("~~ Cnt ~~ : {:?}", app_state.cnt);

    {
        let mut cnt = app_state.cnt.lock().unwrap();
        *cnt += 1;
    }

    if let Some(connection_info) = socket.extensions.get::<ConnectionInfo>() {
        info!("Message from user {}: {:?}", connection_info.user.id, msg);
    } else {
        warn!("Received message but no connection info found");
    }
}
