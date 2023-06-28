use async_graphql::{ComplexObject, SimpleObject};
use mongodb::bson::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::models::ResourceIdentifier;

use super::CreateCVInput;

/// Struct represents CV defined in the Diagram. Note that this struct only
/// represents the metadata of a CV. Actual implementation of the CV is to
/// be discussed.
#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, PartialEq)]
#[graphql(complex)]
pub struct CV {
    pub _id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub comments: Vec<Uuid>,
    #[graphql(skip)]
    pub created: DateTime,
    pub cv: Option<ResourceIdentifier>,
}

#[ComplexObject]
impl CV {
    async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}

impl From<CreateCVInput> for CV {
    fn from(input: CreateCVInput) -> Self {
        Self {
            _id: Uuid::new(),
            author_id: input.author_id,
            title: input.title,
            description: input.description,
            tags: input.tags,
            comments: Vec::default(),
            created: DateTime::now(),
            cv: None,
        }
    }
}
