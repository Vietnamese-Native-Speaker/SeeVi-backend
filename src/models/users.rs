use async_graphql::{SimpleObject, InputObject};
use serde::{Serialize, Deserialize};
use mongodb::bson::uuid::Uuid;

use super::education::Education;

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct User {
    pub user_id: Uuid,
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
    pub education: Vec<Education>
}

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
    pub education: Vec<Education>
}

#[derive(Serialize, Deserialize, Clone, InputObject)]
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country: Option<String>,
    pub skills: Option<String>,
    pub primary_email: Option<String>,
    pub about: Option<String>,
    pub education: Option<Vec<Education>>
}