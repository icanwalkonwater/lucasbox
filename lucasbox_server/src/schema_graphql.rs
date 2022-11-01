use async_graphql::{EmptySubscription, MergedObject};

use crate::entities::{
    collection::CollectionRootQuery,
    collection_item::CollectionItemRootQuery,
    item_file::ItemFileRootQuery,
    tag::TagRootQuery,
    user::{UserMutation, UserRootQuery},
};

pub type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

#[derive(MergedObject, Default)]
pub struct Query(
    UserRootQuery,
    CollectionRootQuery,
    CollectionItemRootQuery,
    ItemFileRootQuery,
    TagRootQuery,
);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation);

pub type Subscription = EmptySubscription;
