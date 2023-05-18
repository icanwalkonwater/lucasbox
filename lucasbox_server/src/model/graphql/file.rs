use async_graphql::{ComplexObject, SimpleObject};
use uuid::Uuid;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct File {
  pub id: Uuid,
  pub name: String,
  pub filepath: String,
}

#[ComplexObject]
impl File {
  async fn size(&self) -> async_graphql::Result<Option<u64>> {
    if !tokio::fs::try_exists(&self.filepath).await? {
      return Ok(None);
    }
    let metadata = tokio::fs::metadata(&self.filepath).await?;
    Ok(Some(metadata.len()))
  }
}
