use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing, Router,
};

use crate::app::App;

#[derive(serde::Deserialize)]
struct CreateRequestBody {
    token: String,
}

#[derive(serde::Serialize)]
struct CreateResponseBody {
    id: String,
}

async fn create(
    State(app): State<App>,
    Json(CreateRequestBody { token }): Json<CreateRequestBody>,
) -> Result<Json<CreateResponseBody>, StatusCode> {
    let id = app
        .create_token(token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(CreateResponseBody { id }))
}

#[derive(serde::Deserialize)]
struct DeletePath {
    token_id: String,
}

async fn delete(
    State(app): State<App>,
    Path(DeletePath { token_id }): Path<DeletePath>,
) -> Result<StatusCode, StatusCode> {
    app.delete_token(token_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn route() -> Router<App> {
    Router::new()
        .route("/tokens", routing::post(create))
        .route("/tokens/{token_id}", routing::delete(delete))
}
