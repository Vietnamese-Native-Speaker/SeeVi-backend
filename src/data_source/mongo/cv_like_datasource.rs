//! Implements the `cv::LikeDataSource` trait for `MongoDB`.

use std::fmt::Display;

use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;
use async_graphql::futures_util::stream::StreamExt;

use crate::{data_source::cv, models::cv::Like, services::cv_service::error::CVServiceError};

use super::MongoDB;

use mongodb::bson;
const CV_LIKE_COLLECTION: &str = "cv_likes";
/// Error type for `LikeDataSource` operations.
#[derive(Debug)]
pub enum LikeError {
    // fail to add likes
    AddLikesFail,

    // fail to remove likes
    DeleteLikesFail,

    // fail to get the number of likes
    LikesNumberNotFound,

    // invalid cv-id
    InvalidCVId(ObjectId),

    // invalid user-id
    InvalidUserId(ObjectId),

    // cannot find like
    LikeNotFound,

    // Like already exists
    LikeAlreadyExists,

    // fail to do queries
    QueryFail,
}

impl Display for LikeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            LikeError::AddLikesFail =>{
                write!(f, "fail to add likes!")
            },
            LikeError::DeleteLikesFail =>{
                write!(f, "fail to remove likes!")
            },
            LikeError::InvalidCVId(id) =>{
                write!(f, "cv-id {:?} is invalid!", id)
            }
            LikeError::InvalidUserId(id) =>{
                write!(f, "user-id {:?} is invalid!", id)
            },
            LikeError::LikeNotFound =>{
                write!(f, "cannot find like!")
            },
            LikeError::LikesNumberNotFound =>{
                write!(f, "fail to get the number of likes!")
            },
            LikeError::LikeAlreadyExists => {
                write!(f, "like already exists!")
            },
            LikeError::QueryFail => {
                write!(f, "fail to do queries")
            }
        }
    }
}

impl std::error::Error for LikeError {

}

impl From<LikeError> for CVServiceError {
    fn from(value: LikeError) -> Self {
        match value{
            LikeError::AddLikesFail => CVServiceError::UpdateLikeFailed,
            LikeError::DeleteLikesFail => CVServiceError::UpdateLikeFailed,
            LikeError::InvalidCVId(id) => CVServiceError::InvalidId(id),
            LikeError::InvalidUserId(id) => CVServiceError::AuthorIdNotFound(id),
            LikeError::LikeAlreadyExists => CVServiceError::UpdateLikeFailed,
            LikeError::LikeNotFound => CVServiceError::LikeNotFound,
            LikeError::QueryFail => CVServiceError::QueryFail,
            LikeError::LikesNumberNotFound => CVServiceError::LikeNotFound
        }
    }
}

#[async_trait::async_trait]
impl cv::like::LikeDataSource for MongoDB {
    type Error = LikeError;

    async fn add_like(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error>{
        let collection = self.db.collection::<Like>(CV_LIKE_COLLECTION);
        let filter = bson::doc!{
            "key.user_id" : user_id.clone(),
            "key.cv_id" : cv_id.clone()
        };
        let check_exist = collection.find_one(filter, None).await;
        match check_exist{
            Ok(like_option) =>{
                match like_option{
                    Some(like) => Err(LikeError::LikeAlreadyExists),
                    None => {
                        let like = Like::new(user_id, cv_id);
                        let result = collection.insert_one(like, None).await;
                        match result {
                            Ok(_) => Ok(()),
                            Err(_) => Err(LikeError::AddLikesFail)
                        }
                    }
                }
            },
            Err(_) => Err(LikeError::QueryFail)
        }
    }

    async fn delete_like(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error>{
        let collection  = self.db.collection::<Like>(CV_LIKE_COLLECTION);
        let filter = bson::doc!{
            "key.user_id": user_id,
            "key.cv_id": cv_id
        };
        let result = collection.find_one_and_delete(filter, None).await;
        match result{
            Ok(_) => Ok(()),
            Err(_) => Err(LikeError::DeleteLikesFail)
        }
    }

    async fn get_likes_count(&self, cv_id: ObjectId) -> Result<i32, Self::Error> {
        let collection = self.db.collection::<Like>(CV_LIKE_COLLECTION);
        let filter = bson::doc!{
            "key.cv_id": cv_id
        };
        let result = collection.count_documents(filter, None).await;
        match result{
            Ok(count) => Ok(count as i32),
            Err(_) => Err(LikeError::LikesNumberNotFound)
        }
    }

    async fn get_likes(&self, cv_id: ObjectId) -> Result<BoxStream<Like>, Self::Error>{
        let collection = self.db.collection::<Like>(CV_LIKE_COLLECTION);
        let filter = bson::doc!{
            "key.cv_id": cv_id
        };
        let result = collection.find(filter, None).await;
        match result{
            Ok(cursor) => Ok(cursor.map(|like|like.unwrap()).boxed()),
            Err(_) => Err(LikeError::QueryFail)
        }
    }
}
