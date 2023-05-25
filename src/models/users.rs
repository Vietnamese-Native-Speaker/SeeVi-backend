use async_graphql::SimpleObject;
use serde::{Serialize, Deserialize};
use mongodb::bson::{uuid::Uuid, oid::ObjectId};

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
    pub other_mails: Option<Vec<String>>,
    pub about: Option<String>,
    pub avatar: Option<ObjectId>,
    pub wallpaper: Option<ObjectId>,
    pub friends_list: Vec<Uuid>,
    pub education: Vec<Education>
}