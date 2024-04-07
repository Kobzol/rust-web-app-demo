#![allow(warnings, unused)]

mod routes;

use axum::routing::post;
use axum::Router;
pub use routes::subscribe::add_subscriber;
use tower_http::trace::TraceLayer;

pub fn create_app() -> Router {
    Router::new()
        .route("/subscriber", post(add_subscriber))
        .layer(TraceLayer::new_for_http())
}
