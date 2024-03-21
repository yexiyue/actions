mod oauth;
use axum::{routing::get, Router};
pub use oauth::OAuth;

use crate::AppState;
mod handler;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", get(handler::login))
        .route("/authorized", get(handler::authorized))
        .route("/refresh", get(handler::refresh))
}
