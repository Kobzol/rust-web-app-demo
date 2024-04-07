#![allow(warnings, unused)]

use anyhow::Context;
use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    /// Port on which the app should listen for TCP/IP connections.
    #[arg(long, env = "PORT", default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let port = args.port;

    let app = rust_web_app_demo::create_app();

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .context("cannot create TCP/IP server")?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
