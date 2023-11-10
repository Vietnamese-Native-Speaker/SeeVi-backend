use async_graphql as gql;
use async_graphql::{ComplexObject, SimpleObject};
use gql::futures_util::StreamExt;
use gql::{connection, Context};
use mongodb::bson::{self, DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::{
    data_source::mongo::{MongoDB, MongoForTesting},
    models::{comment::Comment, ResourceIdentifier},
    object_id::ScalarObjectId,
    services::cv_service::comment_service::CommentService,
};

use super::CreateCVInput;

/// Struct represents CV defined in the Diagram. Note that this struct only
/// represents the metadata of a CV.
#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, PartialEq)]
#[graphql(complex)]
pub struct CV {
    #[serde(rename = "_id")]
    pub id: ScalarObjectId,
    pub author_id: ScalarObjectId,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    #[graphql(skip)]
    pub comments: Vec<bson::oid::ObjectId>,
    #[graphql(skip)]
    pub created: DateTime,
    /// The resource identifier of the CV, can be used to query the actual CV data on the storage.
    pub cv: Option<ResourceIdentifier>,
}

impl From<CreateCVInput> for CV {
    fn from(input: CreateCVInput) -> Self {
        Self {
            id: bson::oid::ObjectId::new().into(),
            author_id: input.author_id.into(),
            title: input.title,
            description: input.description,
            tags: input.tags,
            comments: Vec::default(),
            created: DateTime::now(),
            cv: Uuid::new().into(),
        }
    }
}
