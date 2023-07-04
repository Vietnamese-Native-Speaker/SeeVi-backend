use async_graphql::InputObject;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::models::{education::Education, ResourceIdentifier};

#[derive(Serialize, Deserialize, Clone, InputObject, Builder, Default)]
#[builder(
    pattern = "owned",
    setter(into, prefix = "with", strip_option),
)]
pub struct UpdateCVInput {
    #[builder(default)]
    pub _id: Uuid,
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
