use async_graphql::async_trait::async_trait;
use mongodb::bson::Uuid;

use crate::models::cv::{CreateCVInput, CV};

use super::cv_data_source_error::CVDataSourceError;

/// Primary abstraction for CV Data Source. Ones should implement this trait for
/// different type of database in order to provide that data source to services
#[async_trait]
trait CVDataSource {
    /// Return the CV with the provided id.
    async fn get_cv_by_id(&self, id: Uuid) -> Result<CV, CVDataSourceError> {
        unimplemented!()
    }

    /// Add CV with the provided input.
    async fn create_cv(&self, input: CreateCVInput) -> Result<(), CVDataSourceError> {
        unimplemented!()
    }

    /// Update the CV with the provided input, using the id in the input as
    /// finder.
    async fn update_cv_info(&self, input: CV) -> Result<CV, CVDataSourceError> {
        unimplemented!()
    }

    /// Delete the CV with the provided id.
    async fn delete_cv(&self, id: Uuid) -> Result<(), CVDataSourceError> {
        unimplemented!()
    }
}
