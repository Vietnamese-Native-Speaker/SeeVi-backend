use async_graphql::futures_util::stream::BoxStream;
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

use crate::{models::comment::Like, services::cv_service::comment_service::CommentServiceError};

#[async_trait]
pub trait LikeDataSource {
    type Error: std::error::Error + Send + Sync + Into<CommentServiceError>;

    async fn add_like(&self, user_id: ObjectId, comment_id: ObjectId) -> Result<(), Self::Error>;

    async fn delete_like(&self, user_id: ObjectId, comment_id: ObjectId) -> Result<(), Self::Error>;

    async fn get_likes_count(&self, comment_id: ObjectId) -> Result<i32, Self::Error>;

    async fn get_likes(&self, comment_id: ObjectId) -> Result<BoxStream<Like>, Self::Error>;
}
