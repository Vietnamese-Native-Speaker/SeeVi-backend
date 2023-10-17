use async_graphql::ErrorExtensions;
use mongodb::bson::{self, oid::ObjectId};
use std::fmt;

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BookmarkDataSourceError{
    // fail to add bookmark
    AddBookmarkFail,

    // fail to remove bookmark
    DeleteBookmarkFail,

    // invalid comment-id
    InvalidCommentId(ObjectId),

    // invalid user-id
    InvalidUserId(ObjectId),

    // cannot find bookmark
    BookmarkNotFound,

    // Bookmark already exists
    BookmarkAlreadyExists,

    // fail to do queries
    QueryFail,
}

impl fmt::Display for BookmarkDataSourceError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BookmarkDataSourceError::AddBookmarkFail => {
                write!(f, "fail to add bookmark!")
            },
            BookmarkDataSourceError::DeleteBookmarkFail =>{
                write!(f, "fail to remove bookmark!")
            },
            BookmarkDataSourceError::BookmarkAlreadyExists => {
                write!(f, "bookmark already exists!")
            },
            BookmarkDataSourceError::BookmarkNotFound => {
                write!(f, "cannot find bookmark!")
            },
            BookmarkDataSourceError::InvalidCommentId(id) => {
                write!(f, "comment-id {:?} is invalid!", id)
            },
            BookmarkDataSourceError::InvalidUserId(id) => {
                write!(f, "user-id {:?} is invalid!", id)
            }
            BookmarkDataSourceError::QueryFail => {
                write!(f, "fail to do queries!")
            }
        }
    }
}