use async_graphql::async_trait::async_trait;
use mongodb::bson::Uuid;

use crate::models::cv::{CreateCVInput, CV};
use crate::models::cv_details::{CVDetails};
use super::cv_data_source_error::CVDataSourceError;
use futures_core::stream::BoxStream;

/// Primary abstraction for CV Data Source. Ones should implement this trait for
/// different type of database in order to provide that data source to services
#[async_trait]
pub trait CVDetailsDataSource {
    /// Return the CV with the provided id.
    async fn get_cvs_by_filter(&self, cv_details: CVDetails) -> BoxStream<Result<CV, CVDataSourceError>> {
        unimplemented!()
    }
}
