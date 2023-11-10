pub mod bookmark;
pub mod comment;
mod create_comment_input;
pub mod like;
pub mod update_comment_input;

use async_graphql::SimpleObject;
pub use bookmark::Bookmark;
pub use comment::Comment;
pub use create_comment_input::{
    CreateCommentInput, CreateCommentInputBuilder, CreateCommentInputBuilderError,
};
pub use like::Like;
use serde::Deserialize;
use serde::Serialize;
pub use update_comment_input::UpdateCommentInput;

use crate::object_id::ScalarObjectId;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, SimpleObject)]
#[graphql(name = "CommentInteractionKey")]
pub struct Key {
    pub user_id: ScalarObjectId,
    pub comment_id: ScalarObjectId,
}
