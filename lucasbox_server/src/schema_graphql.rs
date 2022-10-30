use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Result};
use diesel::{*};
use diesel_async::RunQueryDsl;

use crate::{
    entities::{Collection, CollectionItem, ItemFile, Tag},
    extract_connexion,
};

pub type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

#[derive(Default, Debug, Copy, Clone)]
pub struct Query;

#[Object]
impl Query {
    async fn collection(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Collection>> {
        use crate::schema_db::collections::dsl::collections;

        let mut conn = extract_connexion(ctx).await?;
        Ok(collections.find(id).first(&mut *conn).await.optional()?)
    }

    async fn collection_item(&self, ctx: &Context<'_>, id: i32) -> Result<Option<CollectionItem>> {
        use crate::schema_db::collection_items::dsl::collection_items;

        let mut conn = extract_connexion(ctx).await?;
        Ok(collection_items
            .find(id)
            .first(&mut *conn)
            .await
            .optional()?)
    }

    async fn item_file(&self, ctx: &Context<'_>, id: i32) -> Result<Option<ItemFile>> {
        use crate::schema_db::item_files::dsl::item_files;

        let mut conn = extract_connexion(ctx).await?;
        Ok(item_files.find(id).first(&mut *conn).await.optional()?)
    }

    async fn tag(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Tag>> {
        use crate::schema_db::tags::dsl::tags;

        let mut conn = extract_connexion(ctx).await?;
        Ok(tags.find(id).first(&mut *conn).await.optional()?)
    }

    async fn tags(&self, ctx: &Context<'_>) -> Result<Vec<Tag>> {
        use crate::schema_db::tags::dsl::tags;

        let mut conn = extract_connexion(ctx).await?;
        Ok(tags.load(&mut *conn).await?)
    }

    async fn root_collections(&self, ctx: &Context<'_>) -> Result<Vec<Collection>> {
        use crate::schema_db::collections::dsl::*;

        let mut conn = extract_connexion(ctx).await?;
        Ok(collections
            .filter(parent_id.is_null())
            .load::<Collection>(&mut *conn)
            .await?)
    }
}

pub type Mutation = EmptyMutation;

pub type Subscription = EmptySubscription;
