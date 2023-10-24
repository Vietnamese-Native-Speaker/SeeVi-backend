pub mod bookmark;
pub mod comment;
mod create_comment_input;
pub mod like;
pub mod update_comment_input;

pub use bookmark::Bookmark;
pub use bookmark::Key as BookmarkKey;
pub use comment::Comment;
pub use create_comment_input::{
    CreateCommentInput, CreateCommentInputBuilder, CreateCommentInputBuilderError,
};
pub use like::Key as LikeKey;
pub use like::Like;
pub use update_comment_input::UpdateCommentInput;
