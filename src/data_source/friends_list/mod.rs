use async_graphql::futures_util::stream::BoxStream;
use async_trait::async_trait;

mod error;

pub use error::FriendsListError;
use mongodb::bson;

use crate::models::friend_request::FriendRequest;

#[async_trait]
pub trait FriendsListDataSource {
    /// Add new friend request to the database.
    async fn add_friend_request(
        &self,
        _friend_request: FriendRequest,
    ) -> Result<(), FriendsListError> {
        unimplemented!()
    }

    async fn update_friend_request(
        &self,
        _friend_request: FriendRequest,
    ) -> Result<(), FriendsListError> {
        unimplemented!()
    }

    async fn get_friend_request(
        &self,
        _from: bson::oid::ObjectId,
        _to: bson::oid::ObjectId,
    ) -> Result<FriendRequest, FriendsListError> {
        unimplemented!()
    }

    /// Return the list of friend requests of the user.
    async fn friend_requests(
        &self,
        _user_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, FriendsListError>> {
        unimplemented!()
    }

    /// Return the list of friend requests sent by the user.
    async fn friend_requests_sent(
        &self,
        _user_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, FriendsListError>> {
        unimplemented!()
    }

    async fn accepted_friend_requests(
        &self,
        _friend_request_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, FriendsListError>> {
        unimplemented!()
    }
}
