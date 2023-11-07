use async_graphql::{ComplexObject, SimpleObject};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use crate::object_id::ScalarObjectId;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, SimpleObject)]
pub struct CommentBookmarkKey {
    pub user_id: ScalarObjectId,
    pub comment_id: ScalarObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, PartialEq)]
#[graphql(complex)]
pub struct CommentBookmark {
    #[serde(rename = "_id")]
    pub key: CommentBookmarkKey,
    #[graphql(skip)]
    pub created: DateTime,
}

impl CommentBookmark {
    pub fn new(user_id: ScalarObjectId, comment_id: ScalarObjectId) -> Self {
        Self {
            key: CommentBookmarkKey {
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

#[ComplexObject]
impl CommentBookmark {
    pub async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}

