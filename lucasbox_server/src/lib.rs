use std::net::SocketAddr;
use std::sync::Arc;

use async_graphql::{Context, Result};
use diesel_async::AsyncPgConnection;
use jwt_simple::algorithms::HS512Key;
use tokio::sync::{Mutex, MutexGuard};

pub mod entities;
pub mod schema_db;
pub mod schema_graphql;
pub mod auth;

pub type DbConnection = Arc<Mutex<AsyncPgConnection>>;

pub(crate) async fn extract_connexion<'a>(
    ctx: &'a Context<'_>,
) -> Result<MutexGuard<'a, AsyncPgConnection>> {
    Ok(ctx.data::<DbConnection>()?.lock().await)
}

#[derive(Debug, Clone)]
pub struct GlobalConfig {
    pub database_url: String,
    pub invite_code: Option<String>,
    pub bind_address: SocketAddr,
    pub jwt_key: HS512Key,
}
