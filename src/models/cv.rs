use async_graphql::{SimpleObject, InputObject};
use serde::{Serialize, Deserialize};
use mongodb::bson::uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct CV {
    pub _id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>
}

#[derive(Serialize, Deserialize, Clone, InputObject)]
pub struct CreateCVInput {
    pub author_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>
}