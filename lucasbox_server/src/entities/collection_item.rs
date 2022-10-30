use std::time::SystemTime;

use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use diesel::*;
use diesel_async::RunQueryDsl;

use crate::{
    entities::{Collection, ItemFile, Tag},
    get_connection,
    schema_db::collection_items,
};

#[derive(Clone, Debug, SimpleObject, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Collection))]
#[graphql(complex)]
pub struct CollectionItem {
    pub id: i32,
    #[graphql(skip)]
    pub collection_id: i32,
    pub name: String,
    pub description: Option<String>,
    #[graphql(skip)]
    pub updated_at: SystemTime,
    #[graphql(skip)]
    pub created_at: SystemTime,
}

#[ComplexObject]
impl CollectionItem {
    async fn tags(&self, ctx: &Context<'_>) -> Result<Vec<Tag>> {
        use crate::schema_db::{tag_collection_items, tags};

        let mut conn = get_connection(ctx).await?;
        Ok(tag_collection_items::table
            .inner_join(tags::table)
            .filter(tag_collection_items::collection_item_id.eq(self.id))
            .select(tags::all_columns)
            .load::<Tag>(&mut *conn)
            .await?)
    }

    async fn files(&self, ctx: &Context<'_>) -> Result<Vec<ItemFile>> {
        let mut conn = get_connection(ctx).await?;
        Ok(ItemFile::belonging_to(self)
            .load::<ItemFile>(&mut *conn)
            .await?)
    }
}
