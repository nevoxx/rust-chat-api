use std::sync::Arc;
use serde::Deserialize;
use serde_json::Value;
use socketioxide::extract::{SocketRef, Data};
use tracing::{info, warn};
use crate::AppState;
use crate::queries::create_message;
use crate::socket::connection::ConnectionInfo;

#[derive(Debug, Deserialize)]
struct SendMessagePayload {
    #[serde(rename = "channelId")]
    channel_id: String,

    content: Option<String>,

    #[serde(rename = "attachmentIds")]
    attachment_ids: Vec<String>, // optional, but include it if it's in the payload
}

// info!("~~ Cnt ~~ : {:?}", app_state.cnt);
//
// {
//     let mut cnt = app_state.cnt.lock().unwrap();
//     *cnt += 1;
// }
//

pub async fn send_chat_message(socket: &SocketRef, Data(msg): Data<Value>, app_state: Arc<AppState>) {
    if let Some(connection_info) = socket.extensions.get::<ConnectionInfo>() {
        let user_id = connection_info.user.id.clone();
        let payload_result: Result<SendMessagePayload, _> = serde_json::from_value(msg);

        match payload_result {
            Ok(payload) => {
                info!("Creating message from user {}: {:?}", user_id, payload);

                // Create the message using your DB logic
                match create_message(
                    app_state,
                    user_id.clone(),
                    payload.channel_id,
                    payload.content,
                ).await {
                    Ok(message) => {
                        info!("Message saved: {:?}", message);

                        // todo: send response
                    }
                    Err(e) => {
                        warn!("Failed to create message: {}", e);
                    }
                }
            }
            Err(e) => {
                warn!("Failed to deserialize message payload: {}", e);
            }
        }
    } else {
        warn!("Received message but no connection info found");
    }
}
