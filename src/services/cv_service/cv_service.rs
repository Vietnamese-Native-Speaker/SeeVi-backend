use crate::data_source::cv_data_source_error::CVDataSourceError;
use crate::models::cv::CV;
use std::boxed::Box;
use std::fmt::Debug;

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
    pub async fn find_suggested_cvs(&self) -> Result<Vec<CV>, CVDataSourceError> {
        todo!()
    }
}
