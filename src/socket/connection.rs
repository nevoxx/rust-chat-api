use std::sync::Arc;
use serde::Serialize;
use serde_json::Value;
use socketioxide::extract::{SocketRef, Data};
use tracing::{info, warn};
use uuid::Uuid;
use crate::AppState;
use crate::auth::{extract_user_id, parse_token};
use crate::models::User;
use crate::queries::get_user_by_id;

#[derive(Debug, Clone, Serialize)]
pub struct ConnectionInfo {
    pub token: String,
    pub user: User,
}

pub async fn on_connect(socket: SocketRef, Data(data): Data<Value>, app_state: Arc<AppState>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

    // Extract token from connection data
    let token = match data.get("token")
        .and_then(|token_val| token_val.as_str())
        .map(|s| s.to_string()) {
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
        return;
    }

    // Register event handlers
    register_event_handlers(&socket, app_state.clone());
}

async fn authenticate_socket(socket: &SocketRef, token: &str, app_state: Arc<AppState>) -> Result<(), String> {
    // Parse JWT token
    let claims = parse_token(token, app_state.config.jwt_secret.as_ref())
        .map_err(|e| format!("Failed to parse token: {}", e))?;

    // Extract user ID
    let user_id = extract_user_id(&claims)
        .map_err(|e| format!("Failed to extract user ID: {}", e))?;

    // Fetch user from database
    let user = get_user_by_id(app_state, user_id.to_string()).await
        .map_err(|e| format!("Failed to fetch user: {}", e))?;

    // Store connection info
    socket.extensions.insert(ConnectionInfo {
        token: token.to_string(),
        user,
    });

    Ok(())
}

fn register_event_handlers(socket: &SocketRef, app_state: Arc<AppState>) {
    let app_state_clone = app_state.clone();
    socket.on("sendChatMessage", move |socket: SocketRef, Data(msg): Data<Value>| {
        info!("~~ Cnt ~~ : {:?}", app_state_clone.cnt);

        {
            let mut cnt = app_state_clone.cnt.lock().unwrap();
            *cnt += 1;
        }

        if let Some(connection_info) = socket.extensions.get::<ConnectionInfo>() {
            info!("Message from user {}: {:?}", connection_info.user.id, msg);
        } else {
            warn!("Received message but no connection info found");
        }
    });
}
