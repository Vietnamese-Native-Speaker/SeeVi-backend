use async_graphql::futures_util::{stream::BoxStream, task::SpawnExt, StreamExt};
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

use crate::{
    models::cv::{interactions::Share, CV},
    services::cv_service::error::CVServiceError,
};

use super::CVDataSource;

#[async_trait]
pub trait ShareDataSource: CVDataSource {
    type Error: std::error::Error + Send + Sync + Into<CVServiceError>;

    async fn add_share(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error>;

    async fn delete_share(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error>;

    async fn get_shares_by_user_id(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Share>, Self::Error>;

    async fn get_shared_cvs_by_user_id(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Result<CV, Self::Error>>, Self::Error>;

    async fn get_share(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<Share, Self::Error>;

    async fn get_shares_of_cv(
        &self,
        cv_id: ObjectId,
    ) -> Result<BoxStream<Result<Share, Self::Error>>, Self::Error>;

    /// The default implementation might not be efficient, reimplementation is recommended.
    async fn get_shares_count_of_cv(&self, cv_id: ObjectId) -> Result<i32, Self::Error> {
        let count = self.get_shares_of_cv(cv_id).await?.count().await;
        Ok(count as i32)
    }
}
