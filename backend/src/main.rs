use std::net::Ipv4Addr;
use std::str::FromStr as _;

use anyhow::Context as _;
use app::App;

mod app;
mod extractors;
mod handlers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let state = App::new().await?;
    let router = axum::Router::new()
        .nest("/lab/fcm/", handlers::route())
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http());
    let ip_addr = Ipv4Addr::from_str("0.0.0.0").expect("0.0.0.0 to be valid ipv4addr");
    let port = u16::from_str(
        std::env::var_os("PORT")
            .unwrap_or_else(|| std::ffi::OsString::from("3000"))
            .to_str()
            .context("PORT is not UTF-8")?,
    )
    .context("PORT is invalid")?;
    let listener = tokio::net::TcpListener::bind((ip_addr, port)).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
