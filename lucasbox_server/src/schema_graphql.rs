use crate::get_connection;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Result};
use diesel::*;
use diesel_async::RunQueryDsl;

use crate::entities::Collection;

pub type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

#[derive(Default, Debug, Copy, Clone)]
pub struct Query;

#[Object]
impl Query {
    async fn collection(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Collection>> {
        use crate::schema_db::collections::dsl::collections;

        let mut conn = get_connection(ctx).await?;
        Ok(collections.find(id).first(&mut *conn).await.optional()?)
    }

    async fn root_collections(&self, ctx: &Context<'_>) -> Result<Vec<Collection>> {
        use crate::schema_db::collections::dsl::*;

        let mut conn = get_connection(ctx).await?;
        Ok(collections
            .filter(parent_id.is_null())
            .load::<Collection>(&mut *conn)
            .await?)
    }
}

pub type Mutation = EmptyMutation;

pub type Subscription = EmptySubscription;
