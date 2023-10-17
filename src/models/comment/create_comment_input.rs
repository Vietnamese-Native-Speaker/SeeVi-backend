use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

use crate::object_id::ScalarObjectId;

#[derive(Serialize, Deserialize, Clone, InputObject, Builder)]
pub struct CreateCommentInput {
    pub author: ScalarObjectId,
    pub content: String,
}
