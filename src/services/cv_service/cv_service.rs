use crate::data_source::{
    cv_data_source_error::CVDataSourceError, 
    cv_data_source::CVDataSource,
    cv_details_data_source::CVDetailsDataSource,
    cv_details_data_source_error::CVDetailsDataSourceError};
use crate::models::cv::CV;
use crate::models::cv_details::CVDetails;
use std::boxed::Box;
use std::fmt::Debug;
use futures_core::stream::BoxStream;
use std::pin::Pin;
use tokio_stream::Stream;

pub struct CVService {}

impl CVService {
    pub async fn change_title(&mut self, title: String) -> Result<CV, CVDataSourceError> {
        todo!()
    }
    pub async fn change_item_interactions(&mut self) -> Result<CV, CVDataSourceError> {
        todo!()
    }
    pub async fn change_description(
        &mut self,
        _description: String,
    ) -> Result<CV, CVDataSourceError> {
        todo!()
    }
    pub async fn add_tags(&mut self, _tag: String) -> Result<CV, CVDataSourceError> {
        todo!()
    }
    pub async fn remove_tags(&mut self, _tag: String) -> Result<CV, CVDataSourceError> {
        todo!()
    }

    // NOTE: The return type should `Stream<Item = CV>`
    pub async fn find_suggested_cvs<'a>(&'a self, database: &'a(impl CVDetailsDataSource + std::marker::Sync), cv_details: CVDetails) -> Result<Pin<Box<dyn Stream<Item = CV>>>, CVDetailsDataSourceError> {
        database.get_cvs_by_filter(cv_details).await
    }
}
