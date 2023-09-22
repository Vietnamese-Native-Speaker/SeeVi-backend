use mongodb::bson::Uuid;

pub mod users;
pub mod cv;
pub mod education;
pub mod comment;
pub mod friend_request;

pub type ResourceIdentifier = Uuid;

