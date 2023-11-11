//! Implements the `ShareDataSource` trait for `MongoDB`.

use std::fmt::Display;

use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;
use async_graphql::futures_util::stream::StreamExt;

use crate::{
    data_source::cv::share::ShareDataSource,
    models::cv::{interactions::Share, CV},
    services::cv_service::error::CVServiceError,
};

use super::MongoDB;
use mongodb::bson;
const CV_SHARE_COLLECTION: &str = "shares";
const CV_COLLECTION: &str = "cvs";
/// Error type for `LikeDataSource` operations.
#[derive(Debug, PartialEq, Clone)]
pub enum ShareError {
    // fail to add share
    AddShareFail,

    // fail to remove share
    DeleteShareFail,

    // invalid comment-id
    InvalidCVId(ObjectId),

    // invalid user-id
    InvalidUserId(ObjectId),

    // cannot find share
    ShareNotFound,

    // share already exists
    ShareAlreadyExists,

    // fail to do queries
    QueryFail,
}

impl Display for ShareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            ShareError::AddShareFail => {
                write!(f, "fail to add share!")
            },
            ShareError::DeleteShareFail =>{
                write!(f, "fail to remove share!")
            },
            ShareError::ShareAlreadyExists => {
                write!(f, "share already exists!")
            },
            ShareError::ShareNotFound => {
                write!(f, "cannot find share!")
            },
            ShareError::InvalidCVId(id) => {
                write!(f, "cv-id {:?} is invalid!", id)
            },
            ShareError::InvalidUserId(id) => {
                write!(f, "user-id {:?} is invalid!", id)
            }
            ShareError::QueryFail => {
                write!(f, "fail to do queries!")
            }
        }
    }
}

impl std::error::Error for ShareError {
}

impl From<ShareError> for CVServiceError {
    fn from(value: ShareError) -> Self {
        match value{
            ShareError::AddShareFail => CVServiceError::UpdateShareFailed,
            ShareError::DeleteShareFail => CVServiceError::UpdateShareFailed,
            ShareError::InvalidCVId(id) => CVServiceError::InvalidId(id),
            ShareError::InvalidUserId(id) => CVServiceError::AuthorIdNotFound(id),
            ShareError::ShareAlreadyExists => CVServiceError::UpdateShareFailed,
            ShareError::ShareNotFound => CVServiceError::ShareNotFound,
            ShareError::QueryFail => CVServiceError::QueryFail
        }
    }
}

#[async_trait::async_trait]
impl ShareDataSource for MongoDB {
    type Error = ShareError;

    async fn add_share(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        let collection = self.db.collection::<Share>(CV_SHARE_COLLECTION);
        let filter = bson::doc!{
            "_id.user_id": user_id.clone(),
            "_id.cv_id": cv_id
        };
        let result_exist = collection.find_one(filter, None).await;
        match result_exist {
            Ok(share_option) => {
                match share_option {
                    Some(_share) => Err(ShareError::ShareAlreadyExists),
                    None => {
                        let share = Share::new(user_id, cv_id);
                        let add_result = collection.insert_one(share, None).await;
                        match add_result{
                            Ok(_) => Ok(()),
                            Err(_) => Err(ShareError::AddShareFail)
                        }
                    }
                }
            },
            Err(_) => Err(ShareError::QueryFail)
        }
    }

    async fn delete_share(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        let collection = self.db.collection::<Share>(CV_SHARE_COLLECTION);
        let filter = bson::doc!{
            "_id.user_id": user_id,
            "_id.cv_id": cv_id
        };
        let result = collection.find_one_and_delete(filter, None).await;
        match result{
            Ok(_) => Ok(()),
            Err(_) => Err(ShareError::DeleteShareFail)
        }
    }

    async fn get_shares_by_user_id(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Share>, Self::Error> {
        let collection = self.db.collection::<Share>(CV_SHARE_COLLECTION);
        let filter = bson::doc!{
            "_id.user_id": user_id
        };
        let result = collection.find(filter, None).await;
        match result {
            Ok(cursor) => Ok(cursor.map(|share|share.unwrap()).boxed()),
            Err(_) => Err(ShareError::QueryFail)
        }
    }

    async fn get_shared_cvs_by_user_id(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Result<CV, Self::Error>>, Self::Error> {
        let share_collection = self.db.collection::<Share>(CV_SHARE_COLLECTION);
        let share_filter = bson::doc!{
            "_id.user_id": user_id
        };
        let result = share_collection.find(share_filter, None).await;
        match result{
            Ok(share_cursor) =>{
                let list_cv_id = share_cursor.map(|share|share.unwrap().cv_id().to_owned()).collect::<Vec<ObjectId>>().await;
                let cv_collection = self.db.collection::<CV>(CV_COLLECTION);
                let filter = bson::doc!{
                    "_id": {"$in": list_cv_id}
                };
                let find_result = cv_collection.find(filter, None).await;
                match find_result {
                    Ok(cv_cursor) => Ok(cv_cursor.map(|cv|Ok(cv.unwrap())).boxed()),
                    Err(_) => Err(ShareError::QueryFail)
                }
            }
            Err(_) => Err(ShareError::QueryFail)
        }
    }

    async fn get_share(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<Share, Self::Error> {
        let collection = self.db.collection::<Share>(CV_SHARE_COLLECTION);
        let filter = bson::doc!{
            "_id.user_id": user_id,
            "_id.cv_id": cv_id
        };
        let result = collection.find_one(filter, None).await;
        match result{
            Ok(share_option) => {
                match share_option {
                    Some(share) => Ok(share),
                    None => Err(ShareError::ShareNotFound)
                }
            },
            Err(_) => Err(ShareError::QueryFail)
        }
    }

    async fn get_shares_of_cv(
        &self,
        cv_id: ObjectId,
    ) -> Result<BoxStream<Result<Share, Self::Error>>, Self::Error> {
        let collection = self.db.collection::<Share>(CV_SHARE_COLLECTION);
        let filter = bson::doc!{
            "_id.cv_id": cv_id
        };
        let result = collection.find(filter, None).await;
        match result {
            Ok(cursor) => Ok(cursor.map(|share|Ok(share.unwrap())).boxed()),
            Err(_) => Err(ShareError::QueryFail)
        }
    }

    async fn get_shares_count_of_cv(&self, cv_id: ObjectId) -> Result<i32, Self::Error> {
        let collection = self.db.collection::<Share>(CV_SHARE_COLLECTION);
        let filter = bson::doc!{
            "_id.cv_id": cv_id
        };
        let result = collection.count_documents(filter, None).await;
        match result{
            Ok(count) => Ok(count as i32),
            Err(_) => Err(ShareError::QueryFail) 
        }
    }
}
