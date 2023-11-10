use async_graphql::{ComplexObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::{object_id::ScalarObjectId, common::DateTime};

use super::Key;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(name = "CommentLike")]
pub struct Like {
    #[serde(rename = "_id")]
    pub key: Key,
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
