use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing, Router,
};

use crate::services::CreateNotificationService;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct PathParameters {
    token_id: String,
}

async fn create<T: CreateNotificationService>(
    State(app): State<T>,
    Path(PathParameters { token_id }): Path<PathParameters>,
) -> Result<StatusCode, StatusCode> {
    app.create_test_notification(token_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn route<T: Clone + CreateNotificationService + Send + Sync + 'static>() -> Router<T> {
    Router::new().route(
        "/tokens/{token_id}/notifications",
        routing::post(create::<T>),
    )
}

#[cfg(test)]
mod tests {
    use crate::handlers::tests::send_request;

    use super::*;

    #[derive(Clone)]
    struct MockApp;

    #[async_trait::async_trait]
    impl CreateNotificationService for MockApp {
        async fn create_test_notification(&self, _token_id: String) -> anyhow::Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_create() -> anyhow::Result<()> {
        let routes = route().with_state(MockApp);
        let request = axum::http::Request::builder()
            .method(axum::http::Method::POST)
            .uri("/tokens/id34567890/notifications")
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::empty())?;
        let response = send_request(routes, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::NO_CONTENT);
        Ok(())
    }
}
