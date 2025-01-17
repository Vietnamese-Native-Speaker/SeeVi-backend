use crate::data_source::BookmarkDataSource;
use crate::data_source::CVDataSource;
use crate::data_source::CVDataSourceError;
use crate::data_source::CommentDataSource;
use crate::data_source::LikeDataSource;
use crate::data_source::UserDataSource;
use crate::data_source::{FriendsListDataSource, FriendsListError};
use crate::models::comment::Bookmark as CommentBookmark;
use crate::models::comment::Comment;
use crate::models::comment::CreateCommentInput;
use crate::models::comment::Like;
use crate::models::comment::UpdateCommentInput;
use crate::models::cv::interactions::Like as CVLike;
use crate::models::cv::interactions::Share;
use crate::models::cv::Bookmark as CVBookmark;
use crate::models::cv::CreateCVInput;
use crate::models::cv::UpdateCVInput;
use crate::models::cv::CV;
use crate::models::friend_request::{FriendRequest, FriendRequestStatus};
use crate::models::users::{CreateUserInput, UpdateUserInput, User};
use async_graphql::futures_util::stream::BoxStream;
use async_graphql::futures_util::{self, StreamExt};
use async_trait::async_trait;
use mongodb::bson;
use mongodb::bson::oid::ObjectId;
use std::fmt::Display;
use std::fmt::Formatter;
use std::sync::Mutex;

use super::cv_service::comment_service::CommentServiceError;
use super::cv_service::error::CVServiceError;
use super::user_service::error::UserServiceError;

pub struct MockDatabase {
    pub(crate) users: Mutex<Vec<User>>,
    pub(crate) friend_requests: Mutex<Vec<FriendRequest>>,
    pub(crate) cvs: Mutex<Vec<CV>>,
    pub(crate) comments: Mutex<Vec<Comment>>,
    pub(crate) likes: Mutex<Vec<Like>>,
    pub(crate) cv_shares: Mutex<Vec<Share>>,
    pub(crate) cv_bookmarks: Mutex<Vec<CVBookmark>>,
    pub(crate) cv_likes: Mutex<Vec<CVLike>>,
    pub(crate) bookmarks: Mutex<Vec<CommentBookmark>>,
}

impl MockDatabase {
    pub fn new() -> Self {
        MockDatabase {
            users: Mutex::new(Vec::new()),
            friend_requests: Mutex::new(Vec::new()),
            cvs: Mutex::new(Vec::new()),
            comments: Mutex::new(Vec::new()),
            likes: Mutex::new(Vec::new()),
            cv_shares: Mutex::new(Vec::new()),
            cv_bookmarks: Mutex::new(Vec::new()),
            cv_likes: Mutex::new(Vec::new()),
            bookmarks: Mutex::new(Vec::new()),
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
        let stream = futures_util::stream::iter(friend_requests.into_iter());
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
        let stream = futures_util::stream::iter(friend_requests.into_iter());
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

#[derive(Debug, Clone)]
pub struct MockUserDataSourceError;

impl std::fmt::Display for MockUserDataSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dummy Error")
    }
}

impl From<MockUserDataSourceError> for UserServiceError {
    fn from(_: MockUserDataSourceError) -> Self {
        UserServiceError::CreateUserFailed
    }
}

impl std::error::Error for MockUserDataSourceError {}

#[async_trait]
impl UserDataSource for MockDatabase {
    type Error = MockUserDataSourceError;
    async fn get_user_by_username(&self, username: &str) -> Result<User, Self::Error> {
        let users = self.users.lock().unwrap();
        for user in users.iter() {
            if user.username == username {
                return Ok(user.clone());
            }
        }
        Err(MockUserDataSourceError)
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User, Self::Error> {
        let users = self.users.lock().unwrap();
        for user in users.iter() {
            if user.primary_email == email {
                return Ok(user.clone());
            }
        }
        Err(MockUserDataSourceError)
    }
    async fn get_user_by_id(&self, id: bson::oid::ObjectId) -> Result<User, Self::Error> {
        let users = self.users.lock().unwrap();
        for user in users.iter() {
            if user.id == id.into() {
                return Ok(user.clone());
            }
        }
        Err(MockUserDataSourceError)
    }
    async fn update_user_info(&self, updated_user: UpdateUserInput) -> Result<User, Self::Error> {
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
                user.other_emails = updated_user
                    .other_mails
                    .clone()
                    .unwrap_or(user.other_emails.clone());
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
                user.educations = updated_user
                    .educations
                    .clone()
                    .unwrap_or(user.educations.clone());
                return Ok(user.clone());
            }
        }
        return Err(MockUserDataSourceError);
    }

