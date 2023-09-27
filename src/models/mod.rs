use mongodb::bson::Uuid;

pub mod comment;
pub mod users;
pub mod cv;
pub mod education;
pub mod friend_request;

pub type ResourceIdentifier = Uuid;

