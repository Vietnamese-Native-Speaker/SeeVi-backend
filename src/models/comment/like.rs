use async_graphql::{ComplexObject, SimpleObject};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use crate::object_id::ScalarObjectId;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, SimpleObject)]
pub struct CommentLikeKey {
    pub user_id: ScalarObjectId,
    pub comment_id: ScalarObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex)]
pub struct CommentLike {
    #[serde(rename = "_id")]
    pub key: CommentLikeKey,
    #[graphql(skip)]
    pub created: DateTime,
}

impl CommentLike {
    pub fn new(user_id: ScalarObjectId, comment_id: ScalarObjectId) -> Self {
        Self {
            key: CommentLikeKey {
                user_id,
                comment_id,
            },
            created: DateTime::now(),
        }
    }
}

#[ComplexObject]
impl CommentLike {
    pub async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}
