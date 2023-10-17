use async_graphql::{ComplexObject, SimpleObject};
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, SimpleObject)]
pub struct Key {
    pub user_id: ObjectId,
    pub cv_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex)]
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

#[ComplexObject]
impl Like {
    pub async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex)]
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

#[ComplexObject]
impl Bookmark {
    pub async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex)]
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

#[ComplexObject]
impl Share {
    pub async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}
