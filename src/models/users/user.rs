use async_graphql::{Enum, SimpleObject};
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::models::{education::Education, ResourceIdentifier};

use super::CreateUserInput;

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Enum, PartialEq, Eq)]
pub enum Level {
    Fresher,
    Junior,
}

/// The User Model struct.
#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, PartialEq)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    #[graphql(skip)]
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub country: Option<String>,
    pub skills: Vec<String>,
    pub cv: Vec<Uuid>,
    pub primary_email: String,
    pub other_mails: Vec<String>,
    pub about: Option<String>,
    pub avatar: Option<ResourceIdentifier>,
    pub cover_photo: Option<ResourceIdentifier>,
    pub friends_list: Vec<Uuid>,
    pub education: Vec<Education>,
    pub rating: Option<f64>,
    pub level: Option<Level>,
    pub shared_cvs: Vec<Uuid>,
    pub saved_cvs: Vec<Uuid>,
    pub liked_cvs: Vec<Uuid>,
}

impl From<CreateUserInput> for User {
    fn from(input: CreateUserInput) -> Self {
        Self {
            user_id: Uuid::new(),
            username: input.username,
            password: input.password,
            first_name: input.first_name.unwrap_or("None".to_string()),
            last_name: input.last_name.unwrap_or("None".to_string()),
            country: input.country,
            skills: input.skills,
            primary_email: input.primary_email,
            other_mails: input.other_mails,
            about: input.about,
            avatar: input.avatar,
            cover_photo: input.cover_photo,
            education: input.education,
            rating: input.rating,
            level: input.level,
            cv: Vec::default(),
            shared_cvs: Vec::default(),
            saved_cvs: Vec::default(),
            liked_cvs: Vec::default(),
            friends_list: Vec::default(),
        }
    }
}

