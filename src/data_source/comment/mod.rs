mod error;

use async_trait::async_trait;
use mongodb::bson::{self, Uuid};
use async_graphql::futures_util::stream::BoxStream;

pub use error::CommentDataSourceError;

use crate::models::{
    comment::{Comment, CreateCommentInput, UpdateCommentInput},
    ResourceIdentifier,
};

#[async_trait]
pub trait CommentDataSource{
    async fn get_comment_by_id(&self, _id: bson::oid::ObjectId) -> Result<Comment, CommentDataSourceError> {
        unimplemented!()
    }

    async fn get_comments_by_cv_id(&self, _cv_id: bson::oid::ObjectId) -> BoxStream<Result<Comment, CommentDataSourceError>> {
        unimplemented!()
    }

    async fn create_comment(&self, _input: CreateCommentInput) -> Result<Comment, CommentDataSourceError> {
        unimplemented!()
    }

    async fn add_comment(&self, _comment: Comment) -> Result<(), CommentDataSourceError> {
        unimplemented!()
    }

    async fn update_comment(&self, _input: UpdateCommentInput) -> Result<Comment, CommentDataSourceError> {
        unimplemented!()
    }

    async fn delete_comment(&self, _id: bson::oid::ObjectId) -> Result<Comment, CommentDataSourceError> {
        unimplemented!()
    }
}