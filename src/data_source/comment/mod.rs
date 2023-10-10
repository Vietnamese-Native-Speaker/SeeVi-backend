use async_graphql::futures_util::stream::BoxStream;
use async_trait::async_trait;
use mongodb::bson::{self, oid::ObjectId};

use crate::{models::comment::{Comment, CreateCommentInput, UpdateCommentInput}, services::cv_service::comment_service::CommentServiceError};

#[async_trait]
pub trait CommentDataSource {
    type Error: std::error::Error + Send + Sync + Into<CommentServiceError>;
    async fn get_comment_by_id(&self, id: bson::oid::ObjectId) -> Result<Comment, Self::Error>;

    async fn get_comments_list(
        &self,
        ids: Vec<ObjectId>
    ) -> BoxStream<Result<Comment, Self::Error>>;

    async fn create_comment(&self, input: CreateCommentInput) -> Result<Comment, Self::Error>;

    async fn add_comment(&self, comment: Comment) -> Result<(), Self::Error>;

    async fn update_comment(&self, input: UpdateCommentInput) -> Result<Comment, Self::Error>;

    async fn remove_comment(&self, id: bson::oid::ObjectId) -> Result<Comment, Self::Error>;

    async fn find_and_update_comment(
        &self,
        id: bson::oid::ObjectId,
        input: UpdateCommentInput,
    ) -> Result<Comment, Self::Error>;

    async fn add_reply_to_comment(
        &self,
        comment_id: bson::oid::ObjectId,
        reply_id: bson::oid::ObjectId,
    ) -> Result<Comment, Self::Error>;

    async fn remove_reply_from_comment(
        &self,
        comment_id: bson::oid::ObjectId,
        reply_id: bson::oid::ObjectId,
    ) -> Result<Comment, Self::Error>;

    async fn find_and_remove_reply(
        &self,
        comment_id: bson::oid::ObjectId,
        reply_id: bson::oid::ObjectId,
    ) -> Result<Comment, Self::Error>;
}
