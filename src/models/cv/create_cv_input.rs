use async_graphql::InputObject;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

/// CV InputObject for CV creation query in GraphQL.
#[derive(Serialize, Deserialize, Clone, InputObject, Builder)]
#[builder(pattern = "owned")]
#[builder(setter(prefix = "with", strip_option))]
pub struct CreateCVInput {
    pub author_id: Uuid,
    pub title: String,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(setter(custom), field(type = "Vec<String>"))]
    pub tags: Vec<String>,
}

impl CreateCVInput {
    // This method will help users to discover the builder
    pub fn builder() -> CreateCVInputBuilder {
        CreateCVInputBuilder::default()
    }
}

impl CreateCVInputBuilder {
    pub fn new(author_id: Uuid, title: String) -> Self {
        CreateCVInputBuilder {
            author_id: Some(author_id),
            title: Some(title),
            description: None,
            tags: vec![],
        }
    }

    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
}
