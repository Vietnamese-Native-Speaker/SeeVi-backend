use async_graphql::ErrorExtensions;
use mongodb::bson;
use std::fmt;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]

pub enum CommentDataSourceError {
    // Error when the comment id is not found
    IdNotFound(bson::oid::ObjectId),
    // Error when the comment content is empty
    EmptyContent,
    // Error when the comment content is invalid
    InvalidContent(String),
    // Error when create comment fails
    CreateCommentFailed,
    // Error when update comment fails
    UpdateCommentFailed,
    // Error when delete comment fails
    DeleteCommentFailed,
    // Database error
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
            CommentDataSourceError::InvalidContent(content) => {
                write!(f, "Comment content {} is invalid", content)
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

impl ErrorExtensions for CommentDataSourceError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string())
            .extend_with(|_, e| e.set("code", "INVALID_COMMENT"))
    }
}
