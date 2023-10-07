use async_graphql::ErrorExtensions;
use mongodb::bson;
use std::fmt;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]

pub enum CommentServiceError {
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

impl fmt::Display for CommentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommentServiceError::IdNotFound(id) => {
                write!(f, "Comment id {} not found", id)
            }
            CommentServiceError::EmptyContent => {
                write!(f, "Comment content is empty")
            }
            CommentServiceError::NoLikes => {
                write!(f, "Comment likes is zero")
            }
            CommentServiceError::NoBookmarks => {
                write!(f, "Comment bookmarks is zero")
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

static ID_NOT_FOUND: &str = "ID_NOT_FOUND";
static EMPTY_CONTENT: &str = "EMPTY_CONTENT";
static NO_LIKES: &str = "NO_LIKES";
static NO_BOOKMARKS: &str = "NO_BOOKMARKS";
static CREATE_COMMENT_FAILED: &str = "CREATE_COMMENT_FAILED";
static UPDATE_COMMENT_FAILED: &str = "UPDATE_COMMENT_FAILED";
static DELETE_COMMENT_FAILED: &str = "DELETE_COMMENT_FAILED";
static DATABASE_ERROR: &str = "DATABASE_ERROR";

impl ErrorExtensions for CommentServiceError {
    fn extend(&self) -> async_graphql::Error {
        match self {
            CommentServiceError::IdNotFound(id) => {
                async_graphql::Error::new(self.to_string() + &id.to_string()).extend_with(|_, e| e.set("code", ID_NOT_FOUND))
            }
            CommentServiceError::EmptyContent => {
                async_graphql::Error::new(&self.to_string()).extend_with(|_, e| e.set("code", EMPTY_CONTENT))
            }
            CommentServiceError::NoLikes => {
                async_graphql::Error::new(&self.to_string()).extend_with(|_, e| e.set("code", NO_LIKES))
            }
            CommentServiceError::NoBookmarks => {
                async_graphql::Error::new(&self.to_string()).extend_with(|_, e| e.set("code", NO_BOOKMARKS))
            }
            CommentServiceError::CreateCommentFailed => {
                async_graphql::Error::new(&self.to_string()).extend_with(|_, e| e.set("code", CREATE_COMMENT_FAILED))
            }
            CommentServiceError::UpdateCommentFailed => {
                async_graphql::Error::new(&self.to_string()).extend_with(|_, e| e.set("code", UPDATE_COMMENT_FAILED))
            }
            CommentServiceError::DeleteCommentFailed => {
                async_graphql::Error::new(&self.to_string()).extend_with(|_, e| e.set("code", DELETE_COMMENT_FAILED))
            }
            CommentServiceError::DatabaseError => {
                async_graphql::Error::new(&self.to_string()).extend_with(|_, e| e.set("code", DATABASE_ERROR))
            }
        }
    }
}
