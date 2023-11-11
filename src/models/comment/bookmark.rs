use async_graphql::{ComplexObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::{object_id::ScalarObjectId, common::DateTime};

use super::Key;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, PartialEq)]
#[graphql(name = "CommentBookmark")]
pub struct Bookmark {
    #[serde(rename = "_id")]
    pub key: Key,
    pub created: DateTime,
}

impl Bookmark {
    pub fn new(user_id: ScalarObjectId, comment_id: ScalarObjectId) -> Self {
        Self {
            key: Key {
                user_id,
                comment_id,
            },
            created: DateTime::now(),
        }
    }

    pub fn user_id(&self) -> &ScalarObjectId {
        &self.key.user_id
    }

    pub fn comment_id(&self) -> &ScalarObjectId {
        &self.key.comment_id
    }
}
