use async_graphql::futures_util::stream::BoxStream;
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

use crate::{
    models::comment::Bookmark, services::cv_service::comment_service::CommentServiceError,
};

#[async_trait]
pub trait BookmarkDataSource {
    type Error: std::error::Error + Send + Sync + Into<CommentServiceError>;

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

    async fn get_bookmarks_of_user(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Result<Bookmark, Self::Error>>, Self::Error>;

    async fn get_bookmark(
        &self,
        user_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<Option<Bookmark>, Self::Error>;

    async fn get_bookmarks_count(&self, comment_id: ObjectId) -> Result<i32, Self::Error>;
}
