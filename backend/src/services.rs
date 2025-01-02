#[axum::async_trait]
pub trait CreateNotificationService {
    async fn create_test_notification(&self, token_id: String) -> anyhow::Result<()>;
}

#[axum::async_trait]
pub trait CreateTokenService {
    async fn create_token(&self, token: String) -> anyhow::Result<String>;
}

#[axum::async_trait]
pub trait DeleteTokenService {
    async fn delete_token(&self, token_id: String) -> anyhow::Result<()>;
}
