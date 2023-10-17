//! Implements the `BookmarkDataSource` trait for `MongoDB`.

use std::fmt::Display;

use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;

use crate::{
    data_source::cv::bookmark::BookmarkDataSource,
    models::cv::{Bookmark, CV},
    services::cv_service::error::CVServiceError,
};

use super::MongoDB;

/// Error type for `BookmarkDataSource` operations.
#[derive(Debug)]
pub enum BookmarkError {}

impl Display for BookmarkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for BookmarkError {}

impl From<BookmarkError> for CVServiceError {
    fn from(_: BookmarkError) -> Self {
        todo!()
    }
}

#[async_trait::async_trait]
impl BookmarkDataSource for MongoDB {
    type Error = BookmarkError;
    async fn add_bookmark(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        todo!()
    }

    async fn delete_bookmark(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        todo!()
    }

    async fn get_bookmarks_of_user(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Bookmark>, Self::Error> {
        todo!()
    }

    async fn get_bookmark(
        &self,
        user_id: ObjectId,
        cv_id: ObjectId,
    ) -> Result<Bookmark, Self::Error> {
        todo!()
    }

    async fn get_bookmarks_of_cv(
        &self,
        cv_id: ObjectId,
    ) -> Result<BoxStream<Result<Bookmark, Self::Error>>, Self::Error> {
        todo!()
    }

    async fn get_bookmarked_cvs_of_user(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Result<CV, Self::Error>>, Self::Error> {
        todo!()
    }

    /// This default implementation is not efficient, reimplement it if you can.
    async fn get_bookmarks_count_of_cv(&self, cv_id: ObjectId) -> Result<u64, Self::Error> {
        todo!()
    }
}
