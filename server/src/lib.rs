use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use auth::{auth_router, OAuth};
use axum::{extract::FromRef, routing::post, Router};
use graphql::{build_schema, graphiql, graphql_handler, AppSchema};
use reqwest::Client;
use sea_orm::DbConn;
use shuttle_runtime::SecretStore;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::timeout::TimeoutLayer;
mod auth;
use axum_extra::extract::cookie::Key;
mod error;
mod graphql;
mod jwt;
mod service;

#[derive(Clone)]
pub(crate) struct AppState {
    coon: DbConn,
    auth: auth::OAuth,
    req: Client,
    secret_store: Arc<SecretStore>,
    key: Key,
    schema: AppSchema,
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
            schema: build_schema(),
        }
    }
}

pub fn build_root_router(coon: DbConn, secret_store: SecretStore) -> Result<Router> {
    let client_id = secret_store.get("GITHUB_OAUTH_CLIENT_ID").unwrap();
    let client_secret = secret_store.get("GITHUB_OAUTH_CLIENT_SECRET").unwrap();
    let auth = OAuth::new(&client_id, &client_secret, "http://localhost:1420/login")?;
    let app_state = AppState::new(coon, auth, secret_store);
    // 静态路由
    let serve_dir = ServeDir::new("public").not_found_service(ServeFile::new("public/index.html"));

    let router = Router::new()
        .nest_service("/", serve_dir.clone())
        .route("/api/graphql", post(graphql_handler).get(graphiql))
        .nest("/api/auth", auth_router())
        .with_state(app_state.clone())
        .fallback_service(serve_dir)
        .layer(CompressionLayer::new().gzip(true))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid::default()))
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(CorsLayer::permissive());
    Ok(router)
}
