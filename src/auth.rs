use std::sync::Arc;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::{header, StatusCode};
use axum::Json;
use axum::middleware::Next;
use axum::response::IntoResponse;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::models::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn auth(
    State(data): State<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = request.headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        })
        .ok_or_else(|| {
            let json_error = ErrorResponse {
                status: "fail",
                message: "You are not logged in, please provide token.".to_string(),
            };
            return (StatusCode::UNAUTHORIZED, Json(json_error));
        })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.config.jwt_secret.as_ref()),
        &Validation::default(),
    )
        .map_err(|_| {
            let json_error = ErrorResponse {
                status: "fail",
                message: "Invalid token".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(json_error))
        })?
        .claims;

    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let query = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", user_id.to_string());

    let user = match query.fetch_optional(&data.db).await {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Error executing query: {}", e);
            let json_error = ErrorResponse {
                status: "fail",
                message: format!("Error fetching user from database: {}", e),
            };
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json_error)));
        }
    };

    let user = user.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "The user belonging to this token no longer exists".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
