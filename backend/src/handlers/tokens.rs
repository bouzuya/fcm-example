use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing, Router,
};

use crate::services::{CreateTokenService, DeleteTokenService};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct CreateRequestBody {
    token: String,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct CreateResponseBody {
    id: String,
}

async fn create<T: CreateTokenService>(
    State(app): State<T>,
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

async fn delete<T: DeleteTokenService>(
    State(app): State<T>,
    Path(DeletePath { token_id }): Path<DeletePath>,
) -> Result<StatusCode, StatusCode> {
    app.delete_token(token_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn route<T: Clone + CreateTokenService + DeleteTokenService + Send + Sync + 'static>(
) -> Router<T> {
    Router::new()
        .route("/tokens", routing::post(create::<T>))
        .route("/tokens/:token_id", routing::delete(delete::<T>))
}

#[cfg(test)]
mod tests {
    use crate::handlers::tests::{send_request, ResponseExt as _};

    use super::*;

    #[derive(Clone)]
    struct MockApp;

    #[axum::async_trait]
    impl CreateTokenService for MockApp {
        async fn create_token(&self, _token: String) -> anyhow::Result<String> {
            Ok("id34567890".to_owned())
        }
    }

    #[axum::async_trait]
    impl DeleteTokenService for MockApp {
        async fn delete_token(&self, _token_id: String) -> anyhow::Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_create() -> anyhow::Result<()> {
        let routes = route().with_state(MockApp);
        let request = axum::http::Request::builder()
            .method(axum::http::Method::POST)
            .uri("/tokens")
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(serde_json::to_vec(
                &CreateRequestBody {
                    token: "abcd1234".to_owned(),
                },
            )?))?;
        let response = send_request(routes, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(
            response.into_body_as_json::<CreateResponseBody>().await?,
            CreateResponseBody {
                id: "id34567890".to_owned()
            }
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> anyhow::Result<()> {
        let routes = route().with_state(MockApp);
        let request = axum::http::Request::builder()
            .method(axum::http::Method::DELETE)
            .uri("/tokens/id34567890")
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::empty())?;
        let response = send_request(routes, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::NO_CONTENT);
        Ok(())
    }
}
