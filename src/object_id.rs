use std::str::FromStr;

use async_graphql::{connection::CursorType, Scalar, ScalarType};
use mongodb::bson;
use serde::{Deserialize, Serialize};

/// A wrapper around `bson::oid::ObjectId` to make it work with async-graphql.
#[derive(
    Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default,
)]
pub struct ScalarObjectId(bson::oid::ObjectId);

impl ScalarObjectId {
    pub fn new() -> Self {
        ScalarObjectId(bson::oid::ObjectId::new())
    }

    pub fn to_object_id(self) -> bson::oid::ObjectId {
        self.0
    }

    pub fn from_object_id(id: bson::oid::ObjectId) -> Self {
        ScalarObjectId(id)
    }
}

#[Scalar]
impl ScalarType for ScalarObjectId {
    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.to_string())
    }

    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        match value {
            async_graphql::Value::String(s) => Ok(bson::oid::ObjectId::from_str(&s)?.into()),
            _ => Err(async_graphql::InputValueError::expected_type(value)),
        }
    }
}

impl From<bson::oid::ObjectId> for ScalarObjectId {
    fn from(id: bson::oid::ObjectId) -> Self {
        ScalarObjectId(id)
    }
}

impl From<ScalarObjectId> for bson::oid::ObjectId {
    fn from(id: ScalarObjectId) -> Self {
        id.0
    }
}

impl CursorType for ScalarObjectId {
    type Error = bson::oid::Error;

    fn decode_cursor(cursor: &str) -> Result<Self, Self::Error> {
        cursor.parse().map(ScalarObjectId)
    }

    fn encode_cursor(&self) -> String {
        self.0.to_string()
    }
}

impl std::ops::Deref for ScalarObjectId {
    type Target = bson::oid::ObjectId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ScalarObjectId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
