use mongodb::bson::Uuid;

pub mod users;
pub mod cv;
pub mod education;
pub mod comment;

pub type ResourceIdentifier = Uuid;

