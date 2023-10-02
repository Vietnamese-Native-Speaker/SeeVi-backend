use async_graphql::ErrorExtensions;
use mongodb::bson;
use std::fmt;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]

pub enum CommentServiceError {
    // Error when the comment id is not found
    IdNotFound(bson::oid::ObjectId),
    // Error when the comment content is empty
    EmptyContent,
    // Error when the comment content is invalid
    InvalidContent(String),
    /// Error when the number of likes is invalid
    InvalidLikes,
    /// Error when the number of bookmarks is invalid
    InvalidBookmarks,
    /// Error when create comment fails
    CreateCommentFailed,
    /// Error when update comment fails
    UpdateCommentFailed,
    /// Error when delete comment fails
    DeleteCommentFailed,
    /// Database error
    DatabaseError,
}

impl fmt::Display for CommentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommentServiceError::IdNotFound(id) => {
                write!(f, "Comment id {} not found", id)
            }
            CommentServiceError::EmptyContent => {
                write!(f, "Comment content is empty")
            }
            CommentServiceError::InvalidContent(content) => {
                write!(f, "Comment content {} is invalid", content)
            }
            CommentServiceError::InvalidLikes => {
                write!(f, "Comment likes is invalid")
            }
            CommentServiceError::InvalidBookmarks => {
                write!(f, "Comment bookmarks is invalid")
            }
            CommentServiceError::CreateCommentFailed => {
                write!(f, "Create comment failed")
            }
            CommentServiceError::UpdateCommentFailed => {
                write!(f, "Update comment failed")
            }
            CommentServiceError::DeleteCommentFailed => {
                write!(f, "Delete comment failed")
            }
            CommentServiceError::DatabaseError => {
                write!(f, "Database error")
            }
        }
    }
}

impl ErrorExtensions for CommentServiceError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string())
            .extend_with(|_, e| e.set("code", "INVALID_COMMENT"))
    }
}
