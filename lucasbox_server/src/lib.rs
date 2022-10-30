use std::sync::Arc;

use async_graphql::{Context, Result};
use diesel_async::AsyncPgConnection;
use tokio::sync::{Mutex, MutexGuard};

pub mod entities;
pub mod schema_db;
pub mod schema_graphql;

pub type DbConnection = Arc<Mutex<AsyncPgConnection>>;

pub(crate) async fn get_connection<'a>(
    ctx: &'a Context<'_>,
) -> Result<MutexGuard<'a, AsyncPgConnection>> {
    Ok(ctx.data::<DbConnection>()?.lock().await)
}
