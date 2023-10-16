use mongodb::bson::oid::ObjectId;
use std::fmt;

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LikeDataSourceError{
    // fail to add likes
    AddLikesFail,

    // fail to remove likes
    DeleteLikesFail,

    // fail to get the number of likes
    LikesNumberNotFound,

    // invalid comment-id
    InvalidCommentId(ObjectId),

    // invalid user-id
    InvalidUserId(ObjectId),

    // cannot find like
    LikeNotFound,

    // Like already exists
    LikeAlreadyExists,

    // fail to do queries
    QueryFail,
}

impl fmt::Display for LikeDataSourceError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            LikeDataSourceError::AddLikesFail =>{
                write!(f, "fail to add likes!")
            },
            LikeDataSourceError::DeleteLikesFail =>{
                write!(f, "fail to remove likes!")
            },
            LikeDataSourceError::InvalidCommentId(id) =>{
                write!(f, "comment-id {:?} is invalid!", id)
            }
            LikeDataSourceError::InvalidUserId(id) =>{
                write!(f, "user-id {:?} is invalid!", id)
            },
            LikeDataSourceError::LikeNotFound =>{
                write!(f, "cannot find like!")
            },
            LikeDataSourceError::LikesNumberNotFound =>{
                write!(f, "fail to get the number of likes!")
            },
            LikeDataSourceError::LikeAlreadyExists => {
                write!(f, "like already exists!")
            },
            LikeDataSourceError::QueryFail => {
                write!(f, "fail to do queries")
            }
        }
    }
}

