use std::{net::SocketAddr, sync::Arc};

use async_graphql::{Context, Result};
use diesel_async::AsyncPgConnection;
use jwt_simple::algorithms::HS512Key;
use tokio::sync::{Mutex, MutexGuard};

pub mod auth;
pub mod entities;
pub mod schema_db;
pub mod schema_graphql;
pub mod gql_errors;

pub type DbConnection = Arc<Mutex<AsyncPgConnection>>;

#[derive(Debug, Clone)]
pub struct GlobalConfig {
    pub database_url: String,
    pub invite_code: Option<String>,
    pub bind_address: SocketAddr,
    pub jwt_key: HS512Key,
}

pub(crate) async fn extract_connexion<'a>(
    ctx: &'a Context<'_>,
) -> Result<MutexGuard<'a, AsyncPgConnection>> {
    Ok(ctx.data::<DbConnection>()?.lock().await)
}
