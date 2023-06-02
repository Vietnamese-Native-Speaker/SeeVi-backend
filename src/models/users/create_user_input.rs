use async_graphql::InputObject;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::models::education::Education;

/// An InputObject for User creation query in GraphQL.
#[derive(Serialize, Deserialize, Clone, InputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub country: Option<String>,
    pub skills: Option<String>,
    pub cv: Vec<Uuid>,
    pub primary_email: String,
    pub other_mails: Vec<String>,
    pub about: Option<String>,
    pub avatar: Option<Uuid>,
    pub cover_photo: Option<Uuid>,
    pub friends_list: Vec<Uuid>,
    pub education: Vec<Education>,
}

impl CreateUserInput {
    pub fn builder() -> CreateUserInputBuilder {
        CreateUserInputBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateUserInputBuilder {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub country: Option<String>,
    pub skills: Option<String>,
    pub cv: Vec<Uuid>,
    pub primary_email: String,
    pub other_mails: Vec<String>,
    pub about: Option<String>,
    pub avatar: Option<Uuid>,
    pub cover_photo: Option<Uuid>,
    pub friends_list: Vec<Uuid>,
    pub education: Vec<Education>,
}

impl CreateUserInputBuilder {
    pub fn new(
        username: String,
        first_name: String,
        last_name: String,
        primary_email: String,
    ) -> Self {
        CreateUserInputBuilder {
            username,
            first_name,
            last_name,
            country: None,
            skills: None,
            cv: Vec::new(),
            primary_email,
            other_mails: Vec::new(),
            about: None,
            avatar: None,
            cover_photo: None,
            friends_list: Vec::new(),
            education: Vec::new(),
        }
    }

    pub fn with_country(mut self, country: String) -> Self {
        self.country = Some(country);
        self
    }

    pub fn with_skills(mut self, skills: String) -> Self {
        self.skills = Some(skills);
        self
    }

    pub fn with_cv(mut self, cv: Uuid) -> Self {
        self.cv.push(cv);
        self
    }

    pub fn with_other_mails(mut self, other_mails: String) -> Self {
        self.other_mails.push(other_mails);
        self
    }

    pub fn with_about(mut self, about: String) -> Self {
        self.about = Some(about);
        self
    }

    pub fn with_avatar(mut self, avatar: Uuid) -> Self {
        self.avatar = Some(avatar);
        self
    }

    pub fn with_cover_photo(mut self, cover_photo: Uuid) -> Self {
        self.cover_photo = Some(cover_photo);
        self
    }

    pub fn with_friends_list(mut self, friends_list: Uuid) -> Self {
        self.friends_list.push(friends_list);
        self
    }

    pub fn with_education(mut self, education: Education) -> Self {
        self.education.push(education);
        self
    }

    pub fn build(self) -> CreateUserInput {
        CreateUserInput {
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            country: self.country,
            skills: self.skills,
            cv: self.cv,
            primary_email: self.primary_email,
            other_mails: self.other_mails,
            about: self.about,
            avatar: self.avatar,
            cover_photo: self.cover_photo,
            friends_list: self.friends_list,
            education: self.education,
        }
    }
}