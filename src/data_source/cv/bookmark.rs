use async_graphql::futures_util::{stream::BoxStream, StreamExt};
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

use crate::{
    models::cv::{CvBookmark, CV},
    services::cv_service::error::CVServiceError,
};

#[async_trait]
pub trait BookmarkDataSource {
    type Error: std::error::Error + Send + Sync + Into<CVServiceError>;

    async fn add_bookmark(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error>;

    async fn delete_bookmark(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error>;

    async fn get_bookmarks_of_user(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<CvBookmark>, Self::Error>;

    async fn get_bookmark(
        &self,
        user_id: ObjectId,
        cv_id: ObjectId,
    ) -> Result<CvBookmark, Self::Error>;

    async fn get_bookmarks_of_cv(
        &self,
        cv_id: ObjectId,
    ) -> Result<BoxStream<Result<CvBookmark, Self::Error>>, Self::Error>;

    async fn get_bookmarked_cvs_of_user(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Result<CV, Self::Error>>, Self::Error>;

    /// This default implementation is not efficient, reimplement it if you can.
    async fn get_bookmarks_count_of_cv(&self, cv_id: ObjectId) -> Result<u64, Self::Error> {
        Ok(self.get_bookmarks_of_cv(cv_id).await?.count().await as u64)
    }
}
