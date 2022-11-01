use std::time::SystemTime;

use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};
use diesel::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    auth::make_sure_is_connected,
    entities::{CollectionItem, Tag, User},
    extract_connexion,
    schema_db::collections,
};

#[derive(Clone, Debug, SimpleObject, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Self, foreign_key = parent_id))]
#[diesel(belongs_to(User, foreign_key = created_by))]
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
    pub created_by: Option<Uuid>,
    #[graphql(skip)]
    pub updated_at: SystemTime,
    #[graphql(skip)]
    pub created_at: SystemTime,
}

#[ComplexObject]
impl Collection {
    async fn tags(&self, ctx: &Context<'_>) -> Result<Vec<Tag>> {
        use crate::schema_db::{tag_collections, tags};

        let mut conn = extract_connexion(ctx).await?;
        Ok(tag_collections::table
            .inner_join(tags::table)
            .filter(tag_collections::collection_id.eq(self.id))
            .select(tags::all_columns)
            .load::<Tag>(&mut *conn)
            .await?)
    }

    async fn children(&self, ctx: &Context<'_>) -> Result<Vec<Collection>> {
        let mut conn = extract_connexion(ctx).await?;
        Ok(Collection::belonging_to(self)
            .load::<Collection>(&mut *conn)
            .await?)
    }

    async fn items(&self, ctx: &Context<'_>) -> Result<Vec<CollectionItem>> {
        let mut conn = extract_connexion(ctx).await?;
        Ok(CollectionItem::belonging_to(self)
            .load::<CollectionItem>(&mut *conn)
            .await?)
    }
}

#[derive(Default)]
pub struct CollectionRootQuery;

#[Object]
impl CollectionRootQuery {
    async fn collection(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Collection>> {
        use crate::schema_db::collections::dsl::collections;

        make_sure_is_connected(ctx)?;
        let mut conn = extract_connexion(ctx).await?;
        Ok(collections.find(id).first(&mut *conn).await.optional()?)
    }

    async fn root_collections(&self, ctx: &Context<'_>) -> Result<Vec<Collection>> {
        use crate::schema_db::collections::dsl::*;

        make_sure_is_connected(ctx)?;
        let mut conn = extract_connexion(ctx).await?;
        Ok(collections
            .filter(parent_id.is_null())
            .load::<Collection>(&mut *conn)
            .await?)
    }
}
