mod admin_notifications;
mod admin_tokens;
mod assets;
mod notifications;
mod tokens;

use crate::app::App;

pub fn route() -> axum::Router<App> {
    axum::Router::new()
        .merge(admin_notifications::route())
        .merge(admin_tokens::route())
        .merge(assets::route())
        .merge(notifications::route())
        .merge(tokens::route())
}

#[cfg(test)]
mod tests {
    #[async_trait::async_trait]
    pub(crate) trait ResponseExt {
        async fn into_body_string(self) -> anyhow::Result<String>;
        async fn into_body_as_json<T: serde::de::DeserializeOwned>(self) -> anyhow::Result<T>;
    }

    #[async_trait::async_trait]
    impl ResponseExt for axum::http::Response<axum::body::Body> {
        async fn into_body_string(self) -> anyhow::Result<String> {
            let body = axum::body::to_bytes(self.into_body(), usize::MAX).await?;
            Ok(String::from_utf8(body.to_vec())?)
        }
        async fn into_body_as_json<T: serde::de::DeserializeOwned>(self) -> anyhow::Result<T> {
            Ok(serde_json::from_str(&self.into_body_string().await?)?)
        }
    }

    pub(crate) async fn send_request(
        app: axum::Router,
        request: axum::http::Request<axum::body::Body>,
    ) -> anyhow::Result<axum::response::Response<axum::body::Body>> {
        Ok(tower::ServiceExt::oneshot(app, request).await?)
    }
}
