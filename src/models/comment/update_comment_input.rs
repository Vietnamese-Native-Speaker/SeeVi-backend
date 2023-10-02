use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

use crate::object_id::ScalarObjectId;

#[derive(Serialize, Deserialize, Clone, InputObject, Builder, Default)]
#[builder(
    pattern = "owned",
    setter(into, prefix = "with", strip_option),
)]

pub struct UpdateCommentInput {
    #[builder(default)]
    pub id: ScalarObjectId,
    #[builder(default)]
    pub author_id: ScalarObjectId,
    #[builder(default)]
    pub content: Option<String>,
    #[builder(default)]
    pub likes: Option<u32>,
    #[builder(default)]
    pub bookmarks: Option<u32>,
}

impl UpdateCommentInput {
    pub fn builder() -> UpdateCommentInputBuilder {
        UpdateCommentInputBuilder::default()
    }
}
