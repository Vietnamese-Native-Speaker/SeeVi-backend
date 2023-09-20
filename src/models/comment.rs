use async_graphql::{ComplexObject, SimpleObject};
use mongodb::bson::{DateTime, Uuid, self};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, SimpleObject, Builder)]
#[graphql(complex)]
struct Comment {
    pub author: bson::oid::ObjectId,
    pub commented_on: Uuid,
    pub content: String,

    #[graphql(skip)]
    pub created: DateTime,
}

#[ComplexObject]
impl Comment {
    async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}
