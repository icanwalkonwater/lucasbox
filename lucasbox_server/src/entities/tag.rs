use std::time::SystemTime;

use async_graphql::{Context, Object, Result, SimpleObject};
use diesel::*;
use diesel_async::RunQueryDsl;

use crate::{
    auth::make_sure_is_connected,
    entities::{Collection, CollectionItem, ItemFile},
    extract_connexion,
    schema_db::{tag_collection_items, tag_collections, tag_item_files, tags},
};

#[derive(Clone, Debug, SimpleObject, Queryable, Identifiable)]
pub struct Tag {
    pub id: i32,
    pub label: String,
    #[graphql(skip)]
    pub updated_at: SystemTime,
    #[graphql(skip)]
    pub created_at: SystemTime,
}

#[derive(Clone, Debug, Queryable, Identifiable, Associations)]
#[diesel(primary_key(tag_id, collection_id))]
#[diesel(belongs_to(Tag))]
#[diesel(belongs_to(Collection))]
pub struct TagCollection {
    pub tag_id: i32,
    pub collection_id: i32,
}

#[derive(Clone, Debug, Queryable, Identifiable, Associations)]
#[diesel(primary_key(tag_id, collection_item_id))]
#[diesel(belongs_to(Tag))]
#[diesel(belongs_to(CollectionItem))]
pub struct TagCollectionItem {
    pub tag_id: i32,
    pub collection_item_id: i32,
}

#[derive(Clone, Debug, Queryable, Identifiable, Associations)]
#[diesel(primary_key(tag_id, item_file_id))]
#[diesel(belongs_to(Tag))]
#[diesel(belongs_to(ItemFile))]
pub struct TagItemFile {
    pub tag_id: i32,
    pub item_file_id: i32,
}

#[derive(Default)]
pub struct TagRootQuery;

#[Object]
impl TagRootQuery {
    async fn tag(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Tag>> {
        use crate::schema_db::tags::dsl::tags;

        make_sure_is_connected(ctx)?;
        let mut conn = extract_connexion(ctx).await?;
        Ok(tags.find(id).first(&mut *conn).await.optional()?)
    }

    async fn tags(&self, ctx: &Context<'_>) -> Result<Vec<Tag>> {
        use crate::schema_db::tags::dsl::tags;

        make_sure_is_connected(ctx)?;
        let mut conn = extract_connexion(ctx).await?;
        Ok(tags.load(&mut *conn).await?)
    }
}
