use async_graphql::SimpleObject;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

/// Struct represents CV defined in the Diagram. Note that this struct only
/// represents the metadata of a CV. Actual implementation of the CV is to
/// be discussed.
#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct CV {
    pub _id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
}
