use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct User {
    pub user_id: u32,
    pub username: String,
    pub password: String,
}
