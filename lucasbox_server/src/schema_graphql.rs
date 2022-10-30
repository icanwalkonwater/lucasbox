use async_graphql::{EmptyMutation, EmptySubscription, MergedObject};

use crate::entities::{
    collection::CollectionRootQuery, collection_item::CollectionItemRootQuery,
    item_file::ItemFileRootQuery, tag::TagRootQuery,
};

pub type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

#[derive(MergedObject, Default)]
pub struct Query(
    CollectionRootQuery,
    CollectionItemRootQuery,
    ItemFileRootQuery,
    TagRootQuery,
);

pub type Mutation = EmptyMutation;

pub type Subscription = EmptySubscription;
