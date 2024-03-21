use migration::{Migrator, MigratorTrait};
use sea_orm::SqlxPostgresConnector;
use shuttle_runtime::SecretStore;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let coon = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
    Migrator::up(&coon, None).await.unwrap();
    let router = actions::build_root_router(coon, secrets)?;
    Ok(router.into())
}
