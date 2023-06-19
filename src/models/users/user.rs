use async_graphql::{Enum, SimpleObject};
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::models::{education::Education, ResourceIdentifier};

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

