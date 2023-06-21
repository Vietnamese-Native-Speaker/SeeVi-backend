use mongodb::bson::Uuid;

pub mod temp;
pub mod user_service;

type ResourceIdentifier = Uuid;
#[cfg(test)]
mod tests;
pub mod storage_service;
