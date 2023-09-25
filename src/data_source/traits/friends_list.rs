use crate::data_source::DataSourceError;
use async_graphql::futures_util::stream::BoxStream;
use async_trait::async_trait;

use mongodb::bson;

use crate::models::friend_request::FriendRequest;

#[async_trait]
pub trait FriendsListDataSource {
    /// Add new friend request to the database.
    async fn add_friend_request(
        &self,
        _friend_request: FriendRequest,
    ) -> Result<(), DataSourceError>;

    async fn update_friend_request(
        &self,
        _friend_request: FriendRequest,
    ) -> Result<(), DataSourceError>;

    async fn get_friend_request(
        &self,
        from: bson::oid::ObjectId,
        to: bson::oid::ObjectId,
    ) -> Result<FriendRequest, DataSourceError>;

    /// Return the list of friend requests of the user.
    async fn friend_requests(
        &self,
        _user_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, DataSourceError>>;

    /// Return the list of friend requests sent by the user.
    async fn friend_requests_sent(
        &self,
        _user_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, DataSourceError>>;

    async fn accepted_friend_requests(
        &self,
        _friend_request_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, DataSourceError>>;
}
