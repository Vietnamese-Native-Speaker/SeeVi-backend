use async_graphql::InputObject;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, InputObject, Builder, Default)]
#[builder(
    pattern = "owned",
    setter(into, prefix = "with", strip_option),
)]
pub struct UpdateCVInput {
    #[builder(default)]
    pub id: Uuid,
    #[builder(default)]
    pub author_id: Uuid,
    #[builder(default)]
    pub title: Option<String>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub tags: Option<Vec<String>>,
}

impl UpdateCVInput {
    pub fn builder() -> UpdateCVInputBuilder {
        UpdateCVInputBuilder::default()
    }
}
