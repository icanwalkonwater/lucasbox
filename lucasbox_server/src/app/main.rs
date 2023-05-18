use std::{net::SocketAddr, sync::Arc};

use crate::cli::Cli;
use axum::{routing::get, Extension, Router};
use clap::Parser;
use color_eyre::Result;
use lucasbox_server::model::AppGraphQlSchema;
use sqlx::postgres::PgPoolOptions;
use tracing::{info, trace};

mod cli;
mod handlers;

#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();

  let cli: Cli = Cli::parse();

  info!("Connecting to database");
  let db = PgPoolOptions::new()
    .max_connections(3)
    .connect(&cli.database_url)
    .await?;
  let db = Arc::new(db);

  info!("Running migrations");
  sqlx::migrate!("db/migrations").run(db.as_ref()).await?;

  info!("Creating GraphQl schema");
  let graphql_schema =
    AppGraphQlSchema::build(Default::default(), Default::default(), Default::default())
      .data(Arc::clone(&db))
      .finish();
  trace!("GraphQl schema: {}", graphql_schema.sdl());

  info!("Creating app");
  let app = Router::new()
    .route(
      "/",
      get(handlers::graphql_playground).post(handlers::graphql_handler),
    )
    .layer(Extension(Arc::clone(&db)))
    .layer(Extension(graphql_schema));

  let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
  info!("Listening on {addr}");
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await?;

  Ok(())
}
