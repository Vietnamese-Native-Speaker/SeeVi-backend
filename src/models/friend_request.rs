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
