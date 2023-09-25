use crate::data_source::{FriendsListDataSource, FriendsListError};
use crate::data_source::UserDataSource;
use crate::data_source::UserDataSourceError;
use crate::models::friend_request::{FriendRequest, FriendRequestStatus};
use crate::models::users::{CreateUserInput, UpdateUserInput, User};
use async_graphql::futures_util::stream::BoxStream;
use async_graphql::futures_util::{self, StreamExt};
use async_trait::async_trait;
use mongodb::bson::{self, Uuid};
use std::sync::Mutex;

pub struct MockDatabase {
    users: Mutex<Vec<User>>,
    friend_requests: Mutex<Vec<FriendRequest>>,
}

impl MockDatabase {
    pub fn new() -> Self {
        MockDatabase {
            users: Mutex::new(Vec::new()),
            friend_requests: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl FriendsListDataSource for MockDatabase {
    /// Add new friend request to the database.
    async fn add_friend_request(
        &self,
        friend_request: FriendRequest,
    ) -> Result<(), FriendsListError> {
        self.friend_requests
            .lock()
            .unwrap()
            .push(friend_request.clone());
        Ok(())
    }

    async fn update_friend_request(
        &self,
        friend_request: FriendRequest,
    ) -> Result<(), FriendsListError> {
        let mut friend_requests = self.friend_requests.lock().unwrap();
        for request in friend_requests.iter_mut() {
            if request._id.from == friend_request._id.from
                && request._id.to == friend_request._id.to
            {
                *request = friend_request.clone();
                return Ok(());
            }
        }
        Err(FriendsListError::UserNotFound)
    }

    /// Return the list of friend requests of the user.
    async fn friend_requests(
        &self,
        user_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, FriendsListError>> {
        let friend_requests = self.friend_requests.lock().unwrap().clone();
        let stream = futures_util::stream::iter(friend_requests.into_iter());
        let stream = stream.filter(move |friend_request| {
            let friend_request = friend_request.clone();
            async move { friend_request._id.to == user_id }
        });
        stream
            .map(|friend_request| Ok(friend_request.clone()))
            .boxed()
    }

    /// Return the list of friend requests sent by the user.
    async fn friend_requests_sent(
        &self,
        user_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, FriendsListError>> {
        let friend_requests = self.friend_requests.lock().unwrap().clone();
        let mut stream = futures_util::stream::iter(friend_requests.into_iter());
        let stream = stream.filter(move |friend_request| {
            let friend_request = friend_request.clone();
            async move { friend_request._id.from == user_id }
        });
        stream.map(|friend_request| Ok(friend_request)).boxed()
    }

    async fn accepted_friend_requests(
        &self,
        user_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, FriendsListError>> {
        let friend_requests = self.friend_requests.lock().unwrap().clone();
        let mut stream = futures_util::stream::iter(friend_requests.into_iter());
        let stream = stream.filter(move |friend_request| {
            let friend_request = friend_request.clone();
            async move {
                friend_request.status == FriendRequestStatus::Accepted
                    && (friend_request._id.from == user_id || friend_request._id.to == user_id)
            }
        });
        stream.map(|friend_request| Ok(friend_request)).boxed()
    }

    async fn get_friend_request(
        &self,
        from: bson::oid::ObjectId,
        to: bson::oid::ObjectId,
    ) -> Result<FriendRequest, FriendsListError> {
        let friend_requests = self.friend_requests.lock().unwrap();
        for request in friend_requests.iter() {
            if request._id.from == from && request._id.to == to {
                return Ok(request.clone());
            }
        }
        Err(FriendsListError::UserNotFound)
    }
}

#[async_trait]
impl UserDataSource for MockDatabase {
    async fn get_user_by_username(&self, username: &str) -> Result<User, UserDataSourceError> {
        let users = self.users.lock().unwrap();
        for user in users.iter() {
            if user.username == username {
                return Ok(user.clone());
            }
        }
        Err(UserDataSourceError::UsernameNotFound(username.to_string()))
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User, UserDataSourceError> {
        let users = self.users.lock().unwrap();
        for user in users.iter() {
            if user.primary_email == email {
                return Ok(user.clone());
            }
        }
        Err(UserDataSourceError::EmailNotFound(email.to_string()))
    }
    async fn get_user_by_id(&self, id: bson::oid::ObjectId) -> Result<User, UserDataSourceError> {
        let users = self.users.lock().unwrap();
        for user in users.iter() {
            if user.id == id.into() {
                return Ok(user.clone());
            }
        }
        Err(UserDataSourceError::IdNotFound(id.clone()))
    }
    async fn update_user_info(
        &self,
        updated_user: UpdateUserInput,
    ) -> Result<User, UserDataSourceError> {
        let mut users = self.users.lock().unwrap();
        for user in users.iter_mut() {
            if user.id == updated_user.user_id.into() {
                user.username = updated_user
                    .username
                    .clone()
                    .unwrap_or(user.username.clone());
                user.password = updated_user
                    .password
                    .clone()
                    .unwrap_or(user.password.clone());
                user.first_name = updated_user
                    .first_name
                    .clone()
                    .unwrap_or(user.first_name.clone());
                user.last_name = updated_user
                    .last_name
                    .clone()
                    .unwrap_or(user.last_name.clone());
                user.country = updated_user.country.clone().or(user.country.clone());
                user.skills = updated_user.skills.clone().unwrap_or(user.skills.clone());
                user.primary_email = updated_user
                    .primary_email
                    .clone()
                    .unwrap_or(user.primary_email.clone());
                user.other_mails = updated_user
                    .other_mails
                    .clone()
                    .unwrap_or(user.other_mails.clone());
                user.about = updated_user.about.clone().or(user.about.clone());
                user.avatar = updated_user.avatar.clone().or(user.avatar.clone());
                user.cover_photo = updated_user
                    .cover_photo
                    .clone()
                    .or(user.cover_photo.clone());
                user.friends_list = updated_user
                    .friends_list
                    .clone()
                    .unwrap_or(user.friends_list.clone());
                user.education = updated_user
                    .education
                    .clone()
                    .unwrap_or(user.education.clone());
                return Ok(user.clone());
            }
        }
        return Err(UserDataSourceError::IdNotFound(
            updated_user.user_id.clone(),
        ));
    }

    async fn create_user(&self, _input: CreateUserInput) -> Result<User, UserDataSourceError> {
        let mut users = self.users.lock().unwrap();
        let user = User::from(_input);
        users.push(user.clone());
        Ok(user)
    }

    async fn get_users_by_ids(
        &self,
        ids: BoxStream<'async_trait, bson::oid::ObjectId>,
    ) -> BoxStream<Result<User, UserDataSourceError>> {
        let ids = ids.collect::<Vec<_>>().await;
        let users = self.users.lock().unwrap().clone();
        let stream = futures_util::stream::iter(users.into_iter());
        let stream = stream.filter(move |user| {
            let user = user.clone();
            let ids = ids.clone();
            async move { ids.contains(&user.id) }
        });
        stream.map(|user| Ok(user)).boxed()
    }
}
