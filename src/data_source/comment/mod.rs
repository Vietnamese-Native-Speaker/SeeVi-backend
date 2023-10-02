mod error;

use async_graphql::futures_util::stream::BoxStream;
use async_trait::async_trait;
use google_cloud_storage::http::Error;
use mongodb::bson::{self, Uuid};

pub use error::CommentDataSourceError;

use crate::models::comment::{Comment, CreateCommentInput, UpdateCommentInput};

#[async_trait]
pub trait CommentDataSource {
    type Error;
    async fn get_comment_by_id(&self, _id: bson::oid::ObjectId) -> Result<Comment, Error>;

    async fn get_comments_by_cv_id(
        &self,
        _cv_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<Comment, Error>>;

    async fn create_comment(&self, _input: CreateCommentInput) -> Result<Comment, Error>;

    async fn add_comment(&self, _comment: Comment) -> Result<(), Error>;

    async fn update_comment(&self, _input: UpdateCommentInput) -> Result<Comment, Error>;

    async fn delete_comment(&self, _id: bson::oid::ObjectId) -> Result<Comment, Error>;
}
