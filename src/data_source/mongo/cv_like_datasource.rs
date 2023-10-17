//! Implements the `cv::LikeDataSource` trait for `MongoDB`.

use std::fmt::Display;

use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;

use crate::{data_source::cv, models::cv::Like, services::cv_service::error::CVServiceError};

use super::MongoDB;

/// Error type for `LikeDataSource` operations.
#[derive(Debug)]
pub enum LikeError {}

impl Display for LikeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for LikeError {}

impl From<LikeError> for CVServiceError {
    fn from(value: LikeError) -> Self {
        todo!()
    }
}

#[async_trait::async_trait]
impl cv::like::LikeDataSource for MongoDB {
    type Error = LikeError;

    async fn add_like(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        todo!()
    }

    async fn delete_like(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        todo!()
    }

    async fn get_likes_count(&self, cv_id: ObjectId) -> Result<i32, Self::Error> {
        todo!()
    }

    async fn get_likes(&self, cv_id: ObjectId) -> Result<BoxStream<Like>, Self::Error> {
        todo!()
    }
}
