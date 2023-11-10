use async_graphql::{ComplexObject, SimpleObject};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use crate::object_id::ScalarObjectId;

use super::Key;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex, name = "CommentLike")]
pub struct Like {
    #[serde(rename = "_id")]
    pub key: Key,
    #[graphql(skip)]
    pub created: DateTime,
}

impl Like {
    pub fn new(user_id: ScalarObjectId, comment_id: ScalarObjectId) -> Self {
        Self {
            key: Key {
                user_id,
                comment_id,
            },
            created: DateTime::now(),
        }
    }
}

#[ComplexObject]
impl Like {
    pub async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}
