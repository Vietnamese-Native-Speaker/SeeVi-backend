use async_graphql::{ComplexObject, SimpleObject};
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, SimpleObject)]
pub struct CvInteractionsKey {
    pub user_id: ObjectId,
    pub cv_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex)]
pub struct CvLike {
    #[serde(rename = "_id")]
    key: CvInteractionsKey,
    #[graphql(skip)]
    pub created: DateTime,
}

impl CvLike {
    pub fn new(user_id: ObjectId, cv_id: ObjectId) -> Self {
        Self {
            key: CvInteractionsKey { user_id, cv_id },
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
impl CvLike {
    pub async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, PartialEq)]
#[graphql(complex)]
pub struct CvBookmark {
    #[serde(rename = "_id")]
    key: CvInteractionsKey,
    #[graphql(skip)]
    pub created: DateTime,
}

impl CvBookmark {
    pub fn new(user_id: ObjectId, cv_id: ObjectId) -> Self {
        Self {
            key: CvInteractionsKey { user_id, cv_id },
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
impl CvBookmark {
    pub async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, PartialEq)]
#[graphql(complex)]
pub struct CvShare {
    #[serde(rename = "_id")]
    key: CvInteractionsKey,
    #[graphql(skip)]
    pub created: DateTime,
}

impl CvShare {
    pub fn new(user_id: ObjectId, cv_id: ObjectId) -> Self {
        Self {
            key: CvInteractionsKey { user_id, cv_id },
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
impl CvShare {
    pub async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}
