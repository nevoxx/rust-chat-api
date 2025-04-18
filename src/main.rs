mod auth;
mod config;
mod handlers;
mod models;
mod queries;
mod requests;
mod responses;
mod services;
mod socket;

use crate::auth::auth;
use crate::config::Config;
use crate::handlers::{
    get_auth_me_handler, get_channel_messages_handler, get_channels_handler,
    get_link_preview_handler, get_server_info, get_users_handler, hello_handler,
    post_auth_token_handler, post_register_user_handler,
};
use crate::models::User;
use crate::socket::connection::on_connect;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, PasswordHash, rand_core::OsRng};
use axum::{middleware, routing::get, routing::post, Router};
use dotenv::dotenv;
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug)]
pub struct UserConnection {
    pub user: User,
    pub socket: SocketRef,
    pub current_channel_id: Option<String>,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub is_audio_muted: bool,
    pub is_mic_muted: bool,
}

pub struct AppState {
    db: MySqlPool,
    config: Config,
    cnt: Mutex<i32>,
    connected_users: dashmap::DashMap<String, UserConnection>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init Tracing
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    // Load .env file
    dotenv().ok();

    // init config
    let config = Config::init();

    // Connect to the database
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            info!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            error!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };






    // The password to hash
    let password = "123456";

    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Create Argon2 hasher
    let argon2 = Argon2::default();

    // Hash the password
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .expect("failed to hash password")
        .to_string();

    // Print the resulting hash
    println!("Argon2 hash: {}", password_hash);




    // CORS
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        // .allow_headers(Any)
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
        ]);

    // App State
    let app_state = Arc::new(AppState {
        db: pool.clone(),
        config: config.clone(),
        cnt: Mutex::from(0),
        connected_users: dashmap::DashMap::new(),
    });

    // Socket.io Server
    let (socket_layer, io) = SocketIo::builder().req_path("/server/").build_layer();

    // Create a closure that captures the app state
    let state_clone = app_state.clone();
    io.ns("/", move |socket: SocketRef, Data(data): Data<Value>| {
        on_connect(socket, Data(data), state_clone)
    });

    // Create Axum app
    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/auth/register", post(post_register_user_handler))
        .route("/auth/token", post(post_auth_token_handler))
        .merge(
            Router::new()
                .route("/serverinfo", get(get_server_info))
                .route("/channels", get(get_channels_handler))
                .route(
                    "/channels/{channel_id}/messages/",
                    get(get_channel_messages_handler),
                )
                .route("/users", get(get_users_handler))
                .route("/auth/me", get(get_auth_me_handler))
                .route("/fetch-preview-data/", get(get_link_preview_handler))
                .layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .with_state(app_state)
        .layer(socket_layer)
        .layer(cors_layer);

    // Create Listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("🚀 Server started successfully: http://localhost:3000");

    // Run Axum Webserver
    axum::serve(listener, app).await.unwrap();

    // Exit gracefully
    return Ok(());
}
