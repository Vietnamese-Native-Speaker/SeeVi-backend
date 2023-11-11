use async_graphql::SimpleObject;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::common::DateTime;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, SimpleObject)]
#[graphql(name = "CvInteractionKey")]
pub struct Key {
    pub user_id: ObjectId,
    pub cv_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(name = "CvLike")]
pub struct Like {
    #[serde(rename = "_id")]
    key: Key,
    #[graphql(skip)]
    pub created: DateTime,
}

impl Like {
    pub fn new(user_id: ObjectId, cv_id: ObjectId) -> Self {
        Self {
            key: Key { user_id, cv_id },
            created: DateTime::now(),
        }
    }

    pub fn user_id(&self) -> &ObjectId {
        &self.key.user_id
    }

    pub fn cv_id(&self) -> &ObjectId {
        &self.key.cv_id
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, PartialEq)]
#[graphql(name = "CvBookmark")]
pub struct Bookmark {
    #[serde(rename = "_id")]
    key: Key,
    #[graphql(skip)]
    pub created: DateTime,
}

impl Bookmark {
    pub fn new(user_id: ObjectId, cv_id: ObjectId) -> Self {
        Self {
            key: Key { user_id, cv_id },
            created: DateTime::now(),
        }
    }

    pub fn user_id(&self) -> &ObjectId {
        &self.key.user_id
    }

    pub fn cv_id(&self) -> &ObjectId {
        &self.key.cv_id
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, PartialEq)]
#[graphql(name = "CvShare")]
pub struct Share {
    #[serde(rename = "_id")]
    key: Key,
    #[graphql(skip)]
    pub created: DateTime,
}

impl Share {
    pub fn new(user_id: ObjectId, cv_id: ObjectId) -> Self {
        Self {
            key: Key { user_id, cv_id },
            created: DateTime::now(),
        }
    }

    pub fn user_id(&self) -> &ObjectId {
        &self.key.user_id
    }

    pub fn cv_id(&self) -> &ObjectId {
        &self.key.cv_id
    }
}
