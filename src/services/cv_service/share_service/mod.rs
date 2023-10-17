#[cfg(test)]
mod tests;

use async_graphql::futures_util::StreamExt;
use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;

use crate::{
    data_source::{cv::share::ShareDataSource, CVDataSource, UserDataSource},
    models::cv::{interactions::Share, CV},
    services::user_service::error::UserServiceError,
};

use super::error::CVServiceError;

pub struct ShareService;

impl ShareService {
    pub async fn share_cv(
        db: &(impl ShareDataSource + UserDataSource + std::marker::Sync),
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
        let cv = match db.get_cv_by_id(cv_id).await {
            Ok(cv) => cv,
            Err(e) => match e {
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
            },
        };
        if cv.author_id == user_id.into() {
            return Err(CVServiceError::LikeFailed(
                "You can't share your own CV".to_string(),
            ));
        }
        db.add_share(user_id, cv_id).await.map_err(|e| e.into())
    }

    pub async fn unshare_cv(
        db: &(impl ShareDataSource + UserDataSource + CVDataSource + std::marker::Sync),
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
        db.delete_share(user_id, cv_id).await.map_err(|e| e.into())
    }

    pub async fn get_shared_cvs_of_user(
        db: &(impl ShareDataSource + UserDataSource + std::marker::Sync),
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
        db.get_shared_cvs_by_user_id(user_id)
            .await
            .map_err(|e| e.into())
            .map(|stream| stream.map(|item| item.map_err(|err| err.into())).boxed())
    }

    pub async fn get_shares_of_cv(
        db: &(impl ShareDataSource + CVDataSource + std::marker::Sync),
        cv_id: ObjectId,
    ) -> Result<BoxStream<Result<Share, CVServiceError>>, CVServiceError> {
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
        db.get_shares_of_cv(cv_id)
            .await
            .map_err(|e| e.into())
            .map(|stream| stream.map(|item| item.map_err(|err| err.into())).boxed())
    }

    pub async fn get_shares_count_of_cv(
        db: &(impl ShareDataSource + CVDataSource + std::marker::Sync),
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
        db.get_shares_count_of_cv(cv_id)
            .await
            .map_err(|e| e.into())
            .map(|count| count as u64)
    }
}
