use async_graphql::async_trait::async_trait;
use futures_core::stream::{Stream, BoxStream};
use mongodb::bson::Uuid;

use crate::models::cv::{CreateCVInput, CV};

use super::cv_data_source_error::CVDataSourceError;

/// Primary abstraction for CV Data Source. Ones should implement this trait for
/// different type of database in order to provide that data source to services
#[async_trait]
pub trait CVDataSource {
    /// Return the CV with the provided id.
    async fn get_cv_by_id(&self, _id: Uuid) -> Result<CV, CVDataSourceError> {
        unimplemented!()
    }

    /// Add CV with the provided input.
    async fn create_cv(&self, _input: CreateCVInput) -> Result<CV, CVDataSourceError> {
        unimplemented!()
    }

    async fn get_recommended_cvs(&self) -> BoxStream<Result<CV, CVDataSourceError>> {
        unimplemented!()
    }

    /// Update the CV with the provided input, using the id in the input as
    /// finder.
    async fn update_cv_info(&self, _input: CV) -> Result<CV, CVDataSourceError> {
        unimplemented!()
    }

    /// Delete the CV with the provided id.
    async fn delete_cv(&self, _id: Uuid) -> Result<(), CVDataSourceError> {
        unimplemented!()
    }
}
