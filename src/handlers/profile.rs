use std::sync::Arc;

use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{Utc, SecondsFormat};
use reqwest::Client;

use crate::models::{ErrorResponse, ProfileResponse, User};
use crate::services::cat_service::{get_cat_fact_strict, CatApiError};

#[utoipa::path(
    get, path = "/me", tag = "Profile",
    responses(
        (status = 200, description = "Success", body = ProfileResponse),
        (status = 400, description = "Upstream bad request", body = ErrorResponse),
        (status = 401, description = "Upstream unauthorized", body = ErrorResponse),
        (status = 403, description = "Upstream forbidden", body = ErrorResponse),
        (status = 404, description = "Upstream not found", body = ErrorResponse),
        (status = 429, description = "Upstream rate limited", body = ErrorResponse),
        (status = 502, description = "Upstream failure", body = ErrorResponse),
        (status = 504, description = "Upstream timeout", body = ErrorResponse),
    )
)]
pub async fn me_handler(
    Extension(client): Extension<Arc<Client>>,
) -> Response {
    match get_cat_fact_strict(&client).await {
        Ok(fact) => {
            // ✅ Success payload (exact spec)
            let body = ProfileResponse {
                status: "success".into(),
                user: User {
                    email: "sammymenim@gmail.com".into(),
                    name: "Samuel Menim".into(),
                    stack: "Rust/Axum".into(),
                },
                timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
                fact,
            };
            (StatusCode::OK, Json(body)).into_response()
        }
        Err(err) => {
            // ❌ Failure payload (simple, uniform)
            let (status, message) = map_cat_error(err);
            let body = ErrorResponse {
                status: "failed".into(),
                message: message.into(),
                status_code: status.as_u16(),
            };
            (status, Json(body)).into_response()
        }
    }
}

/// Convert cat upstream errors into your friendly message + correct HTTP status.
fn map_cat_error(e: CatApiError) -> (StatusCode, &'static str) {
    use CatApiError::*;
    match e {
        Timeout     => (StatusCode::GATEWAY_TIMEOUT, "Cat oracle is taking too long ⏳"),
        Network     => (StatusCode::BAD_GATEWAY,     "Can’t reach the cat fact service 📡"),
        InvalidJson => (StatusCode::BAD_GATEWAY,     "Cat fact is garbled in translation 😼"),
        Unknown     => (StatusCode::BAD_GATEWAY,     "Cats are mysterious creatures 🐱"),
        Status(code) => {
            // Convert numeric upstream code -> axum StatusCode safely
            let status = StatusCode::from_u16(code).unwrap_or(StatusCode::BAD_GATEWAY);

            // Choose a friendly message by numeric values/range
            let msg = match code {
                400 => "Cat fact request was invalid 🐾",
                401 => "Cat facts need credentials 🐱‍👤",
                403 => "The cats won’t share facts 😿",
                404 => "This cat fact strayed too far 🐈‍⬛",
                429 => "Too many requests for cat wisdom 💤",
                500..=599 => "The cat oracle is sleepy (server error) 😴",
                _ => "Cats are mysterious creatures 🐱",
            };

            (status, msg)
        }
    }
}
