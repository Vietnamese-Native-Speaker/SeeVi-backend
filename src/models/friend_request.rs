use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FriendRequestStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FriendRequest {
    pub from: bson::Uuid,
    pub to: bson::Uuid,
    pub status: FriendRequestStatus,
    pub created_at: bson::DateTime,
    pub updated_at: bson::DateTime,
}

impl FriendRequest {
    pub fn new(from: bson::Uuid, to: bson::Uuid) -> Self {
        Self {
            from,
            to,
            status: FriendRequestStatus::Pending,
            created_at: bson::DateTime::now(),
            updated_at: bson::DateTime::now(),
        }
    }

    pub fn accept(&mut self) {
        self.status = FriendRequestStatus::Accepted;
        self.updated_at = bson::DateTime::now();
    }

    pub fn reject(&mut self) {
        self.status = FriendRequestStatus::Rejected;
        self.updated_at = bson::DateTime::now();
    }
}