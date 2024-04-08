use crate::config::AppConfig;
use crate::model::Subscriber;
use crate::{get_active_subscribers, AppResult};
use askama::Template;
use axum::extract::State;
use axum::Json;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(askama::Template)]
#[template(path = "newsletter.html")]
struct NewsletterTemplate {
    name: String,
}

#[tracing::instrument]
pub async fn send_newsletter(
    State(pool): State<PgPool>,
    State(config): State<Arc<AppConfig>>,
) -> AppResult<Json<String>> {
    let subscribers = get_active_subscribers(pool).await?;

    let client = reqwest::Client::new();
    let futures = subscribers.into_iter().map(|s| {
        let Subscriber {
            name,
            email,
            expiration,
        } = s;
        let client = &client;
        let config = &config;
        async move {
            tracing::info!("Sending newsletter to {email}");
            let body = NewsletterTemplate { name }.render()?;
            tracing::info!("Newsletter content: {body}");
            // client
            //     .post("https://api.postmarkapp.com/email")
            //     .header("X-Postmark-Server-Token", config.postmark_api_key())
            //     .json(&serde_json::json!({
            //         "From": "jakub@berankovi.net",
            //         "To": email,
            //         "Subject": "Mail API test",
            //         "HtmlBody": body,
            //         "MessageStream": "outbound"
            //     }))
            //     .send()
            //     .await?
            //     .error_for_status()?;
            Ok::<_, anyhow::Error>(())
        }
    });
    let results: Vec<Result<_, _>> = futures_util::future::join_all(futures).await;
    let _results: Vec<()> = results.into_iter().collect::<Result<Vec<_>, _>>()?;

    Ok(Json("ok".to_string()))
}
