use mongodb::bson::Uuid;

pub mod user_service;
pub mod storage_service;
pub mod cv_service;
pub mod auth_service;

type ResourceIdentifier = Uuid;
#[cfg(test)]
mod tests;
