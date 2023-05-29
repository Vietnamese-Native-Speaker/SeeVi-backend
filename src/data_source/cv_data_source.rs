use mongodb::bson::Uuid;

use crate::models::cv::{CV, CreateCVInput};

/// Primary abstraction for CV Data Source. Ones should implement this trait for
/// different type of database in order to provide that data source to services
trait CVDataSource {
    /// Return the CV with the provided id.
    fn get_cv_by_id(&self, id: Uuid) -> Result<CV, CVDataSourceError> {
        unimplemented!()
    }

    /// Add CV with the provided input.
    fn create_cv(&self, input: CreateCVInput) -> Result<(), CVDataSourceError> {
        unimplemented!()
    }

    /// Update the CV with the provided input, using the id in the input as
    /// finder.
    fn update_cv_info(&self, input: CV) -> Result<CV, CVDataSourceError> {
        unimplemented!()
    }

    /// Delete the CV with the provided id.
    fn delete_cv(&self, id: Uuid) -> Result<(), CVDataSourceError> {
        unimplemented!()
    }
}

pub enum CVDataSourceError {}
