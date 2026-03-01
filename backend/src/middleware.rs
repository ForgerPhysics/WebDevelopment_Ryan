use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

use crate::models::Claims;

pub async fn auth_guard(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = match auth_header {
        Some(value) if value.starts_with("Bearer ") => &value[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    let secret = std::env::var("JWT_SECRET").map_err(|_| StatusCode::UNAUTHORIZED)?;
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id = Uuid::parse_str(&data.claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;
    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
