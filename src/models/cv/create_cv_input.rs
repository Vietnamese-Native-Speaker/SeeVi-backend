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

impl CreateCVInput {
    // This method will help users to discover the builder
    pub fn builder() -> CreateCVInputBuilder {
        CreateCVInputBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateCVInputBuilder{
    pub author_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

impl CreateCVInputBuilder{
    pub fn new(
        author_id:Uuid,
        title: String,
    ) -> Self{
        CreateCVInputBuilder{
            author_id,
            title,
            description:None,
            tags: vec![],
        }
    }

    pub fn with_author_id(mut self, author_id: Uuid) -> Self {
        self.author_id = author_id;
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn build(self) -> CreateCVInput{
        CreateCVInput{
            author_id: self.author_id,
            title: self.title,
            description: self.description,
            tags: self.tags,
        }
    }
}