#[cfg(test)]
mod tests;
use std::pin::Pin;

use async_graphql::futures_util::{stream::BoxStream, StreamExt};
use mongodb::bson::oid::ObjectId;

use crate::{
    data_source::{
        UserDataSource, UserDataSourceError, {FriendsListDataSource, FriendsListError},
    },
    models::{
        friend_request::FriendRequest,
        users::{CreateUserInput, UpdateUserInput, User},
    },
};

pub struct UserService;

impl UserService {
    pub async fn get_user_by_id(
        database: &(impl UserDataSource + std::marker::Sync),
        user_id: ObjectId,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id).await;
        match user {
            Ok(user) => {
                return Ok(user);
            }
            Err(_) => {
                return Err(UserDataSourceError::IdNotFound(user_id));
            }
        }
    }

    pub async fn update_user(
        database: &(impl UserDataSource + std::marker::Sync),
        update_user_input: UpdateUserInput,
    ) -> Result<User, UserDataSourceError> {
        let user = database.update_user_info(update_user_input).await;
        match user {
            Ok(user) => {
                return Ok(user);
            }
            Err(_) => {
                return Err(UserDataSourceError::UpdateUserFailed);
            }
        }
    }

    pub async fn create_user(
        database: &(impl UserDataSource + std::marker::Sync),
        user: CreateUserInput,
    ) -> Result<User, UserDataSourceError> {
        let user = database.create_user(user).await;
        match user {
            Ok(user) => {
                return Ok(user);
            }
            Err(_) => {
                return Err(UserDataSourceError::CreateUserFailed);
            }
        }
    }

    pub async fn get_user_by_username(
        database: &(impl UserDataSource + std::marker::Sync),
        username: String,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_username(&username).await;
        match user {
            Ok(user) => {
                return Ok(user);
            }
            Err(_) => {
                return Err(UserDataSourceError::UsernameNotFound(username));
            }
        }
    }

    pub async fn send_friend_request(
        database: &(impl UserDataSource + FriendsListDataSource + std::marker::Sync),
        user_id: ObjectId,
        friend_id: ObjectId,
        message: Option<impl Into<String>>,
    ) -> Result<(), FriendsListError> {
        let user = database.get_user_by_id(user_id).await;
        if user.is_err() {
            return Err(FriendsListError::UserNotFound);
        }
        let friend = database.get_user_by_id(friend_id).await;
        if friend.is_err() {
            return Err(FriendsListError::UserNotFound);
        }
        let friend_request = FriendRequest::new(user_id, friend_id, message);
        let friend_request = database.add_friend_request(friend_request).await;
        match friend_request {
            Ok(_) => {
                return Ok(());
            }
            Err(_) => {
                return Err(FriendsListError::AddFriendFailed);
            }
        }
    }

    pub async fn accept_friend_request(
        database: &(impl UserDataSource + FriendsListDataSource + std::marker::Sync),
        user_id: ObjectId,
        friend_id: ObjectId,
    ) -> Result<(), FriendsListError> {
        let friend_request = database.get_friend_request(friend_id, user_id).await;
        let friend_request = friend_request?.accept();
        database.update_friend_request(friend_request).await?;
        Ok(())
    }

    pub async fn reject_friend_request(
        database: &(impl UserDataSource + FriendsListDataSource + std::marker::Sync),
        user_id: ObjectId,
        friend_id: ObjectId,
    ) -> Result<(), FriendsListError> {
        let friend_request = database.get_friend_request(friend_id, user_id).await;
        let friend_request = friend_request?.reject();
        database.update_friend_request(friend_request).await?;
        Ok(())
    }

    pub async fn friend_lists(
        database: &(impl UserDataSource + FriendsListDataSource + std::marker::Sync),
        user_id: ObjectId,
    ) -> BoxStream<Result<User, UserDataSourceError>> {
        let users = database
            .accepted_friend_requests(user_id)
            .await
            .map(|f| {
                let f = f.unwrap();
                if f._id.from == user_id {
                    f._id.to
                } else {
                    f._id.from
                }
            })
            .boxed();
        database.get_users_by_ids(users).await
    }
}
