mod error;

use crate::models::{
    comment::Comment,
    cv::{CreateCVInput, UpdateCVInput, CV},
};
use async_graphql::{async_trait::async_trait, futures_util::stream::BoxStream};
use mongodb::bson::oid::ObjectId;

pub use error::CVDataSourceError;

/// Primary abstraction for CV Data Source. Ones should implement this trait for
/// different type of database in order to provide that data source to services
#[async_trait]
pub trait CVDataSource {
    /// Return the CV with the provided id.
    async fn get_cv_by_id(&self, _id: ObjectId) -> Result<CV, CVDataSourceError> {
        unimplemented!()
    }

    /// Add CV with the provided input.
    async fn create_cv(&self, _input: CreateCVInput) -> Result<CV, CVDataSourceError> {
        unimplemented!()
    }

    /// Update the CV with the provided input, using the id in the input as
    /// finder.
    async fn update_cv_info(&self, _input: UpdateCVInput) -> Result<CV, CVDataSourceError> {
        unimplemented!()
    }

    /// Delete the CV with the provided id.
    async fn delete_cv(&self, _id: ObjectId) -> Result<(), CVDataSourceError> {
        unimplemented!()
    }

    /// Add comment to the CV with the provided id.
    async fn add_comment_to_cv(
        &self,
        _cv_id: ObjectId,
        _comment: Comment,
    ) -> Result<CV, CVDataSourceError> {
        unimplemented!()
    }

    /// Remove comment from the CV with the provided id.
    async fn remove_comment_from_cv(
        &self,
        _cv_id: ObjectId,
        _comment_id: ObjectId,
    ) -> Result<CV, CVDataSourceError> {
        unimplemented!()
    }

    async fn find_and_update_cv(
        &self,
        _cv_id: ObjectId,
        _input: UpdateCVInput,
    ) -> Result<CV, CVDataSourceError>;

    async fn get_comments_by_cv_id(
        &self,
        _cv_id: ObjectId,
    ) -> Result<Vec<ObjectId>, CVDataSourceError> {
        unimplemented!()
    }

    async fn get_cvs_by_user_id(
        &self,
        _user_id: ObjectId,
    ) -> Result<BoxStream<Result<CV, CVDataSourceError>>, CVDataSourceError>;
}
