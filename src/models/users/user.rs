use async_graphql as gql;
use async_graphql::{connection, Context, Enum, SimpleObject};
use gql::futures_util::StreamExt;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::models::cv::CV;
use crate::{
    data_source::mongo::{MongoDB, MongoForTesting},
    services::cv_service::cv_service::CVService,
    services::user_service::UserService,
};
use crate::{
    models::{education::Education, sex::Sex, ResourceIdentifier},
    object_id::ScalarObjectId,
};

use super::CreateUserInput;

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Enum, PartialEq, Eq)]
pub enum Level {
    Fresher,
    Junior,
}

/// The User Model struct.
#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, PartialEq)]
#[graphql(complex)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ScalarObjectId,
    pub username: String,
    #[graphql(skip)]
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub country: Option<String>,
    pub skills: Vec<String>,
    #[graphql(skip)]
    pub cv: Vec<Uuid>,
    pub primary_email: String,
    pub other_emails: Vec<String>,
    pub about: Option<String>,
    pub avatar: Option<ResourceIdentifier>,
    pub cover_photo: Option<ResourceIdentifier>,
    #[graphql(skip)]
    pub friends_list: Vec<Uuid>,
    pub educations: Vec<Education>,
    pub rating: Option<f64>,
    pub level: Option<Level>,
    pub shared_cvs: Vec<Uuid>,
    pub saved_cvs: Vec<Uuid>,
    pub liked_cvs: Vec<Uuid>,
    pub city: Option<String>,
    pub experiences: Option<String>,
    pub personalities: Vec<String>,
    pub sex: Option<Sex>,
}


impl From<CreateUserInput> for User {
    fn from(input: CreateUserInput) -> Self {
        Self {
            id: mongodb::bson::oid::ObjectId::new().into(),
            username: input.username,
            password: input.password,
            first_name: input.first_name.unwrap_or("None".to_string()),
            last_name: input.last_name.unwrap_or("None".to_string()),
            country: input.country,
            skills: input.skills,
            primary_email: input.primary_email,
            other_emails: input.other_emails,
            about: input.about,
            avatar: input.avatar,
            cover_photo: input.cover_photo,
            educations: input.educations,
            rating: input.rating,
            level: input.level,
            cv: Vec::default(),
            shared_cvs: Vec::default(),
            saved_cvs: Vec::default(),
            liked_cvs: Vec::default(),
            friends_list: Vec::default(),
            city: input.city,
            experiences: input.experiences,
            personalities: input.personalities,
            sex: input.sex,
        }
    }
}
