use mongodb::bson::Uuid;

pub mod temp;
mod user_service;

type ResourceIdentifier = Uuid;
#[cfg(test)]
mod tests;