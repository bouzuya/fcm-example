use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing, Json, Router,
};

use crate::app::{App, Token};

pub struct AdminAuth;

impl FromRequestParts<App> for AdminAuth {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, app: &App) -> Result<Self, Self::Rejection> {
        let secret = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .map(|s| s.trim_start_matches("Bearer "))
            .ok_or_else(|| StatusCode::UNAUTHORIZED)?;
        if app.is_admin(secret) {
            Ok(Self)
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

#[derive(serde::Serialize)]
pub struct ListResponseBody {
    tokens: Vec<ListResponseBodyToken>,
}

#[derive(serde::Serialize)]
pub struct ListResponseBodyToken {
    created_at: u64,
    id: String,
}

pub async fn list(
    _: AdminAuth,
    State(app): State<App>,
) -> Result<Json<ListResponseBody>, StatusCode> {
    let tokens = app
        .list_tokens()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(
            |Token {
                 created_at,
                 id,
                 token: _,
             }| ListResponseBodyToken { created_at, id },
        )
        .collect::<Vec<ListResponseBodyToken>>();
    Ok(Json(ListResponseBody { tokens }))
}

pub fn route() -> Router<App> {
    Router::new().route("/admin/tokens", routing::get(list))
}
