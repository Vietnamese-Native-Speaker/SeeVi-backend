use futures_core::stream::BoxStream;
use futures_core::Stream;
use tokio_stream::StreamExt;

use crate::data_source::cv_data_source::CVDataSource;
use crate::data_source::cv_data_source_error::CVDataSourceError;
use crate::data_source::user_data_source::UserDataSource;
use crate::models::cv::{CreateCVInput, CV};
use std::boxed::Box;
use std::fmt::Debug;
use std::pin::Pin;

pub struct CVService;

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

    pub async fn find_suggested_cvs(
        database: &(impl CVDataSource + std::marker::Sync),
    ) -> BoxStream<CV> {
        let stream = database.get_recommended_cvs().await;
        Pin::from(Box::new(stream.map(|rs| rs.unwrap())))
    }
}
