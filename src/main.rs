#![allow(warnings, unused)]

use anyhow::Context;
use axum::{routing::get, Json, Router};
use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    /// Port on which the app should listen for TCP/IP connections.
    #[arg(long, env = "PORT", default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let port = args.port;

    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .context("cannot create TCP/IP server")?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handler() -> Json<String> {
    Json("foo".to_string())
}
