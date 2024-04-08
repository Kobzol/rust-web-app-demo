use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use std::path::Path;

#[derive(serde::Deserialize, Debug)]
pub struct AppConfig {
    db_url: Secret<String>,
    postmark_api_key: Secret<String>,
}

impl AppConfig {
    #[cfg(test)]
    pub fn new() -> Self {
        Self {
            db_url: String::new().into(),
            postmark_api_key: String::new().into(),
        }
    }

    pub fn db_url(&self) -> &str {
        self.db_url.expose_secret()
    }

    pub fn postmark_api_key(&self) -> &str {
        self.postmark_api_key.expose_secret()
    }
}

pub fn parse_app_config<P: AsRef<Path>>(path: P) -> anyhow::Result<AppConfig> {
    let path = path.as_ref();
    let data = std::fs::read_to_string(path)
        .with_context(|| format!("Cannot read app config from {}", path.display()))?;
    let config: AppConfig = toml::from_str(&data).context("Cannot deserialize app config")?;
    Ok(config)
}
