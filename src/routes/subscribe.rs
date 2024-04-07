use axum::Json;
use axum_valid::Valid;
use chrono::Utc;

#[derive(serde::Deserialize, Debug)]
enum SubscriptionExpiration {
    Never,
    At { date: chrono::DateTime<Utc> },
}

#[derive(serde::Deserialize, validator::Validate, Debug)]
pub struct SubscriberParams {
    #[validate(length(min = 1))]
    name: String,
    #[validate(email)]
    email: String,
    expiration: SubscriptionExpiration,
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn add_subscriber(Valid(Json(params)): Valid<Json<SubscriberParams>>) -> Json<String> {
    tracing::info!("Adding subscriber");
    Json("ok".to_string())
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use http::header::CONTENT_TYPE;
    use http::{Method, Request, StatusCode};
    use tower::util::ServiceExt;

    use crate::create_app;

    use super::*;

    #[tokio::test]
    async fn wrong_email() -> anyhow::Result<()> {
        let app = create_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/subscriber")
                    .method(Method::POST)
                    .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        r#"
{"name": "", "email": "", "expiration": "Never"}
"#,
                    ))?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[test]
    fn deserialize_subscriber() {
        let params: SubscriberParams = serde_json::from_str(
            r#"
{"name": "Foo", "email": "foo@bar.com", "expiration": "Never"}
"#,
        )
        .unwrap();
        insta::assert_debug_snapshot!(params, @r###"
        SubscriberParams {
            name: "Foo",
            email: "foo@bar.com",
            expiration: Never,
        }
        "###);
    }
}
