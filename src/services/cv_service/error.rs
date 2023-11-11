use std::fmt;

use mongodb::bson::oid::ObjectId;

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CVServiceError{
    // uuid cannot be found
    ObjectIdNotFound(ObjectId),

    // author id cannot be found
    AuthorIdNotFound(ObjectId),

    // description is longer than limit
    TooLongDescription,

    // title is empty
    EmptyTitle,

    // id is empty
    EmptyId,

    // title is too long
    TooLongTitle,

    // title is invalid
    InvalidTitle(String),

    // id is invalid
    InvalidId(ObjectId),

    // Cannot find CV
    QueryFail,

    // add comment to cv failed
    AddCommentFailed,

    // remove comment from cv failed
    RemoveCommentFailed,

    /// Database Error
    DatabaseError,

    /// Failed to like cv
    LikeFailed(String),

    /// like not found
    LikeNotFound,

    /// bookmark not found
    BookmarkNotFound,

    /// share not found 
    ShareNotFound, 

    // update like failed
    UpdateLikeFailed,

    /// update bookmark failed
    UpdateBookmarkFailed,

    /// update share failed
    UpdateShareFailed
}

impl fmt::Display for CVServiceError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CVServiceError::LikeFailed(s) => {
                write!(f, "Like failed: {}", s)
            }
            CVServiceError::DatabaseError => {
                write!(f, "Database error")
            }
            CVServiceError::ObjectIdNotFound(uuid) => {
                write!(f, "Uuid {:?} is not found", uuid)
            }
            CVServiceError::TooLongDescription => {
                write!(f, "Description is too long")
            }
            CVServiceError::EmptyTitle => {
                write!(f, "Title cannot be empty")
            }
            CVServiceError::EmptyId => {
                write!(f, "Id cannot be empty")
            }
            CVServiceError::InvalidTitle(s) => {
                write!(f, "Title {:?} is invalid", s)
            }
            CVServiceError::InvalidId(objectid) => {
                write!(f, "Uuid {:?} is invalid", objectid)
            }
            CVServiceError::TooLongTitle => {
                write!(f, "Title is too long")
            }
            CVServiceError::AuthorIdNotFound(objectid) => {
                write!(f, "Author id {:?} is not found", objectid)
            }
            CVServiceError::QueryFail => {
                write!(f, "Fail to find CV")
            }
            CVServiceError::AddCommentFailed => {
                write!(f, "Add comment failed")
            }
            CVServiceError::RemoveCommentFailed => {
                write!(f, "Remove comment failed")
            }
            CVServiceError::BookmarkNotFound => {
                write!(f, "Cannot find bookmark")
            }
            CVServiceError::ShareNotFound => {
                write!(f, "Cannot find share")
            }
            CVServiceError::LikeNotFound => {
                write!(f, "Cannot find like")
            }
            CVServiceError::UpdateBookmarkFailed => {
                write!(f, "Cannot update bookmark")
            }
            CVServiceError::UpdateShareFailed => {
                write!(f, "Cannot update share")
            }
            CVServiceError::UpdateLikeFailed => {
                write!(f, "Cannot update like")
            }
        }
    }
}
