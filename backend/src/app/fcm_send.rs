mod fcm_client;

use std::collections::HashMap;

pub use fcm_client::FcmClient;

use fcm_client::{send, Message, WebpushConfig, WebpushConfigNotification};

pub async fn send_all(
    client: FcmClient,
    tokens: Vec<String>,
    message: String,
    url: String,
) -> anyhow::Result<()> {
    futures::future::join_all(
        tokens
            .into_iter()
            .map(|token| send(client.clone(), token, message.clone(), url.clone())),
    )
    .await;
    Ok(())
}

async fn send(client: FcmClient, token: String, body: String, url: String) -> anyhow::Result<()> {
    let _response = client
        .send(
            send::PathParameters {
                parent: format!("projects/{}", client.project_id()),
            },
            send::RequestBody {
                message: Message {
                    webpush: Some(WebpushConfig {
                        notification: Some(WebpushConfigNotification {
                            body: Some(body),
                            data: Some(
                                vec![("url".to_owned(), url)]
                                    .into_iter()
                                    .collect::<HashMap<String, String>>(),
                            ),
                            icon: Some("https://bouzuya.net/images/favicon.png".to_owned()),
                            require_interaction: Some(true),
                            title: Some("bouzuya.net からのお知らせ".to_owned()),
                            ..Default::default()
                        }),
                    }),
                    token: Some(token),
                    ..Default::default()
                },
            },
        )
        .await?;
    Ok(())
}
