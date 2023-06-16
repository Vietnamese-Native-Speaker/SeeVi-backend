use async_graphql::InputObject;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::models::education::Education;
use derive_builder::Builder;

use super::user::Level;

/// An InputObject for User creation query in GraphQL.
#[derive(Serialize, Deserialize, Clone, InputObject, Builder)]
#[builder(pattern = "owned", setter(into, prefix = "with", strip_option))]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub country: Option<String>,
    #[builder(setter(custom), field(type = "Vec<String>"))]
    pub skills: Vec<String>,
    pub primary_email: String,
    #[builder(setter(custom), field(type = "Vec<String>"))]
    pub other_mails: Vec<String>,
    #[builder(default)]
    pub about: Option<String>,
    #[builder(default)]
    pub avatar: Option<Uuid>,
    #[builder(default)]
    pub cover_photo: Option<Uuid>,
    #[builder(setter(custom), field(type = "Vec<Education>"))]
    pub education: Vec<Education>,
    #[builder(default)]
    pub rating: Option<f64>,
    #[builder(default)]
    pub level: Option<Level>,
}

impl CreateUserInput {
    pub fn builder() -> CreateUserInputBuilder {
        CreateUserInputBuilder::default()
    }
}

impl CreateUserInputBuilder {
    pub fn with_other_mail(mut self, other_mails: impl Into<String>) -> Self {
        self.other_mails.push(other_mails.into());
        self
    }
    pub fn with_education(mut self, education: Education) -> Self {
        self.education.push(education);
        self
    }

    pub fn with_skill<T: Into<String>>(mut self, skill: T) -> Self {
        self.skills.push(skill.into());
        self
    }
}
