use tower_http::services::{ServeDir, ServeFile};

use crate::app::App;

pub fn route() -> axum::Router<App> {
    axum::Router::new()
        .nest_service(
            "/firebase-messaging-sw.js",
            ServeFile::new("public/firebase-messaging-sw.js"),
        )
        .nest_service("/assets", ServeDir::new("public/assets"))
        .nest_service("/", ServeFile::new("public/index.html"))
}
