use async_graphql::InputObject;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::models::education::Education;

/// An InputObject for User update query in GraphQL
#[derive(Serialize, Deserialize, Clone, InputObject)]
pub struct UpdateUserInput {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country: Option<String>,
    pub skills: Option<String>,
    pub primary_email: Option<String>,
    pub about: Option<String>,
    pub education: Option<Vec<Education>>,
}

impl UpdateUserInput {
    pub fn builder() -> UpdateUserInputBuilder {
        UpdateUserInputBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateUserInputBuilder {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country: Option<String>,
    pub skills: Option<String>,
    pub primary_email: Option<String>,
    pub about: Option<String>,
    pub education: Option<Vec<Education>>,
}

impl UpdateUserInputBuilder {
    pub fn new(user_id: Uuid) -> Self {
        UpdateUserInputBuilder {
            user_id,
            username: None,
            first_name: None,
            last_name: None,
            country: None,
            skills: None,
            primary_email: None,
            about: None,
            education: None, 
        }
    }

    pub fn with_username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn with_first_name(mut self, first_name: String) -> Self {
        self.first_name = Some(first_name);
        self
    }

    pub fn with_last_name(mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn with_country(mut self, country: String) -> Self {
        self.country = Some(country);
        self
    }

    pub fn with_skills(mut self, skills: String) -> Self {
        self.skills = Some(skills);
        self
    }

    pub fn with_primary_email(mut self, primary_email: String) -> Self {
        self.primary_email = Some(primary_email);
        self
    }

    pub fn with_about(mut self, about: String) -> Self {
        self.about = Some(about);
        self
    }

    pub fn with_education(mut self, education: Vec<Education>) -> Self {
        self.education = Some(education);
        self
    }

    pub fn build(self) -> UpdateUserInput {
        UpdateUserInput{
            user_id: self.user_id,
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            country: self.country,
            skills: self.skills,
            primary_email: self.primary_email,
            about: self.about,
            education: self.education,
        }
    }
}