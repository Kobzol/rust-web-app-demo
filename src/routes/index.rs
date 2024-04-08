use askama::Template;
use axum::extract::State;
use axum::response::Html;
use sqlx::PgPool;

use crate::model::Subscriber;
use crate::{get_active_subscribers, AppResult};

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    subscribers: Vec<Subscriber>,
}

mod filters {
    use gravatar::{Gravatar, Rating};

    pub fn gravatar_url<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let s = s.to_string();
        let url = Gravatar::new(&s)
            .set_rating(Some(Rating::Pg))
            .set_size(Some(64))
            .image_url()
            .to_string();
        Ok(url)
    }
}

#[tracing::instrument]
pub async fn index_page(State(pool): State<PgPool>) -> AppResult<Html<String>> {
    let subscribers = get_active_subscribers(pool).await?;

    Ok(Html(
        IndexTemplate {
            subscribers: subscribers.collect(),
        }
        .render()?,
    ))
}
