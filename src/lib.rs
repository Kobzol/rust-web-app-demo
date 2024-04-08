#![allow(warnings, unused)]

use axum::body::Body;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use chrono::{TimeZone, Utc};
use http::StatusCode;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

use crate::model::{Subscriber, SubscriptionExpiration};
use crate::routes::index::index_page;
pub use config::parse_app_config;
pub use routes::subscribe::add_subscriber;

mod config;
mod model;
mod routes;

type AppResult<T> = Result<T, AppError>;

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(format!("{:?}", self.0)))
            .unwrap()
            .into_response()
    }
}

impl<T> From<T> for AppError
where
    T: Into<anyhow::Error>,
{
    fn from(value: T) -> Self {
        let error = value.into();
        Self(error)
    }
}

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(index_page))
        .route("/subscriber", post(add_subscriber))
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}

/// Returns all active subscribers from the database.
async fn get_active_subscribers(pool: PgPool) -> anyhow::Result<impl Iterator<Item = Subscriber>> {
    let subscribers = sqlx::query!("SELECT name, email, expire_at FROM subscriber")
        .fetch_all(&pool)
        .await?;
    let subscribers = subscribers
        .into_iter()
        .map(|s| Subscriber {
            name: s.name,
            email: s.email,
            expiration: match s.expire_at {
                Some(date) => SubscriptionExpiration::At {
                    date: Utc.from_utc_datetime(&date),
                },
                None => SubscriptionExpiration::Never,
            },
        })
        .filter(|s| match s.expiration {
            SubscriptionExpiration::Never => true,
            SubscriptionExpiration::At { date } => date <= Utc::now(),
        });
    Ok(subscribers)
}
