#[cfg(test)]
mod tests;
use async_graphql::futures_util::StreamExt;
use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;

use crate::{
    data_source::{cv::bookmark::BookmarkDataSource, CVDataSource, UserDataSource},
    models::cv::CV,
    services::user_service::error::UserServiceError,
};

use super::error::CVServiceError;

pub struct BookmarkService;

impl BookmarkService {
    pub async fn bookmark_cv(
        db: &(impl BookmarkDataSource + UserDataSource + CVDataSource + std::marker::Sync),
        user_id: ObjectId,
        cv_id: ObjectId,
    ) -> Result<(), CVServiceError> {
        if let Err(e) = db.get_user_by_id(user_id).await.map_err(|e| e.into()) {
            match e {
                UserServiceError::IdNotFound(_) => {
                    return Err(CVServiceError::LikeFailed("User not found".to_string()));
                }
                UserServiceError::DatabaseError => return Err(CVServiceError::DatabaseError),
                _ => {
                    return Err(CVServiceError::LikeFailed(
                        "Something went wrong".to_string(),
                    ));
                }
            }
        }
        if let Err(e) = db.get_cv_by_id(cv_id).await {
            match e {
                crate::data_source::CVDataSourceError::IdNotFound(_) => {
                    return Err(CVServiceError::LikeFailed("CV not found".to_string()));
                }
                crate::data_source::CVDataSourceError::DatabaseError => {
                    return Err(CVServiceError::DatabaseError)
                }
                _ => {
                    return Err(CVServiceError::LikeFailed(
                        "Something went wrong".to_string(),
                    ));
                }
            }
        }
        db.add_bookmark(user_id, cv_id).await.map_err(|e| e.into())
    }

    pub async fn unbookmark_cv(
        db: &(impl BookmarkDataSource + UserDataSource + CVDataSource + std::marker::Sync),
        user_id: ObjectId,
        cv_id: ObjectId,
    ) -> Result<(), CVServiceError> {
        if let Err(e) = db.get_user_by_id(user_id).await.map_err(|e| e.into()) {
            match e {
                UserServiceError::IdNotFound(_) => {
                    return Err(CVServiceError::LikeFailed("User not found".to_string()));
                }
                UserServiceError::DatabaseError => return Err(CVServiceError::DatabaseError),
                _ => {
                    return Err(CVServiceError::LikeFailed(
                        "Something went wrong".to_string(),
                    ));
                }
            }
        }
        if let Err(e) = db.get_cv_by_id(cv_id).await {
            match e {
                crate::data_source::CVDataSourceError::IdNotFound(_) => {
                    return Err(CVServiceError::LikeFailed("CV not found".to_string()));
                }
                crate::data_source::CVDataSourceError::DatabaseError => {
                    return Err(CVServiceError::DatabaseError)
                }
                _ => {
                    return Err(CVServiceError::LikeFailed(
                        "Something went wrong".to_string(),
                    ));
                }
            }
        }
        db.delete_bookmark(user_id, cv_id)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_bookmarked_cvs_of_user(
        db: &(impl BookmarkDataSource + UserDataSource + std::marker::Sync),
        user_id: ObjectId,
    ) -> Result<BoxStream<Result<CV, CVServiceError>>, CVServiceError> {
        if let Err(e) = db.get_user_by_id(user_id).await.map_err(|e| e.into()) {
            match e {
                UserServiceError::IdNotFound(_) => {
                    return Err(CVServiceError::LikeFailed("User not found".to_string()));
                }
                UserServiceError::DatabaseError => return Err(CVServiceError::DatabaseError),
                _ => {
                    return Err(CVServiceError::LikeFailed(
                        "Something went wrong".to_string(),
                    ));
                }
            }
        }
        db.get_bookmarked_cvs_of_user(user_id)
            .await
            .map_err(|e| e.into())
            .map(|stream| stream.map(|item| item.map_err(|e| e.into())).boxed())
    }

    pub async fn get_bookmark_count_of_cv(
        db: &(impl BookmarkDataSource + CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
    ) -> Result<u64, CVServiceError> {
        if let Err(e) = db.get_cv_by_id(cv_id).await {
            match e {
                crate::data_source::CVDataSourceError::IdNotFound(_) => {
                    return Err(CVServiceError::LikeFailed("CV not found".to_string()));
                }
                crate::data_source::CVDataSourceError::DatabaseError => {
                    return Err(CVServiceError::DatabaseError)
                }
                _ => {
                    return Err(CVServiceError::LikeFailed(
                        "Something went wrong".to_string(),
                    ));
                }
            }
        }
        db.get_bookmarks_count_of_cv(cv_id)
            .await
            .map_err(|e| e.into())
    }
}
