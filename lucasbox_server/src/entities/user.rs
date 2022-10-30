use std::time::SystemTime;
use async_graphql::SimpleObject;

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
