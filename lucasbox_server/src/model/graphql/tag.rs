use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Tag {
  pub id: u32,
  pub name: String,
}
