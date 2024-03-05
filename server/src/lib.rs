use std::sync::Arc;

use anyhow::Result;
use auth::{auth_router, OAuth};
use axum::{extract::FromRef, middleware, routing::get, Router};
use jwt::Claims;
use middlewares::claims::claims_middleware;
use reqwest::Client;
use sea_orm::DbConn;
use shuttle_secrets::SecretStore;
mod auth;
use axum_extra::extract::cookie::Key;
mod error;
mod graphql;
mod jwt;
mod middlewares;
mod service;

#[derive(Clone)]
pub(crate) struct AppState {
    coon: DbConn,
    auth: auth::OAuth,
    req: Client,
    secret_store: Arc<SecretStore>,
    key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}
impl FromRef<AppState> for Client {
    fn from_ref(state: &AppState) -> Self {
        state.req.clone()
    }
}

impl FromRef<AppState> for DbConn {
    fn from_ref(state: &AppState) -> Self {
        state.coon.clone()
    }
}

impl AppState {
    pub fn new(coon: DbConn, auth: auth::OAuth, secret_store: SecretStore) -> Self {
        Self {
            coon,
            auth,
            req: Client::new(),
            secret_store: Arc::new(secret_store),
            key: Key::generate(),
        }
    }
}

pub fn build_root_router(coon: DbConn, secret_store: SecretStore) -> Result<Router> {
    let client_id = secret_store.get("GITHUB_OAUTH_CLIENT_ID").unwrap();
    let client_secret = secret_store.get("GITHUB_OAUTH_CLIENT_SECRET").unwrap();
    let auth = OAuth::new(
        &client_id,
        &client_secret,
        "http://localhost:8000/api/auth/callback",
    )?;
    let app_state = AppState::new(coon, auth, secret_store);
    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/api/auth", auth_router())
        .with_state(app_state.clone())
        .layer(middleware::from_fn_with_state(app_state, claims_middleware));
    Ok(router)
}

pub async fn hello_world(claims: Claims) -> &'static str {
    tracing::info!("Hello, world! {:#?}", claims);
    "Hello, world!"
}
