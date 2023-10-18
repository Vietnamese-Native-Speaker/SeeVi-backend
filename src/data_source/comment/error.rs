use mongodb::bson;
use std::fmt;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]

pub enum CommentDataSourceError {
    /// Error when the comment id is not found
    IdNotFound(bson::oid::ObjectId),

    /// Error when the comment content is empty
    EmptyContent,

    /// Error when the number of likes is invalid
    NoLikes,

    /// Error when the number of bookmarks is invalid
    NoBookmarks,

    /// Error when create comment fails
    CreateCommentFailed,

    /// Error when update comment fails
    UpdateCommentFailed,

    /// Error when delete comment fails
    DeleteCommentFailed,

    /// Database error
    DatabaseError,
}

impl fmt::Display for CommentDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommentDataSourceError::IdNotFound(id) => {
                write!(f, "Comment id {} not found", id)
            }
            CommentDataSourceError::EmptyContent => {
                write!(f, "Comment content is empty")
            }
            CommentDataSourceError::NoLikes => {
                write!(f, "Comment likes is zero")
            }
            CommentDataSourceError::NoBookmarks => {
                write!(f, "Comment bookmarks is zero")
            }
            CommentDataSourceError::CreateCommentFailed => {
                write!(f, "Create comment failed")
            }
            CommentDataSourceError::UpdateCommentFailed => {
                write!(f, "Update comment failed")
            }
            CommentDataSourceError::DeleteCommentFailed => {
                write!(f, "Delete comment failed")
            }
            CommentDataSourceError::DatabaseError => {
                write!(f, "Database error")
            }
        }
    }
}
