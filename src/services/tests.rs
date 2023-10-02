use crate::data_source::CVDataSource;
use crate::data_source::CVDataSourceError;
use crate::data_source::CommentDataSource;
use crate::data_source::UserDataSource;
use crate::data_source::UserDataSourceError;
use crate::data_source::{FriendsListDataSource, FriendsListError};
use crate::models::comment::Comment;
use crate::models::comment::CreateCommentInput;
use crate::models::comment::UpdateCommentInput;
use crate::models::cv::CV;
use crate::models::friend_request::{FriendRequest, FriendRequestStatus};
use crate::models::users::{CreateUserInput, UpdateUserInput, User};
use async_graphql::futures_util::stream::BoxStream;
use async_graphql::futures_util::{self, StreamExt};
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use mongodb::bson;
use std::sync::Mutex;

use super::cv_service::comment_service::CommentServiceError;

pub struct MockDatabase {
    users: Mutex<Vec<User>>,
    friend_requests: Mutex<Vec<FriendRequest>>,
    cvs: Mutex<Vec<CV>>,
    comments: Mutex<Vec<Comment>>,
}

impl MockDatabase {
    pub fn new() -> Self {
        MockDatabase {
            users: Mutex::new(Vec::new()),
            friend_requests: Mutex::new(Vec::new()),
            cvs: Mutex::new(Vec::new()),
            comments: Mutex::new(Vec::new()),
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
            if request.id.from == friend_request.id.from && request.id.to == friend_request.id.to {
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
            async move { friend_request.id.to == user_id }
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
            async move { friend_request.id.from == user_id }
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
                    && (friend_request.id.from == user_id || friend_request.id.to == user_id)
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
            if request.id.from == from && request.id.to == to {
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
        ids: Vec<bson::oid::ObjectId>,
    ) -> BoxStream<Result<User, UserDataSourceError>> {
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

#[async_trait]
impl CVDataSource for MockDatabase {
    /// Add comment to the CV with the provided id.
    async fn add_comment_to_cv(
        &self,
        _cv_id: ObjectId,
        _comment_id: ObjectId,
    ) -> Result<CV, CVDataSourceError> {
        let cvs = self.cvs.lock().unwrap();
        cvs.iter_mut()
            .find(|cv| cv.id == _cv_id.into())
            .map_or_else(
                || Err(CVDataSourceError::IdNotFound(_cv_id.clone())),
                |cv| {
                    cv.comments.push(_comment_id.clone());
                    Ok(cv.clone())
                },
            )
    }

    /// Remove comment from the CV with the provided id.
    async fn remove_comment_from_cv(
        &self,
        _cv_id: ObjectId,
        _comment_id: ObjectId,
    ) -> Result<CV, CVDataSourceError> {
        let cvs = self.cvs.lock().unwrap();
        cvs.iter_mut()
            .find(|cv| cv.id == _cv_id.into())
            .map_or_else(
                || Err(CVDataSourceError::IdNotFound(_cv_id.clone())),
                |cv| {
                    cv.comments.retain(|comment| comment != &_comment_id);
                    Ok(cv.clone())
                },
            )
    }
}

#[derive(Debug)]
struct DummyCommentDataSourceError;

impl std::fmt::Display for DummyCommentDataSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dummy error")
    }
}

impl From<DummyCommentDataSourceError> for CommentServiceError {
    fn from(_: DummyCommentDataSourceError) -> Self {
        CommentServiceError::EmptyContent
    }
}

impl std::error::Error for DummyCommentDataSourceError {}

#[async_trait]
impl CommentDataSource for MockDatabase {
    type Error = DummyCommentDataSourceError;
    async fn get_comment_by_id(&self, _id: bson::oid::ObjectId) -> Result<Comment, Self::Error> {
        let comments = self.comments.lock().unwrap();
        comments.iter().find(|cmt| cmt.id == _id.into()).map_or_else(
            || Err(DummyCommentDataSourceError), 
            |comment| Ok(comment.clone())
        )
    }

    async fn get_comments_by_cv_id(
        &self,
        _cv_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<Comment, Self::Error>> {
        let comments = self.comments.lock().unwrap().clone();
        let stream = futures_util::stream::iter(comments.into_iter());
        let stream = stream.filter(move |comment| {
            let comment_id = comment.id.clone();
            async move { comment_id == _cv_id.into() }
        });
        stream.map(|comment| Ok(comment)).boxed()
    }

    async fn create_comment(&self, _input: CreateCommentInput) -> Result<Comment, Self::Error> {
        let mut comments = self.comments.lock().unwrap();
        let comment = _input.into();
        comments.push(comment);
        Ok(comment)
    }

    async fn add_comment(&self, _comment: Comment) -> Result<(), Self::Error> {
        let mut comments = self.comments.lock().unwrap();
        comments.push(_comment);
        Ok(())
    }

    async fn update_comment(&self, _input: UpdateCommentInput) -> Result<Comment, Self::Error> {
        let mut comments = self.comments.lock().unwrap();
        for comment in comments.iter_mut() {
            if comment.id == _input.id.into() {
                comment.content = _input.content.clone().unwrap_or(comment.content.clone());
                comment.likes = _input.likes.clone().unwrap_or(comment.likes);
                comment.bookmarks = _input.bookmarks.clone().unwrap_or(comment.bookmarks);
                comment.shares = _input.shares.clone().unwrap_or(comment.shares);
                comment.replies = _input.replies.clone().unwrap_or(comment.replies);
                return Ok(comment.clone());
            }
        }
        Err(DummyCommentDataSourceError)
    }

    async fn delete_comment(&self, _id: bson::oid::ObjectId) -> Result<Comment, Self::Error> {
        let mut comments = self.comments.lock().unwrap();
        for (i, comment) in comments.iter().enumerate() {
            if comment.id == _id.into() {
                return Ok(comments.remove(i));
            }
        }
        Err(DummyCommentDataSourceError)
    }

    async fn add_reply_to_comment(
        &self,
        _comment_id: bson::oid::ObjectId,
        _reply_id: bson::oid::ObjectId,
    ) -> Result<Comment, Self::Error> {
        let mut comments = self.comments.lock().unwrap();
        for comment in comments.iter_mut() {
            if comment.id == _comment_id.into() {
                comment.replies.push(_reply_id.clone().into());
                return Ok(comment.clone());
            }
        }
        Err(DummyCommentDataSourceError)
    }

    async fn remove_reply_from_comment(
        &self,
        _comment_id: bson::oid::ObjectId,
        _reply_id: bson::oid::ObjectId,
    ) -> Result<Comment, Self::Error> {
        let mut comments = self.comments.lock().unwrap();
        for comment in comments.iter_mut() {
            if comment.id == _comment_id.into() {
                comment.replies.retain(|reply| reply != &_reply_id.into());
                return Ok(comment.clone());
            }
        }
        Err(DummyCommentDataSourceError)
    }
}
