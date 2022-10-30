use async_graphql::SimpleObject;
use diesel::{Associations, Identifiable, Queryable};
use std::time::SystemTime;

use crate::{
    entities::{Collection, CollectionItem, ItemFile},
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
