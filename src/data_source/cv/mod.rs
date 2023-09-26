mod error;

use async_graphql::async_trait::async_trait;
use mongodb::bson::{oid::ObjectId, Uuid};
use crate::models::cv::{CreateCVInput, UpdateCVInput, CV};

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
}