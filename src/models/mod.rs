use mongodb::bson::Uuid;

pub mod users;
pub mod cv;
pub mod education;
pub mod comment;
pub mod cv_details;
pub mod range_values;
pub mod sex;

pub type ResourceIdentifier = Uuid;

