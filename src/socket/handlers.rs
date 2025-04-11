use crate::queries::create_message;
use crate::responses::MessageResource;
use crate::socket::connection::ConnectionInfo;
use crate::socket::events::socket_publish_events;
use crate::AppState;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use socketioxide::extract::{AckSender, Data, SocketRef};
use socketioxide::SocketIo;
use std::sync::Arc;
use tracing::{info, warn};

#[derive(Debug, Deserialize)]
struct SendMessagePayload {
    #[serde(rename = "channelId")]
    channel_id: String,

    content: Option<String>,

    #[serde(rename = "attachmentIds")]
    attachment_ids: Vec<String>, // optional, but include it if it's in the payload
}

#[derive(Debug, Serialize)]
pub struct ReceiveChatMessagePayload {
    message: MessageResource,
}

// info!("~~ Cnt ~~ : {:?}", app_state.cnt);
//
// {
//     let mut cnt = app_state.cnt.lock().unwrap();
//     *cnt += 1;
// }
//

pub async fn send_chat_message_handler(
    io: &SocketIo,
    socket: &SocketRef,
    Data(payload): Data<Value>,
    app_state: Arc<AppState>,
) {
    let connection_info = match socket.extensions.get::<ConnectionInfo>() {
        Some(info) => info,
        None => {
            warn!("Received message but no connection info found");
            return;
        }
    };

    let user_id = connection_info.user.id.clone();
    let payload: SendMessagePayload = match serde_json::from_value(payload) {
        Ok(payload) => payload,
        Err(e) => {
            warn!("Failed to deserialize message payload: {}", e);
            return;
        }
    };

    info!("Creating message from user {}: {:?}", user_id, payload);

    let message =
        match create_message(app_state, user_id, payload.channel_id, payload.content).await {
            Ok(message) => message,
            Err(e) => {
                warn!("Failed to create message: {}", e);
                return;
            }
        };

    info!("Message saved: {:?}", message);

    if let Err(e) = io
        .emit(
            socket_publish_events::RECEIVE_CHAT_MESSAGE,
            &ReceiveChatMessagePayload {
                message: message.to_resource(connection_info.user.to_resource()),
            },
        )
        .await
    {
        warn!("Failed to emit message: {}", e);
    }
}

pub async fn send_poke_handler(
    io: &SocketIo,
    socket: &SocketRef,
    Data(payload): Data<Value>,
    ack: AckSender,
    app_state: Arc<AppState>,
) {
    let connection_info = match socket.extensions.get::<ConnectionInfo>() {
        Some(info) => info,
        None => {
            warn!("Received message but no connection info found");
            return;
        }
    };

    // Extract receiver user ID from payload
    let receiver_user_id = match payload.get("userId").and_then(|id| id.as_str()) {
        Some(id) => id,
        None => {
            let _ = ack.send(&json!({
                "success": false,
                "error": "No userId provided in payload"
            }));
            return;
        }
    };

    let message = payload
        .get("message")
        .and_then(|m| m.as_str())
        .unwrap_or("");

    let created_at = payload
        .get("createdAt")
        .and_then(|dt| dt.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

    // Check if receiver exists and is connected
    if let Some(connection) = app_state.connected_users.get(receiver_user_id) {
        // Prepare the payload
        let poke_payload = json!({
            "user": connection_info.user.to_resource(),
            "message": message,
            "createdAt": created_at
        });

        // Emit event to the specific receiver
        connection
            .socket
            .emit(socket_publish_events::RECEIVE_POKE, &poke_payload)
            .ok();

        // Send success callback
        let _ = ack.send(&json!({ "success": true }));
    } else {
        // User not connected
        let _ = ack.send(&json!({
            "success": false,
            "error": format!("User with ID {} is currently not connected!", receiver_user_id)
        }));
    }
}

pub async fn send_kick_handler(
    io: &SocketIo,
    socket: &SocketRef,
    Data(payload): Data<Value>,
    app_state: Arc<AppState>,
) {
}

pub async fn send_user_is_typing_handler(
    io: &SocketIo,
    socket: &SocketRef,
    Data(payload): Data<Value>,
    app_state: Arc<AppState>,
) {
}

pub async fn send_user_microphone_status_changed(
    io: &SocketIo,
    socket: &SocketRef,
    Data(payload): Data<Value>,
    app_state: Arc<AppState>,
) {
}

pub async fn send_user_audio_mute_status_changed(
    io: &SocketIo,
    socket: &SocketRef,
    Data(payload): Data<Value>,
    app_state: Arc<AppState>,
) {
}
