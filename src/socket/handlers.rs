use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::extract::{SocketRef, Data};
use socketioxide::SocketIo;
use tracing::{info, warn};
use crate::AppState;
use crate::queries::create_message;
use crate::responses::MessageResource;
use crate::socket::connection::ConnectionInfo;
use crate::socket::events::socket_publish_events;

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

pub async fn send_chat_message_handler(io: &SocketIo, socket: &SocketRef, Data(msg): Data<Value>, app_state: Arc<AppState>) {
    let connection_info = match socket.extensions.get::<ConnectionInfo>() {
        Some(info) => info,
        None => {
            warn!("Received message but no connection info found");
            return;
        }
    };

    let user_id = connection_info.user.id.clone();
    let payload: SendMessagePayload = match serde_json::from_value(msg) {
        Ok(payload) => payload,
        Err(e) => {
            warn!("Failed to deserialize message payload: {}", e);
            return;
        }
    };

    info!("Creating message from user {}: {:?}", user_id, payload);

    let message = match create_message(
        app_state,
        user_id,
        payload.channel_id,
        payload.content,
    ).await {
        Ok(message) => message,
        Err(e) => {
            warn!("Failed to create message: {}", e);
            return;
        }
    };

    info!("Message saved: {:?}", message);

    if let Err(e) = io.emit(socket_publish_events::RECEIVE_CHAT_MESSAGE, &ReceiveChatMessagePayload {
        message: message.to_resource(connection_info.user.to_resource())
    }).await {
        warn!("Failed to emit message: {}", e);
    }
}


pub async fn send_poke_handler(io: &SocketIo, socket: &SocketRef, Data(msg): Data<Value>, app_state: Arc<AppState>) {

}


pub async fn send_kick_handler(io: &SocketIo, socket: &SocketRef, Data(msg): Data<Value>, app_state: Arc<AppState>) {

}


pub async fn send_user_is_typing_handler(io: &SocketIo, socket: &SocketRef, Data(msg): Data<Value>, app_state: Arc<AppState>) {

}


pub async fn send_user_microphone_status_changed(io: &SocketIo, socket: &SocketRef, Data(msg): Data<Value>, app_state: Arc<AppState>) {

}


pub async fn send_user_audio_mute_status_changed(io: &SocketIo, socket: &SocketRef, Data(msg): Data<Value>, app_state: Arc<AppState>) {

}



