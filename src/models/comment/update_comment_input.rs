use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

use crate::object_id::ScalarObjectId;

#[derive(Serialize, Deserialize, Clone, InputObject, Builder, Default)]
#[builder(pattern = "owned", setter(into, prefix = "with", strip_option))]

pub struct UpdateCommentInput {
    #[builder(default)]
    pub id: ScalarObjectId,
    #[builder(default)]
    pub content: Option<String>,
    #[builder(default)]
    pub replies: Option<Vec<ScalarObjectId>>,
}

impl UpdateCommentInput {
    pub fn builder() -> UpdateCommentInputBuilder {
        UpdateCommentInputBuilder::default()
    }
}
