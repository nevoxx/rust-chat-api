use crate::auth::{extract_user_id, parse_token};
use crate::models::User;
use crate::queries::get_user_by_id;
use crate::socket::events::socket_listen_events;
use crate::socket::handlers::{
    send_chat_message_handler, send_kick_handler, send_poke_handler,
    send_user_audio_mute_status_changed, send_user_is_typing_handler,
    send_user_microphone_status_changed,
};
use crate::{AppState, UserConnection};
use serde::Serialize;
use serde_json::Value;
use socketioxide::extract::{AckSender, Data, SocketRef};
use socketioxide::SocketIo;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct ConnectionInfo {
    pub token: String,
    pub user: User,
}

pub async fn on_connect(socket: SocketRef, Data(data): Data<Value>, app_state: Arc<AppState>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

    // Extract token from connection data
    let token = match data
        .get("token")
        .and_then(|token_val| token_val.as_str())
        .map(|s| s.to_string())
    {
        Some(token) => token,
        None => {
            warn!("No token provided in connection data");
            return;
        }
    };

    // Initial acknowledgment
    socket.emit("auth", &data).ok();

    // Authenticate and establish connection
    if let Err(e) = authenticate_socket(&socket, &token, app_state.clone()).await {
        warn!("Authentication failed: {}", e);
        let _ = socket.disconnect();
        return;
    }

    // Register event handlers
    register_event_handlers(&socket, app_state.clone());
}

async fn authenticate_socket(
    socket: &SocketRef,
    token: &str,
    app_state: Arc<AppState>,
) -> Result<(), String> {
    // Parse JWT token
    let claims = parse_token(token, app_state.config.jwt_secret.as_ref())
        .map_err(|e| format!("Failed to parse token: {}", e))?;

    // Extract user ID
    let user_id =
        extract_user_id(&claims).map_err(|e| format!("Failed to extract user ID: {}", e))?;

    // Fetch user from database
    let user = get_user_by_id(app_state.clone(), user_id.to_string())
        .await
        .map_err(|e| format!("Failed to fetch user: {}", e))?;

    // Store connection info
    socket.extensions.insert(ConnectionInfo {
        token: token.to_string(),
        user: user.clone(),
    });

    app_state.connected_users.insert(
        user.id.clone(),
        UserConnection {
            user: user.clone(),
            socket: socket.clone(),
            current_channel_id: None,
            connected_at: chrono::Utc::now(),
            is_audio_muted: false,
            is_mic_muted: false,
        },
    );

    Ok(())
}

fn register_event_handlers(socket: &SocketRef, app_state: Arc<AppState>) {
    let app_state_clone = app_state.clone();
    socket.on(
        socket_listen_events::SEND_CHAT_MESSAGE,
        |io: SocketIo, socket: SocketRef, Data(payload): Data<Value>| async move {
            send_chat_message_handler(&io, &socket, Data(payload), app_state_clone).await;
        },
    );

    let app_state_clone = app_state.clone();
    socket.on(
        socket_listen_events::SEND_POKE,
        |io: SocketIo, socket: SocketRef, Data(payload): Data<Value>, ack: AckSender| async move {
            send_poke_handler(&io, &socket, Data(payload), ack, app_state_clone).await;
        },
    );

    let app_state_clone = app_state.clone();
    socket.on(
        socket_listen_events::SEND_KICK,
        |io: SocketIo, socket: SocketRef, Data(payload): Data<Value>, ack: AckSender| async move {
            send_kick_handler(&io, &socket, Data(payload), ack, app_state_clone).await;
        },
    );

    let app_state_clone = app_state.clone();
    socket.on(
        socket_listen_events::SEND_USER_IS_TYPING,
        |io: SocketIo, socket: SocketRef, Data(payload): Data<Value>| async move {
            send_user_is_typing_handler(&io, &socket, Data(payload), app_state_clone).await;
        },
    );

    let app_state_clone = app_state.clone();
    socket.on(
        socket_listen_events::SEND_USER_MICROPHONE_STATUS_CHANGED,
        |io: SocketIo, socket: SocketRef, Data(payload): Data<Value>| async move {
            send_user_microphone_status_changed(&io, &socket, Data(payload), app_state_clone).await;
        },
    );

    let app_state_clone = app_state.clone();
    socket.on(
        socket_listen_events::SEND_USER_AUDIO_MUTE_STATUS_CHANGED,
        |io: SocketIo, socket: SocketRef, Data(payload): Data<Value>| async move {
            send_user_audio_mute_status_changed(&io, &socket, Data(payload), app_state_clone).await;
        },
    );
}
