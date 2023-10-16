use async_graphql::{ComplexObject, SimpleObject};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use crate::object_id::ScalarObjectId;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, SimpleObject)]
pub struct Key {
    pub user_id: ScalarObjectId,
    pub comment_id: ScalarObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Like {
    #[serde(rename = "_id")]
    pub key: Key,
    #[graphql(skip)]
    pub created: DateTime,
}

#[ComplexObject]
impl Like {
    pub async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}
