mod admin_notifications;
mod admin_tokens;
mod assets;
mod tokens;

use crate::app::App;

pub fn route() -> axum::Router<App> {
    axum::Router::new()
        .merge(admin_notifications::route())
        .merge(admin_tokens::route())
        .merge(assets::route())
        .merge(tokens::route())
}
