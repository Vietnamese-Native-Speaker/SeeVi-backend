use async_graphql::futures_util::{stream::BoxStream, StreamExt};
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

use crate::{models::cv::CvLike, services::cv_service::error::CVServiceError};

#[async_trait]
pub trait LikeDataSource {
    type Error: std::error::Error + Send + Sync + Into<CVServiceError>;

    async fn add_like(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error>;

    async fn delete_like(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error>;

    /// The default implementation might not be efficient, reimplementation is recommended.
    async fn get_likes_count(&self, cv_id: ObjectId) -> Result<i32, Self::Error> {
        let count = self.get_likes(cv_id).await?.count().await;
        Ok(count as i32)
    }

    async fn get_likes(&self, cv_id: ObjectId) -> Result<BoxStream<CvLike>, Self::Error>;
}
