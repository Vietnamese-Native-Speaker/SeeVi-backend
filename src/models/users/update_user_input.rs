use async_graphql::InputObject;
use mongodb::bson::{Uuid, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::models::{education::Education, ResourceIdentifier};

/// An InputObject for User update query in GraphQL
#[derive(Serialize, Deserialize, Clone, InputObject, Builder, Default)]
#[builder(
    pattern = "owned",
    setter(into, prefix = "with", strip_option),
)]
pub struct UpdateUserInput {
    pub user_id: ObjectId,
    #[builder(default)]
    pub username: Option<String>,
    #[builder(default)]
    pub password: Option<String>,
    #[builder(default)]
    pub first_name: Option<String>,
    #[builder(default)]
    pub last_name: Option<String>,
    #[builder(default)]
    pub country: Option<String>,
    #[builder(default)]
    pub skills: Option<Vec<String>>,
    #[builder(default)]
    pub primary_email: Option<String>,
    #[builder(default)]
    pub other_mails: Option<Vec<String>>,
    #[builder(default)]
    pub about: Option<String>,
    #[builder(default)]
    pub avatar: Option<ResourceIdentifier>,
    #[builder(default)]
    pub cover_photo: Option<ResourceIdentifier>,
    #[builder(default)]
    pub friends_list: Option<Vec<ResourceIdentifier>>,
    #[builder(default)]
    pub education: Option<Vec<Education>>,
}

impl UpdateUserInput {
    pub fn builder() -> UpdateUserInputBuilder {
        UpdateUserInputBuilder::default()
    }
}
