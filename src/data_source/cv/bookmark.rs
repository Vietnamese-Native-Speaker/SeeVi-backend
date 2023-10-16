use async_graphql::futures_util::stream::BoxStream;
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

use crate::{models::cv::Bookmark, services::cv_service::error::CVServiceError};

#[async_trait]
pub trait BookmarkDataSource {
    type Error: std::error::Error + Send + Sync + Into<CVServiceError>;

    async fn add_bookmark(
        &self,
        user_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<(), Self::Error>;

    async fn delete_bookmark(
        &self,
        user_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<(), Self::Error>;

    async fn get_bookmark_of_user(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Bookmark>, Self::Error>;

    async fn get_bookmark(
        &self,
        user_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<Bookmark, Self::Error>;
}
