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
