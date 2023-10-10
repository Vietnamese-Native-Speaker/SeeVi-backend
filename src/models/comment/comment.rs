use async_graphql::{ComplexObject, SimpleObject};
use mongodb::bson::{self, DateTime};
use serde::{Deserialize, Serialize};

use crate::object_id::ScalarObjectId;

use super::create_comment_input::CreateCommentInput;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, Builder)]
#[graphql(complex)]
pub struct Comment {
    #[serde(rename = "_id")]
    pub id: ScalarObjectId,
    pub author: ScalarObjectId,

    pub content: String,

    #[graphql(skip)]
    pub created: DateTime,

    pub likes: u32,
    pub bookmarks: u32,
    pub shares: u32,

    pub replies: Vec<ScalarObjectId>,
}

#[ComplexObject]
impl Comment {
    async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}

impl Comment {
    pub fn new(content: String, author: bson::oid::ObjectId) -> Self {
        Self {
            id: bson::oid::ObjectId::new().into(),
            author: author.into(),
            content,
            created: bson::DateTime::now(),
            likes: 0,
            bookmarks: 0,
            shares: 0,
            replies: vec![],
        }
    }
}

impl From<CreateCommentInput> for Comment {
    fn from(input: CreateCommentInput) -> Self {
        Self::new(input.content, input.author.into())
    }
}
