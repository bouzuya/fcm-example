mod fcm_send;

use std::{collections::HashMap, sync::Arc};

use anyhow::Context as _;
use fcm_send::FcmClient;
use tokio::sync::Mutex;

use crate::services::{CreateNotificationService, CreateTokenService, DeleteTokenService};

#[derive(Clone, Debug)]
pub struct Token {
    pub created_at: u64,
    pub id: String,
    pub token: String,
}

#[derive(Clone)]
pub struct App {
    fcm_client: FcmClient,
    secret: String,
    tokens: Arc<Mutex<HashMap<String, Token>>>,
}

impl App {
    pub async fn new() -> anyhow::Result<Self> {
        let fcm_client = FcmClient::new().await?;
        let secret = std::env::var("ADMIN_SECRET").context("ADMIN_SECRET not found")?;
        Ok(Self {
            fcm_client,
            secret,
            tokens: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    #[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(self))]
    pub async fn create_notification(
        &self,
        token_ids: Vec<String>,
        message: String,
        url: String,
    ) -> anyhow::Result<()> {
        let tokens = self.tokens.lock().await;

        if message.is_empty() {
            anyhow::bail!("invalid message");
        }

        if !(url.starts_with("https://bouzuya.net") || url.starts_with("https://blog.bouzuya.net"))
        {
            anyhow::bail!("invalid url");
        }

        let tokens = token_ids
            .iter()
            .filter_map(|token_id| tokens.get(token_id).cloned())
            .collect::<Vec<Token>>();

        fcm_send::send_all(
            self.fcm_client.clone(),
            tokens
                .into_iter()
                .map(|token| token.token)
                .collect::<Vec<String>>(),
            message,
            url,
        )
        .await?;
        Ok(())
    }

    pub fn is_admin(&self, secret: &str) -> bool {
        self.secret == secret
    }

    #[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(self))]
    pub async fn list_tokens(&self) -> anyhow::Result<Vec<Token>> {
        let tokens = self.tokens.lock().await;
        Ok(tokens.values().cloned().collect::<Vec<Token>>())
    }
}

#[axum::async_trait]
impl CreateNotificationService for App {
    #[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(self))]
    async fn create_test_notification(&self, token_id: String) -> anyhow::Result<()> {
        self.create_notification(
            vec![token_id],
            "テスト通知です".to_owned(),
            "https://bouzuya.net/".to_owned(),
        )
        .await
    }
}

#[axum::async_trait]
impl CreateTokenService for App {
    #[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(self))]
    async fn create_token(&self, token: String) -> anyhow::Result<String> {
        let mut tokens = self.tokens.lock().await;

        let id = {
            let mut bytes = [0u8; 36];
            rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut bytes);
            hex::encode(bytes)
        };
        let created_at = std::time::SystemTime::duration_since(
            &std::time::SystemTime::now(),
            std::time::UNIX_EPOCH,
        )?
        .as_secs();
        tokens.entry(id.clone()).or_insert(Token {
            created_at,
            id: id.clone(),
            token,
        });
        Ok(id)
    }
}

#[axum::async_trait]
impl DeleteTokenService for App {
    #[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(self))]
    async fn delete_token(&self, token_id: String) -> anyhow::Result<()> {
        let mut tokens = self.tokens.lock().await;
        tokens.remove(&token_id);
        Ok(())
    }
}
