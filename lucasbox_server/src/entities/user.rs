use async_graphql::SimpleObject;
use std::time::SystemTime;

#[derive(Clone, Debug, SimpleObject)]
pub struct User {
    pub id: u32,
    pub username: String,
    #[graphql(skip)]
    pub password: String,
    #[graphql(skip)]
    pub updated_at: SystemTime,
    #[graphql(skip)]
    pub created_at: SystemTime,
}
