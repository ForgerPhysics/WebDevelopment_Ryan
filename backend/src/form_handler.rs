use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreateFormRequest, FormResponse, UpdateFormRequest};

#[derive(sqlx::FromRow)]
struct FormRecord {
    id: Uuid,
    user_id: Uuid,
    title: String,
    description: String,
    created_at: sqlx::types::chrono::NaiveDateTime,
}

impl From<FormRecord> for FormResponse {
    fn from(record: FormRecord) -> Self {
        Self {
            id: record.id,
            user_id: record.user_id,
            title: record.title,
            description: record.description,
            created_at: record.created_at,
        }
    }
}

pub async fn create_form(
    State(pool): State<PgPool>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<CreateFormRequest>,
) -> impl IntoResponse {
    let record = sqlx::query_as::<_, FormRecord>(
        "INSERT INTO forms (id, user_id, title, description, created_at) VALUES ($1, $2, $3, $4, NOW()) RETURNING id, user_id, title, description, created_at",
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .fetch_one(&pool)
    .await;

    match record {
        Ok(record) => (
            StatusCode::CREATED,
            Json(FormResponse::from(record)),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_forms(State(pool): State<PgPool>) -> impl IntoResponse {
    let records = sqlx::query_as::<_, FormRecord>(
        "SELECT id, user_id, title, description, created_at FROM forms",
    )
    .fetch_all(&pool)
    .await;

    match records {
        Ok(records) => {
            let forms = records.into_iter().map(FormResponse::from).collect::<Vec<_>>();
            Json(forms).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_form(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let record = sqlx::query_as::<_, FormRecord>(
        "SELECT id, user_id, title, description, created_at FROM forms WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match record {
        Ok(Some(record)) => Json(FormResponse::from(record)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update_form(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<UpdateFormRequest>,
) -> impl IntoResponse {
    let record = sqlx::query_as::<_, FormRecord>(
        "UPDATE forms SET title = $1, description = $2 WHERE id = $3 AND user_id = $4 RETURNING id, user_id, title, description, created_at",
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(id)
    .bind(user_id)
    .fetch_optional(&pool)
    .await;

    match record {
        Ok(Some(record)) => Json(FormResponse::from(record)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_form(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Extension(user_id): Extension<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM forms WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(&pool)
        .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
