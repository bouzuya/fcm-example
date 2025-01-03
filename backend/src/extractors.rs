use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use crate::app::App;

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
