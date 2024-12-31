use axum::{extract::State, http::StatusCode, routing, Json, Router};

use crate::{app::App, extractors::AdminAuth};

#[derive(serde::Deserialize)]
pub struct CreateRequestBody {
    message: String,
    token_ids: Vec<String>,
    url: String,
}

#[derive(serde::Serialize)]
pub struct CreateResponseBody {}

pub async fn create(
    _: AdminAuth,
    State(app): State<App>,
    Json(CreateRequestBody {
        message,
        token_ids,
        url,
    }): Json<CreateRequestBody>,
) -> Result<Json<CreateResponseBody>, StatusCode> {
    app.create_notification(token_ids, message, url)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(CreateResponseBody {}))
}

pub fn route() -> Router<App> {
    Router::new().route("/admin/notifications", routing::post(create))
}
