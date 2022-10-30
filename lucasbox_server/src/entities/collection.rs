use std::time::SystemTime;

use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use diesel::*;
use diesel_async::RunQueryDsl;

use crate::{
    entities::{CollectionItem, Tag},
    get_connection,
    schema_db::collections,
};

#[derive(Clone, Debug, SimpleObject, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Self, foreign_key = parent_id))]
#[graphql(complex)]
pub struct Collection {
    pub id: i32,
    #[graphql(skip)]
    pub parent_id: Option<i32>,
    pub inline: bool,
    pub level: i16,
    pub name: String,
    pub description: Option<String>,
    #[graphql(skip)]
    pub updated_at: SystemTime,
    #[graphql(skip)]
    pub created_at: SystemTime,
}

#[ComplexObject]
impl Collection {
    async fn tags(&self, ctx: &Context<'_>) -> Result<Vec<Tag>> {
        use crate::schema_db::{tag_collections, tags};

        let mut conn = get_connection(ctx).await?;
        Ok(tag_collections::table
            .inner_join(tags::table)
            .filter(tag_collections::collection_id.eq(self.id))
            .select(tags::all_columns)
            .load::<Tag>(&mut *conn)
            .await?)
    }

    async fn children(&self, ctx: &Context<'_>) -> Result<Vec<Collection>> {
        let mut conn = get_connection(ctx).await?;
        Ok(Collection::belonging_to(self)
            .load::<Collection>(&mut *conn)
            .await?)
    }

    async fn items(&self, ctx: &Context<'_>) -> Result<Vec<CollectionItem>> {
        let mut conn = get_connection(ctx).await?;
        Ok(CollectionItem::belonging_to(self)
            .load::<CollectionItem>(&mut *conn)
            .await?)
    }
}
