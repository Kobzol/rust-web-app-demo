use crate::model::SubscriptionExpiration;
use crate::AppResult;
use axum::extract::State;
use axum::Json;
use axum_valid::Valid;
use sqlx::PgPool;

#[derive(serde::Deserialize, validator::Validate, Debug)]
pub struct SubscriberParams {
    #[validate(length(min = 1))]
    name: String,
    #[validate(email)]
    email: String,
    expiration: SubscriptionExpiration,
}

#[tracing::instrument]
pub async fn add_subscriber(
    State(pool): State<PgPool>,
    Valid(Json(params)): Valid<Json<SubscriberParams>>,
) -> AppResult<Json<String>> {
    tracing::info!("Adding subscriber");

    sqlx::query!(
        r#"
INSERT INTO subscriber(name, email, expire_at)
VALUES ($1, $2, $3)
"#,
        params.name,
        params.email,
        match params.expiration {
            SubscriptionExpiration::Never => None,
            SubscriptionExpiration::At { date } => Some(date.naive_utc()),
        }
    )
    .execute(&pool)
    .await?;

    Ok(Json("ok".to_string()))
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use http::header::CONTENT_TYPE;
    use http::{Method, Request, StatusCode};
    use tower::util::ServiceExt;

    use crate::create_app;

    use super::*;

    #[sqlx::test]
    async fn wrong_email(pool: PgPool) -> anyhow::Result<()> {
        let app = create_app(pool);
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

    #[sqlx::test]
    async fn insert_subscriber(pool: PgPool) -> anyhow::Result<()> {
        let app = create_app(pool.clone());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/subscriber")
                    .method(Method::POST)
                    .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        r#"
{"name": "Foo bar", "email": "foo@bar.com", "expiration": "Never"}
"#,
                    ))?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        let data = sqlx::query!("SELECT name, email, expire_at FROM subscriber")
            .fetch_one(&pool)
            .await?;
        insta::assert_debug_snapshot!(data, @r###"
        Record {
            name: "Foo bar",
            email: "foo@bar.com",
            expire_at: None,
        }
        "###);

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
