use std::time::SystemTime;

use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};
use diesel::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    auth::make_sure_is_connected,
    entities::{CollectionItem, Tag, User},
    extract_connexion,
    schema_db::item_files,
};

#[derive(Clone, Debug, SimpleObject, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(CollectionItem))]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[graphql(complex)]
pub struct ItemFile {
    pub id: i32,
    #[graphql(skip)]
    pub collection_item_id: i32,
    pub name: String,
    #[graphql(skip)]
    pub filepath: String,
    #[graphql(skip)]
    pub created_by: Option<Uuid>,
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

#[derive(Default)]
pub struct ItemFileRootQuery;

#[Object]
impl ItemFileRootQuery {
    async fn item_file(&self, ctx: &Context<'_>, id: i32) -> Result<Option<ItemFile>> {
        use crate::schema_db::item_files::dsl::item_files;

        make_sure_is_connected(ctx)?;
        let mut conn = extract_connexion(ctx).await?;
        Ok(item_files.find(id).first(&mut *conn).await.optional()?)
    }
}
