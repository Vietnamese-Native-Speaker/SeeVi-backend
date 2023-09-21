use async_graphql::{ComplexObject, SimpleObject};
use mongodb::bson::{self, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, SimpleObject, Builder)]
#[graphql(complex)]
struct Comment {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub author: bson::oid::ObjectId,

    pub content: String,

    #[graphql(skip)]
    pub created: DateTime,

    pub likes: u32,
    pub bookmarks: u32,
    pub shares: u32,

    pub replies: Vec<bson::oid::ObjectId>,
}

#[ComplexObject]
impl Comment {
    async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}
