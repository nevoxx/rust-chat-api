use crate::auth::TokenClaims;
use crate::models::User;
use crate::requests::{LoginRequest, RegisterRequest};
use crate::responses::{
    ChannelResource, ConnectionStateResource, ServerInfoResource, UserListResource,
};
use crate::{queries, AppState};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tracing::info;

pub async fn hello_handler() -> impl IntoResponse {
    "Hello, Rust! V2!"
}

pub async fn get_server_info(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let channels = queries::get_channels(data).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": format!("Error: {}", e),
            })),
        )
    })?;

    let channels = channels
        .iter()
        .map(|channel| channel.to_resource())
        .collect::<Vec<ChannelResource>>();

    let response = ServerInfoResource {
        id: "27551e8f-8e8d-4c54-b8e8-4c005a56076b".to_string(),
        name: "Testing Server".to_string(),
        description: "Lorem Ipsum ...".to_string(),
        createdAt: "2023-06-15 19:25:40".to_string(),
        updatedAt: "2023-06-15 19:25:40".to_string(),
        channels,
    };

    return Ok((StatusCode::OK, Json(json!(response))));
}

pub async fn get_channels_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let channels = queries::get_channels(data).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": format!("Error: {}", e),
            })),
        )
    })?;

    let channel_resources = channels
        .iter()
        .map(|channel| channel.to_resource())
        .collect::<Vec<ChannelResource>>();

    return Ok((StatusCode::OK, Json(json!(channel_resources))));
}

pub async fn get_channel_messages_handler(
    State(data): State<Arc<AppState>>,
    Path(channel_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let messages = queries::get_channel_messages(data.clone(), channel_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": format!("Error: {}", e),
                })),
            )
        })?;

    // Collect unique user IDs from messages

    let unique_user_ids: HashSet<String> = messages
        .iter()
        .map(|message| message.user_id.clone())
        .collect();

    let unique_user_ids_vec: Vec<String> = unique_user_ids.into_iter().collect();

    // Fetch only the needed users

    let users = queries::get_users(data.clone(), Some(&unique_user_ids_vec))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": format!("Failed to fetch users: {}", e) })),
            )
        })?;

    // Build a lookup map for users
    let user_map: HashMap<String, User> = users.into_iter().map(|u| (u.id.clone(), u)).collect();

    // Build message resources
    let message_resources = messages
        .into_iter()
        .map(|msg| {
            let user = user_map.get(&msg.user_id).map(|u| u.to_resource());

            return msg.to_resource(user.unwrap());
        })
        .collect::<Vec<_>>();

    Ok((StatusCode::OK, Json(json!(message_resources))))
}

#[axum_macros::debug_handler]
pub async fn post_auth_token_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = ?",
        body.username
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        let error_response = json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = json!({
            "status": "fail",
            "message": "Invalid username or password1",
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = json!({
            "status": "fail",
            "message": "Invalid username or password2"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.config.jwt_secret.as_ref()),
    )
    .unwrap();

    Ok((
        StatusCode::OK,
        Json(json!({"accessToken": token, "refreshToken": ""})),
    ))
}

pub async fn get_auth_me_handler(
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!(user.to_auth_me_resource())))
}

pub async fn post_register_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_exists: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE username = ?)")
            .bind(body.username.to_owned())
            .fetch_one(&data.db)
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

    if let Some(exists) = user_exists {
        if exists {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "User with that username already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error while hashing password: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    // let user =
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, username, display_name, password) VALUES (UUID(), ?, ?, ?);
        "#,
        body.username.to_string(),
        body.username.to_string(),
        hashed_password
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let user_response = serde_json::json!({"status": "success"});

    Ok(Json(user_response))
}

pub async fn get_users_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let users = queries::get_users(data.clone(), None).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": format!("Error: {}", e),
            })),
        )
    })?;

    let user_resources = users
        .iter()
        .map(|user| {
            let user_resource = user.to_resource();
            let connection_state = match data.connected_users.get(&user.id) {
                Some(connection) => ConnectionStateResource {
                    isOnline: true,
                    connectedAt: Some(connection.connected_at),
                    currentChannelId: connection.current_channel_id.clone(),
                    isAudioMuted: Some(connection.is_audio_muted),
                    isMicrophoneMuted: Some(connection.is_mic_muted),
                },
                None => ConnectionStateResource {
                    isOnline: false,
                    connectedAt: None,
                    currentChannelId: None,
                    isAudioMuted: None,
                    isMicrophoneMuted: None,
                },
            };

            UserListResource {
                user: user_resource,
                connectionState: connection_state,
            }
        })
        .collect::<Vec<UserListResource>>();

    Ok((StatusCode::OK, Json(json!(user_resources))))
}

pub async fn get_link_preview_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    return Ok(());
}
