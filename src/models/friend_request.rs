use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FriendRequestStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FriendRequest {
    pub from: ObjectId,
    pub to: ObjectId,
    pub message: String,
    pub status: FriendRequestStatus,
    pub created_at: bson::DateTime,
    pub updated_at: bson::DateTime,
}

impl FriendRequest {
    pub fn new(from: ObjectId, to: ObjectId, message: impl Into<String>) -> Self {
        Self {
            from,
            to,
            message: message.into(),
            status: FriendRequestStatus::Pending,
            created_at: bson::DateTime::now(),
            updated_at: bson::DateTime::now(),
        }
    }

    pub fn accept(mut self) -> Self {
        self.status = FriendRequestStatus::Accepted;
        self.updated_at = bson::DateTime::now();
        self
    }

    pub fn reject(mut self) -> Self {
        self.status = FriendRequestStatus::Rejected;
        self.updated_at = bson::DateTime::now();
        self
    }
}

