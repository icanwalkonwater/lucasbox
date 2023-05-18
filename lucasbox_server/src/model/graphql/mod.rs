use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
pub use file::*;
pub use folder::*;
pub use tag::*;
use uuid::Uuid;

mod file;
mod folder;
mod tag;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
  async fn tags(&self) -> Vec<Tag> {
    vec![
      Tag {
        id: 0,
        name: "Franchise".into(),
      },
      Tag {
        id: 1,
        name: "Movie".into(),
      },
    ]
  }

  async fn folder(&self, _id: Option<Uuid>) -> Folder {
    Folder {
      id: Uuid::new_v4(),
      name: "Root".into(),
    }
  }
}

pub type AppGraphQlSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
