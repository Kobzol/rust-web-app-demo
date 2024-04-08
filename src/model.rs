use chrono::Utc;

#[derive(serde::Deserialize, Debug)]
pub enum SubscriptionExpiration {
    Never,
    At { date: chrono::DateTime<Utc> },
}

pub struct Subscriber {
    pub name: String,
    pub email: String,
    pub expiration: SubscriptionExpiration,
}
