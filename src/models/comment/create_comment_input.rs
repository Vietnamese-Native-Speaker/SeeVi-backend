use async_graphql::InputObject;
use serde::{Serialize, Deserialize};

use crate::object_id::ScalarObjectId;

#[derive(Serialize, Deserialize, Clone, InputObject)]
pub struct CreateCommentInput {
    pub author: ScalarObjectId,
    pub content: String,
}
