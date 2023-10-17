//! Implements the `ShareDataSource` trait for `MongoDB`.

use std::fmt::Display;

use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;

use crate::{
    data_source::cv::share::ShareDataSource,
    models::cv::{interactions::Share, CV},
    services::cv_service::error::CVServiceError,
};

use super::MongoDB;

/// Error type for `LikeDataSource` operations.
#[derive(Debug)]
pub enum ShareError {}

impl Display for ShareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for ShareError {}

impl From<ShareError> for CVServiceError {
    fn from(value: ShareError) -> Self {
        todo!()
    }
}

#[async_trait::async_trait]
impl ShareDataSource for MongoDB {
    type Error = ShareError;

    async fn add_share(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        todo!()
    }

    async fn delete_share(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        todo!()
    }

    async fn get_shares_by_user_id(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Share>, Self::Error> {
        todo!()
    }

    async fn get_shared_cvs_by_user_id(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Result<CV, Self::Error>>, Self::Error> {
        todo!()
    }

    async fn get_share(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<Share, Self::Error> {
        todo!()
    }

    async fn get_shares_of_cv(
        &self,
        cv_id: ObjectId,
    ) -> Result<BoxStream<Result<Share, Self::Error>>, Self::Error> {
        todo!()
    }

    async fn get_shares_count_of_cv(&self, cv_id: ObjectId) -> Result<i32, Self::Error> {
        todo!()
    }
}
