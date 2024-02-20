use axum::{routing::get, Router};
use sea_orm::{SqlxPostgresConnector};
use sqlx::PgPool;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let conn = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);

    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
