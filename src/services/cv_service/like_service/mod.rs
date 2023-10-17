#[cfg(test)]
mod tests;
use async_graphql::futures_util::StreamExt;
use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;

use crate::{
    data_source::{cv::like::LikeDataSource, CVDataSource, UserDataSource},
    models::cv::Like,
    services::user_service::error::UserServiceError,
};

use super::error::CVServiceError;

pub struct LikeService;

impl LikeService {
    pub async fn like_cv(
        db: &(impl LikeDataSource + UserDataSource + CVDataSource + std::marker::Sync),
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
        db.add_like(user_id, cv_id).await.map_err(|e| e.into())
    }

    pub async fn unlike_cv(
        db: &(impl LikeDataSource + UserDataSource + CVDataSource + std::marker::Sync),
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
        db.delete_like(user_id, cv_id).await.map_err(|e| e.into())
    }

    pub async fn get_likes_by_cv(
        db: &(impl LikeDataSource + UserDataSource + CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
    ) -> Result<BoxStream<Result<Like, CVServiceError>>, CVServiceError> {
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
        db.get_likes(cv_id)
            .await
            .map_err(|e| e.into())
            .map(|stream| stream.map(|item| Ok(item)).boxed())
    }

    pub async fn get_likes_count(
        db: &(impl LikeDataSource + UserDataSource + CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
    ) -> Result<i32, CVServiceError> {
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
        db.get_likes_count(cv_id).await.map_err(|e| e.into())
    }
}
