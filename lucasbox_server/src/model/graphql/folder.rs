use async_graphql::{ComplexObject, SimpleObject};
use uuid::Uuid;
use crate::model::graphql::file::File;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Folder {
  pub id: Uuid,
  pub name: String,
}

#[ComplexObject]
impl Folder {
  async fn files(&self) -> Vec<File> {
    vec![
      File {
        id: Uuid::new_v4(),
        name: "Test".to_string(),
        filepath: "./Cargo.toml".to_string(),
      },
      File {
        id: Uuid::new_v4(),
        name: "Uwu".to_string(),
        filepath: "/owo.mkv".to_string(),
      },
    ]
  }

  async fn folders(&self) -> Vec<Folder> {
    vec![Folder {
      id: Uuid::new_v4(),
      name: "Another".to_string(),
    }]
  }
}
