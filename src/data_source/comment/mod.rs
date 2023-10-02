use async_graphql::futures_util::stream::BoxStream;
use async_trait::async_trait;
use mongodb::bson;

use crate::{models::comment::{Comment, CreateCommentInput, UpdateCommentInput}, services::cv_service::comment_service::CommentServiceError};

#[async_trait]
pub trait CommentDataSource {
    type Error: std::error::Error + Send + Sync + Into<CommentServiceError>;
    async fn get_comment_by_id(&self, _id: bson::oid::ObjectId) -> Result<Comment, Self::Error>;

    async fn get_comments_by_cv_id(
        &self,
        _cv_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<Comment, Self::Error>>;

    async fn create_comment(&self, _input: CreateCommentInput) -> Result<Comment, Self::Error>;

    async fn add_comment(&self, _comment: Comment) -> Result<(), Self::Error>;

    async fn update_comment(&self, _input: UpdateCommentInput) -> Result<Comment, Self::Error>;

    async fn delete_comment(&self, _id: bson::oid::ObjectId) -> Result<Comment, Self::Error>;

    async fn add_reply_to_comment(
        &self,
        _comment_id: bson::oid::ObjectId,
        _reply_id: bson::oid::ObjectId,
    ) -> Result<Comment, Self::Error>;

    async fn remove_reply_from_comment(
        &self,
        _comment_id: bson::oid::ObjectId,
        _reply_id: bson::oid::ObjectId,
    ) -> Result<Comment, Self::Error>;
}
