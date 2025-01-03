#[async_trait::async_trait]
pub trait CreateNotificationService {
    async fn create_test_notification(&self, token_id: String) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait CreateTokenService: Send + Sync {
    async fn create_token(&self, token: String) -> anyhow::Result<String>;
}

#[async_trait::async_trait]
pub trait DeleteTokenService: Send + Sync {
    async fn delete_token(&self, token_id: String) -> anyhow::Result<()>;
}
