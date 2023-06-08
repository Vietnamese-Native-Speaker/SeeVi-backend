use async_graphql::InputObject;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::models::education::Education;
use derive_builder::Builder;

/// An InputObject for User creation query in GraphQL.
#[derive(Serialize, Deserialize, Clone, InputObject, Builder)]
#[builder(pattern = "owned")]
#[builder(setter(prefix = "with", strip_option))]
pub struct CreateUserInput {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub country: Option<String>,
    pub skills: Option<String>,
    #[builder(setter(custom), field(type = "Vec<Uuid>"))]
    pub cv: Vec<Uuid>,
    pub primary_email: String,
    #[builder(setter(custom), field(type = "Vec<String>"))]
    pub other_mails: Vec<String>,
    pub about: Option<String>,
    pub avatar: Option<Uuid>,
    pub cover_photo: Option<Uuid>,
    #[builder(setter(custom), field(type = "Vec<Uuid>"))]
    pub friends_list: Vec<Uuid>,
    #[builder(setter(custom), field(type = "Vec<Education>"))]
    pub education: Vec<Education>,
}

impl CreateUserInput {
    pub fn builder() -> CreateUserInputBuilder {
        CreateUserInputBuilder::default()
    }
}

impl CreateUserInputBuilder {
    pub fn with_cv(mut self, cv: Uuid) -> Self {
        self.cv.push(cv);
        self
    }

    pub fn with_other_mail(mut self, other_mails: String) -> Self {
        self.other_mails.push(other_mails);
        self
    }

    pub fn with_friend(mut self, friend: Uuid) -> Self {
        self.friends_list.push(friend);
        self
    }

    pub fn with_education(mut self, education: Education) -> Self {
        self.education.push(education);
        self
    }
}
