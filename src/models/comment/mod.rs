pub mod bookmark;
pub mod comment;
mod create_comment_input;
pub mod like;
pub mod update_comment_input;

pub use bookmark::CommentBookmark;
pub use bookmark::CommentBookmarkKey as BookmarkKey;
pub use comment::Comment;
pub use create_comment_input::{
    CreateCommentInput, CreateCommentInputBuilder, CreateCommentInputBuilderError,
};
pub use like::CommentLikeKey as LikeKey;
pub use like::CommentLike;
pub use update_comment_input::UpdateCommentInput;
