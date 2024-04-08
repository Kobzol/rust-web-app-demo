use rust_web_app_demo::parse_app_config;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let config = parse_app_config("app-config.toml")?;
    tracing::info!("App config: {config:?}");

    sqlx::query("DROP TABLE _sqlx_migrations;")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DROP TABLE subscriber;")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Cannot run migrations");

    let app = rust_web_app_demo::create_app(pool, config);
    Ok(app.into())
}
