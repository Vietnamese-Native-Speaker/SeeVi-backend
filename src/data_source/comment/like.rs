use async_trait::async_trait;
use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;

use crate::{models::comment::Like, services::cv_service::comment_service::CommentServiceError};

#[async_trait]
pub trait LikeDataSource {
    type Error: std::error::Error + Send + Sync + Into<CommentServiceError>;

    fn add_like(&self, user_id: ObjectId, comment_id: ObjectId) -> Result<(), Self::Error>;

    fn delete_like(&self, user_id: ObjectId, comment_id: ObjectId) -> Result<(), Self::Error>;

    fn get_likes_count(&self, comment_id: ObjectId) -> Result<i32, Self::Error>;

    fn get_likes(&self, comment_id: ObjectId) -> Result<BoxStream<Like>, Self::Error>;
}
