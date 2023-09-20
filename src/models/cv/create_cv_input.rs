use async_graphql::InputObject;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// CV InputObject for CV creation query in GraphQL.
#[derive(Serialize, Deserialize, Clone, InputObject, Builder)]
#[builder(pattern = "owned", setter(into, prefix = "with", strip_option))]
pub struct CreateCVInput {
    pub author_id: ObjectId,
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
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}
