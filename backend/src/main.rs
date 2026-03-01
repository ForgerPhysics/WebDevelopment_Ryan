use axum::{routing::{delete, get, post, put}, Router};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

mod auth;
mod form_handler;
mod middleware;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok(); 
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    let auth_router = Router::new()
        .route("/api/auth/register", post(auth::register_user))
        .route("/api/auth/login", post(auth::login_user));

    let public_forms = Router::new()
        .route("/api/forms", get(form_handler::get_forms))
        .route("/api/forms/:id", get(form_handler::get_form));

    let protected_forms = Router::new()
        .route("/api/forms", post(form_handler::create_form))
        .route("/api/forms/:id", put(form_handler::update_form))
        .route("/api/forms/:id", delete(form_handler::delete_form))
        .route_layer(axum::middleware::from_fn(middleware::auth_guard));

    let app = Router::new()
        .route("/health", get(health))
        .merge(auth_router)
        .merge(public_forms)
        .merge(protected_forms)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(pool);

    
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> &'static str {
    "OK"
}