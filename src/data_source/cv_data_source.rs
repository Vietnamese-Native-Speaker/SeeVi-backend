use mongodb::bson::Uuid;

use crate::models::cv::{CV, CreateCVInput};

trait CVDataSource {
    fn get_cv_by_id(&self, id: Uuid) -> Result<CV, CVDataSourceError> {
        unimplemented!()
    }
    fn create_cv(&self, input: CreateCVInput) -> Result<(), CVDataSourceError> {
        unimplemented!()
    }
    fn update_cv_info(&self, input: CV) -> Result<CV, CVDataSourceError> {
        unimplemented!()
    }
    fn delete_cv(&self, id: Uuid) -> Result<(), CVDataSourceError> {
        unimplemented!()
    }
}

pub enum CVDataSourceError {}