    async fn create_user(&self, _input: CreateUserInput) -> Result<User, Self::Error> {
        let mut users = self.users.lock().unwrap();
        let user = User::from(_input);
        users.push(user.clone());
        Ok(user)
    }

    async fn get_users_by_ids(
        &self,
        ids: Vec<bson::oid::ObjectId>,
    ) -> BoxStream<Result<User, Self::Error>> {
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
    async fn get_cvs_by_user_id(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Result<CV, CVDataSourceError>>, CVDataSourceError> {
        let cvs = self.cvs.lock().unwrap();
        let stream = futures_util::stream::iter(cvs.clone().into_iter());
        let stream = stream.filter(move |cv| {
            let cv = cv.clone();
            let user_id = user_id.clone();
            async move { user_id == cv.author_id.into() }
        });
        Ok(stream.map(|user| Ok(user)).boxed())
    }

    async fn get_comments_by_cv_id(
        &self,
        _cv_id: ObjectId,
    ) -> Result<Vec<ObjectId>, CVDataSourceError> {
        let cvs = self.cvs.lock().unwrap();
        for cv in cvs.iter() {
            if cv.id == _cv_id.into() {
                return Ok(cv.comments.clone());
            }
        }
        Err(CVDataSourceError::IdNotFound(_cv_id.clone()))
    }

    async fn create_cv(&self, _input: CreateCVInput) -> Result<CV, CVDataSourceError> {
        let mut cvs = self.cvs.lock().unwrap();
        let cv = CV::from(_input);
        cvs.push(cv.clone());
        Ok(cv)
    }

    async fn get_cv_by_id(&self, _id: ObjectId) -> Result<CV, CVDataSourceError> {
        let cvs = self.cvs.lock().unwrap();
        for cv in cvs.iter() {
            if cv.id == _id.into() {
                return Ok(cv.clone());
            }
        }
        Err(CVDataSourceError::IdNotFound(_id.clone()))
    }

    async fn find_and_update_cv(
        &self,
        _cv_id: ObjectId,
        _input: UpdateCVInput,
    ) -> Result<CV, CVDataSourceError> {
        let mut cvs = self.cvs.lock().unwrap();
        for cv in cvs.iter_mut() {
            if cv.id == _cv_id.into() {
                cv.title = _input.title.clone().unwrap_or(cv.title.clone());
                cv.description = _input.description.clone();
                cv.tags = _input.tags.clone().unwrap_or(cv.tags.clone());
                return Ok(cv.clone());
            }
        }
        Err(CVDataSourceError::IdNotFound(_cv_id.clone()))
    }

    /// Add comment to the CV with the provided id.
    async fn add_comment_to_cv(
        &self,
        _cv_id: ObjectId,
        _comment_id: Comment,
    ) -> Result<CV, CVDataSourceError> {
        let mut cvs = self.cvs.lock().unwrap();
        cvs.iter_mut()
            .find(|cv| cv.id == _cv_id.into())
            .map_or_else(
                || Err(CVDataSourceError::IdNotFound(_cv_id.clone())),
                |cv| {
                    cv.comments.push(_comment_id.clone().id.into());
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
        let mut cvs = self.cvs.lock().unwrap();
        // remove comment from the db
        let mut comments = self.comments.lock().unwrap();
        for (i, comment) in comments.iter().enumerate() {
            if comment.id == _comment_id.into() {
                comments.remove(i);
                break;
            }
        }
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
pub struct DummyCommentDataSourceError;

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
        comments
            .iter()
            .find(|cmt| cmt.id == _id.into())
            .map_or_else(
                || Err(DummyCommentDataSourceError),
                |comment| Ok(comment.clone()),
            )
    }

    async fn get_comments_list(
        &self,
        _ids: Vec<ObjectId>,
    ) -> BoxStream<Result<Comment, Self::Error>> {
        let comments = self.comments.lock().unwrap().clone();
        let stream = futures_util::stream::iter(comments.into_iter());
        let stream = stream.filter(move |comment| {
            let comment = comment.clone();
            let ids = _ids.clone();
            async move { ids.contains(&comment.id) }
        });
        stream.map(|comment| Ok(comment)).boxed()
    }

    async fn create_comment(&self, _input: CreateCommentInput) -> Result<Comment, Self::Error> {
        let mut comments = self.comments.lock().unwrap();
        let comment: Comment = _input.into();
        comments.push(comment.clone());
        Ok(comment)
    }

    async fn add_comment(&self, _comment: Comment) -> Result<(), Self::Error> {
        let mut comments = self.comments.lock().unwrap();
        comments.push(_comment);
        Ok(())
    }

    async fn remove_comment(&self, _id: bson::oid::ObjectId) -> Result<Comment, Self::Error> {
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

    async fn find_and_remove_reply(
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

    async fn find_and_update_comment(
        &self,
        _id: bson::oid::ObjectId,
        _input: UpdateCommentInput,
    ) -> Result<Comment, Self::Error> {
        let mut comments = self.comments.lock().unwrap();
        for comment in comments.iter_mut() {
            if comment.id == _id.into() {
                comment.content = _input.content.clone().unwrap_or(comment.content.clone());
                comment.replies = _input.replies.clone().unwrap_or(comment.replies.clone());
                return Ok(comment.clone());
            }
        }
        Err(DummyCommentDataSourceError)
    }
}

#[derive(Debug)]
pub struct DummyLikeDataSource(String);

impl std::fmt::Display for DummyLikeDataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<DummyLikeDataSource> for CommentServiceError {
    fn from(_: DummyLikeDataSource) -> Self {
        CommentServiceError::EmptyContent
    }
}

impl std::error::Error for DummyLikeDataSource {}

#[async_trait]
impl LikeDataSource for MockDatabase {
    type Error = DummyLikeDataSource;
    async fn add_like(
        &self,
        user_id: bson::oid::ObjectId,
        comment_id: bson::oid::ObjectId,
    ) -> Result<(), Self::Error> {
        let mut likes = self.likes.lock().unwrap();
        for like in likes.iter_mut() {
            if like.key.user_id == user_id.clone().into()
                && like.key.comment_id == comment_id.clone().into()
            {
                return Err(DummyLikeDataSource("like already exists".to_string()));
            }
        }
        likes.push(Like::new(user_id.into(), comment_id.into()));
        Ok(())
    }

    async fn delete_like(
        &self,
        user_id: bson::oid::ObjectId,
        comment_id: bson::oid::ObjectId,
    ) -> Result<(), Self::Error> {
        let mut likes = self.likes.lock().unwrap();
        for like in likes.iter_mut() {
            if like.key.user_id == user_id.clone().into()
                && like.key.comment_id == comment_id.clone().into()
            {
                likes.retain(|like| {
                    like.key.user_id != user_id.clone().into()
                        && like.key.comment_id != comment_id.into()
                });
                return Ok(());
            }
        }
        Err(DummyLikeDataSource("like not found".to_string()))
    }

    async fn get_likes_count_of_comment(&self, comment_id: bson::oid::ObjectId) -> Result<i32, Self::Error> {
        let likes = self.likes.lock().unwrap();
        let mut count = 0;
        for like in likes.iter() {
            if like.key.comment_id == comment_id.clone().into() {
                count += 1;
            }
        }
        Ok(count)
    }

    async fn get_likes(
        &self,
        comment_id: bson::oid::ObjectId,
    ) -> Result<BoxStream<Like>, Self::Error> {
        let likes = self.likes.lock().unwrap().clone();
        let stream = futures_util::stream::iter(likes.into_iter());
        let stream = stream.filter(move |like| {
            let like = like.clone();
            async move { like.key.comment_id == comment_id.clone().into() }
        });
        Ok(stream.map(|like| like).boxed())
    }
}

#[derive(Debug)]
pub enum CVInteractionError {
    NotFound,
    AlreadyExists,
}

impl Display for CVInteractionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CVInteractionError::NotFound => write!(f, "share not found"),
            CVInteractionError::AlreadyExists => write!(f, "share already exists"),
        }
    }
}

impl std::error::Error for CVInteractionError {}

impl From<CVInteractionError> for CVServiceError {
    fn from(_: CVInteractionError) -> Self {
        CVServiceError::DatabaseError
    }
}

#[derive(Debug)]
pub struct DummyBookmarkDataSource(String);

impl std::fmt::Display for DummyBookmarkDataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<DummyBookmarkDataSource> for CommentServiceError {
    fn from(_: DummyBookmarkDataSource) -> Self {
        CommentServiceError::EmptyContent
    }
}

impl std::error::Error for DummyBookmarkDataSource {}

#[async_trait]
impl BookmarkDataSource for MockDatabase {
    type Error = DummyBookmarkDataSource;
    async fn add_bookmark(
        &self,
        user_id: bson::oid::ObjectId,
        comment_id: bson::oid::ObjectId,
    ) -> Result<(), Self::Error> {
        let mut bookmakrs = self.bookmarks.lock().unwrap();
        for bookmark in bookmakrs.iter_mut() {
            if bookmark.key.user_id == user_id.clone().into()
                && bookmark.key.comment_id == comment_id.clone().into()
            {
                return Err(DummyBookmarkDataSource(
                    "bookmark already exists".to_string(),
                ));
            }
        }
        bookmakrs.push(CommentBookmark::new(user_id.into(), comment_id.into()));
        Ok(())
    }

    async fn delete_bookmark(
        &self,
        user_id: bson::oid::ObjectId,
        comment_id: bson::oid::ObjectId,
    ) -> Result<(), Self::Error> {
        let mut bookmakrs = self.bookmarks.lock().unwrap();
        for bookmark in bookmakrs.iter_mut() {
            if bookmark.key.user_id == user_id.clone().into()
                && bookmark.key.comment_id == comment_id.clone().into()
            {
                bookmakrs.retain(|bookmark| {
                    bookmark.key.user_id != user_id.clone().into()
                        && bookmark.key.comment_id != comment_id.into()
                });
                return Ok(());
            }
        }
        Err(DummyBookmarkDataSource("bookmark not found".to_string()))
    }

    async fn get_bookmark(
        &self,
        user_id: ObjectId,
        comment_id: ObjectId,
    ) -> Result<Option<CommentBookmark>, Self::Error> {
        let bookmarks = self.bookmarks.lock().unwrap();
        for bookmark in bookmarks.iter() {
            if bookmark.key.user_id == user_id.clone().into()
                && bookmark.key.comment_id == comment_id.clone().into()
            {
                return Ok(Some(bookmark.clone()));
            }
        }
        Ok(None)
    }

    async fn get_bookmarks_of_user(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Result<CommentBookmark, Self::Error>>, Self::Error> {
        let bookmarks = self.bookmarks.lock().unwrap().clone();
        let stream = futures_util::stream::iter(bookmarks.into_iter());
        let stream = stream.filter(move |bookmark| {
            let bookmark = bookmark.clone();
            async move { bookmark.key.user_id == user_id.clone().into() }
        });
        Ok(stream.map(|bookmark| Ok(bookmark)).boxed())
    }

    async fn get_bookmarks_count(&self, comment_id: ObjectId) -> Result<i32, Self::Error> {
        let bookmarks = self.bookmarks.lock().unwrap();
        let mut count = 0;
        for bookmark in bookmarks.iter() {
            if bookmark.key.comment_id == comment_id.clone().into() {
                count += 1;
            }
        }
        Ok(count)
    }
}
