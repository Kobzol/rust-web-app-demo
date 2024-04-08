#![allow(warnings, unused)]

use axum::routing::post;
use axum::Router;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

pub use config::parse_app_config;
pub use routes::subscribe::add_subscriber;

mod config;
mod routes;

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/subscriber", post(add_subscriber))
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}
