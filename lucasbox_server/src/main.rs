use std::net::SocketAddr;

use axum::{routing::get, Router};
use color_eyre::Result;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<()> {
  tracing::subscriber::set_global_default(
    tracing_subscriber::FmtSubscriber::builder()
      .with_max_level(Level::DEBUG)
      .finish(),
  )?;

  let app = Router::new().route("/", get(|| async { "Hello, World" }));

  let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
  info!("Serving on {addr}");
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await?;

  Ok(())
}
