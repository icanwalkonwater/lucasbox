use std::time::SystemTime;

use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use diesel::*;
use diesel_async::RunQueryDsl;

use crate::{
    entities::{CollectionItem, Tag},
    extract_connexion,
    schema_db::item_files,
};

#[derive(Clone, Debug, SimpleObject, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(CollectionItem))]
#[graphql(complex)]
pub struct ItemFile {
    pub id: i32,
    #[graphql(skip)]
    pub collection_item_id: i32,
    pub name: String,
    #[graphql(skip)]
    pub filepath: String,
    #[graphql(skip)]
    pub updated_at: SystemTime,
    #[graphql(skip)]
    pub created_at: SystemTime,
}

#[ComplexObject]
impl ItemFile {
    async fn tags(&self, ctx: &Context<'_>) -> Result<Vec<Tag>> {
        use crate::schema_db::{tag_item_files, tags};

        let mut conn = extract_connexion(ctx).await?;
        Ok(tag_item_files::table
            .inner_join(tags::table)
            .filter(tag_item_files::item_file_id.eq(self.id))
            .select(tags::all_columns)
            .load::<Tag>(&mut *conn)
            .await?)
    }
}
