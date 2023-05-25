use async_graphql::SimpleObject;
use serde::{Serialize, Deserialize};
use mongodb::bson::uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct CV {
    pub _id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>
}
