use async_graphql::futures_util::stream::BoxStream;
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

use crate::{models::cv::interactions::Share, services::cv_service::error::CVServiceError};

#[async_trait]
pub trait ShareDataSource {
    type Error: std::error::Error + Send + Sync + Into<CVServiceError>;

    async fn add_share(&self, user_id: ObjectId, comment_id: ObjectId) -> Result<(), Self::Error>;

    async fn delete_share(
        &self,
        user_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<(), Self::Error>;

    async fn get_shares_by_user_id(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Share>, Self::Error>;

    async fn get_share(
        &self,
        user_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<Share, Self::Error>;
}
