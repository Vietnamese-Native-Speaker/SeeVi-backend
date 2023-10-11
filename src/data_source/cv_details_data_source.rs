use async_graphql::async_trait::async_trait;
use mongodb::bson::Uuid;
use std::pin::Pin;
use crate::models::cv::{CreateCVInput, CV};
use crate::models::cv_details::{CVDetails};
use super::cv_details_data_source_error::CVDetailsDataSourceError;
use futures_core::stream::BoxStream;
use tokio_stream::Stream;

#[async_trait]
pub trait CVDetailsDataSource {
    /// Return the CV with the provided filter.
    async fn get_cvs_by_filter(&self, cv_details: CVDetails) -> Result<Pin<Box<dyn Stream<Item = CV>>>, CVDetailsDataSourceError> {
        unimplemented!()
    }
}
