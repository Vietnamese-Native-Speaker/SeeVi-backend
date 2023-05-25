use async_graphql::SimpleObject;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct Education {
    pub institution: String,
    pub course: Option<String>,
    pub degree: Option<String>
}