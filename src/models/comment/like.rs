use async_graphql::{ComplexObject, SimpleObject};
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::object_id::ScalarObjectId;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LikeID {
    pub from: ObjectId,
    pub to: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Like {
    #[serde(rename = "_id")]
    like_id: LikeID,
    #[graphql(skip)]
    pub created: DateTime,
}

#[ComplexObject]
impl Like {
    pub async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}
