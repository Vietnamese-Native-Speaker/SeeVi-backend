use async_graphql::InputObject;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

/// CV InputObject for CV creation query in GraphQL.
#[derive(Serialize, Deserialize, Clone, InputObject)]
pub struct CreateCVInput {
    pub author_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
}
