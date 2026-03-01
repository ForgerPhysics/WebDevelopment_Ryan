use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{AuthResponse, Claims, LoginRequest, RegisterRequest};

#[derive(sqlx::FromRow)]
struct UserRecord {
    id: Uuid,
    password_hash: String,
}

pub async fn register_user(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let password_hash = match hash(&payload.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let user_id = Uuid::new_v4();
    let result = sqlx::query(
        "INSERT INTO users (id, username, password_hash, created_at) VALUES ($1, $2, $3, NOW())",
    )
    .bind(user_id)
    .bind(&payload.username)
    .bind(password_hash)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(err) => {
            if let sqlx::Error::Database(db_err) = &err {
                if db_err.code().as_deref() == Some("23505") {
                    return (StatusCode::CONFLICT, "username already exists").into_response();
                }
            }
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn login_user(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let record = sqlx::query_as::<_, UserRecord>(
        "SELECT id, password_hash FROM users WHERE username = $1",
    )
    .bind(&payload.username)
    .fetch_one(&pool)
    .await;

    let record = match record {
        Ok(record) => record,
        Err(sqlx::Error::RowNotFound) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let is_valid = match verify(&payload.password, &record.password_hash) {
        Ok(valid) => valid,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if !is_valid {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let secret = match std::env::var("JWT_SECRET") {
        Ok(secret) if !secret.is_empty() => secret,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let exp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as usize + 3600)
        .unwrap_or(0);

    let claims = Claims {
        sub: record.id.to_string(),
        exp,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(token) => token,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    (StatusCode::OK, Json(AuthResponse { token })).into_response()
}
