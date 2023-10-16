use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::{options::ClientOptions, Client, Database};

use crate::data_source::comment::comment_data_error::CommentDataSourceError;
use crate::data_source::comment::{LikeDataSource, LikeDataSourceError};
use crate::models::comment::like::Key;
use crate::models::cv_details::CVDetails;
use crate::models::education::Education;
use crate::models::friend_request::FriendRequest;
use crate::models::sex::Sex;
use crate::mongo::mongo::bson::doc;
use crate::services::cv_service::comment_service::CommentServiceError;
use crate::services::cv_service::error::CVServiceError;
use crate::services::user_service::error::UserServiceError;
use crate::{
    data_source::{
        CVDetailsDataSource, CommentDataSource, FriendsListDataSource, FriendsListError,
        UserDataSource, UserDataSourceError,
    },
    models::comment::{Comment, CreateCommentInput, Like, UpdateCommentInput},
};
use async_graphql::futures_util::stream::BoxStream;
use async_graphql::futures_util::stream::StreamExt;

use async_trait::async_trait;

use mongodb::bson;

use crate::models::cv::{self, CV};
use crate::models::users::{self, User};

use crate::data_source::CVDataSource;
use crate::data_source::CVDataSourceError;

const FRIEND_REQUEST_COLLECTION: &str = "friend_requests";
const CV_COLLECTION: &str = "cvs";
const USER_COLLECTION: &str = "users";
const APP_NAME: &str = "SeeVi";
const COMMENT_COLLECTION: &str = "comments";
const LIKE_COLLECTION: &str = "likes";
pub struct MongoDB {
    client: Client,
    pub db: Database,
}

#[allow(dead_code)]
impl MongoDB {
    pub async fn client(&self) -> Client {
        self.client.clone()
    }

    pub async fn init() -> MongoDB {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017")
            .await
            .expect("Failed to parse options!");
        client_options.app_name = Some(APP_NAME.to_string());
        let client = Client::with_options(client_options).expect("Failed to initialize database!");
        let db = client.database("tmp");
        MongoDB { client, db }
    }

    pub async fn init_with_database_name(name: &str) -> MongoDB {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017")
            .await
            .expect("Failed to parse options!");
        client_options.app_name = Some("SeeVi".to_string());
        let client = Client::with_options(client_options).expect("Failed to initialize database!");
        let db = client.database(name);
        db.drop(None).await.unwrap();
        MongoDB { client, db }
    }
}

fn update_input_to_bson(input: users::UpdateUserInput) -> bson::Document {
    let mut update = bson::doc! {};
    input
        .username
        .map(|username| update.insert("username", username));
    input
        .first_name
        .map(|first_name| update.insert("first_name", first_name));
    input
        .last_name
        .map(|last_name| update.insert("last_name", last_name));
    input
        .country
        .map(|country| update.insert("country", country));
    input.skills.map(|skills| update.insert("skills", skills));
    input
        .primary_email
        .map(|primary_email| update.insert("primary_email", primary_email));
    input.about.map(|about| update.insert("about", about));
    input.educations.map(|education| {
        update.insert(
            "educations",
            bson::to_bson::<Vec<Education>>(&education).unwrap(),
        )
    });
    input
        .experiences
        .map(|exp| update.insert("experiences", exp));
    let update = bson::doc! {"$set": update};
    update
}

impl From<UserDataSourceError> for UserServiceError {
    fn from(error: UserDataSourceError) -> Self {
        match error {
            UserDataSourceError::IdNotFound(id) => UserServiceError::IdNotFound(id),
            UserDataSourceError::UsernameNotFound(username) => {
                UserServiceError::UsernameNotFound(username)
            }
            UserDataSourceError::EmailNotFound(email) => UserServiceError::EmailNotFound(email),
            UserDataSourceError::InvalidUsername(username) => {
                UserServiceError::InvalidUsername(username)
            }
            UserDataSourceError::InvalidEmail(email) => UserServiceError::InvalidEmail(email),
            UserDataSourceError::InvalidNameField(name) => UserServiceError::InvalidNameField(name),
            UserDataSourceError::CreateUserFailed => UserServiceError::CreateUserFailed,
            UserDataSourceError::WrongEmailUsernameOrPassword => {
                UserServiceError::WrongEmailUsernameOrPassword
            }
            UserDataSourceError::InvalidPassword => UserServiceError::InvalidPassword,
            UserDataSourceError::UpdateUserFailed => UserServiceError::UpdateUserFailed,
            UserDataSourceError::InvalidToken => UserServiceError::InvalidToken,
            UserDataSourceError::UsernameTaken(username) => {
                UserServiceError::UsernameTaken(username)
            }
            UserDataSourceError::EmailTaken(email) => UserServiceError::EmailTaken(email),
            UserDataSourceError::EmptyUsername => UserServiceError::EmptyUsername,
            UserDataSourceError::EmptyEmail => UserServiceError::EmptyEmail,
            UserDataSourceError::EmptyName => UserServiceError::EmptyName,
            UserDataSourceError::DatabaseError => UserServiceError::DatabaseError,
        }
    }
}

impl std::error::Error for UserDataSourceError {}

// Implement datasource for MongoDB
#[async_trait]
impl UserDataSource for MongoDB {
    type Error = UserDataSourceError;
    async fn get_user_by_id(&self, id: bson::oid::ObjectId) -> Result<users::User, Self::Error> {
        let collection: mongodb::Collection<users::User> = self.db.collection(USER_COLLECTION);
        let filter = bson::doc! {"_id": id};
        let result = collection.find_one(filter, None).await;
        match result {
            Ok(user) => match user {
                Some(user) => Ok(user),
                None => Err(UserDataSourceError::IdNotFound(id)),
            },
            Err(_) => Err(UserDataSourceError::IdNotFound(id)),
        }
    }

    async fn get_user_by_username(&self, username: &str) -> Result<users::User, Self::Error> {
        let collection: mongodb::Collection<users::User> = self.db.collection(USER_COLLECTION);
        let filter = bson::doc! {"username": username.clone()};
        let result = collection.find_one(filter, None).await;
        match result {
            Ok(user) => match user {
                Some(user) => Ok(user),
                None => Err(UserDataSourceError::UsernameNotFound(username.to_string())),
            },
            Err(_) => Err(UserDataSourceError::UsernameNotFound(username.to_string())),
        }
    }

    async fn create_user(&self, input: users::CreateUserInput) -> Result<users::User, Self::Error> {
        let collection = self.db.collection::<User>(USER_COLLECTION);
        let username = input.username.clone();
        let user: users::User = users::User::from(input);
        let filter = bson::doc! {"username" : &username};
        let check_username_already_exist = collection
            .find_one(filter, None)
            .await
            .expect("find one user failed");
        match check_username_already_exist {
            Some(_) => Err(UserDataSourceError::UsernameTaken(username)),
            None => {
                let result = collection.insert_one(&user, None).await;
                match result {
                    Ok(_) => Ok(user),
                    Err(_) => Err(UserDataSourceError::InvalidUsername(username)),
                }
            }
        }
    }

    async fn update_user_info(
        &self,
        input: users::UpdateUserInput,
    ) -> Result<users::User, Self::Error> {
        let collection: mongodb::Collection<users::User> = self.db.collection(USER_COLLECTION);
        let filter = bson::doc! {"_id": input.user_id};
        let user_id = input.user_id.clone();
        let update = update_input_to_bson(input);
        let result = collection
            .find_one_and_update(
                filter.clone(),
                update,
                FindOneAndUpdateOptions::builder()
                    .return_document(ReturnDocument::After)
                    .build(),
            )
            .await
            .expect("update user failed");

        // let updated_user = collection.find_one(filter.clone(), None).await.unwrap();
        // .expect("could not find user after updating");
        match result {
            Some(user) => Ok(user),
            None => Err(UserDataSourceError::IdNotFound(user_id)),
        }
    }

    async fn delete_user(&self, id: bson::oid::ObjectId) -> Result<users::User, Self::Error> {
        let collection: mongodb::Collection<users::User> = self.db.collection(USER_COLLECTION);
        let filter = bson::doc! {"_id": id};
        let user = self.get_user_by_id(id).await;
        let result = collection.delete_one(filter, None).await;
        match result {
            Ok(_) => user,
            Err(_) => Err(UserDataSourceError::IdNotFound(id)),
        }
    }

    async fn update_avatar(&self, _photo_id: bson::Uuid) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn update_cover_photo(&self, _photo_id: bson::Uuid) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn add_other_email(&self, _email: String) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn delete_other_email(&self, _email: String) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn get_user_by_email(&self, _email: &str) -> Result<users::User, Self::Error> {
        unimplemented!()
    }

    async fn get_users_by_ids(
        &self,
        user_ids: Vec<bson::oid::ObjectId>,
    ) -> BoxStream<Result<User, Self::Error>> {
        let collection: mongodb::Collection<users::User> = self.db.collection(USER_COLLECTION);
        let list_ids = user_ids;
        let filter = bson::doc! {"_id": {"$in": list_ids}};
        let cursor = collection.find(filter, None).await.unwrap();
        let stream = cursor
            .map(|result| match result {
                Ok(doc) => Ok(doc),
                Err(_) => Err(UserDataSourceError::DatabaseError),
            })
            .boxed();
        stream
    }
}

#[async_trait]
impl CVDataSource for MongoDB {
    async fn get_cvs_by_user_id(
        &self,
        user_id: ObjectId,
    ) -> Result<BoxStream<Result<cv::CV, CVDataSourceError>>, CVDataSourceError> {
        let collection: mongodb::Collection<cv::CV> = self.db.collection(CV_COLLECTION);
        let filter = bson::doc! {"author_id": user_id};
        let result = collection.find(filter, None).await;
        let cursor = match result {
            Ok(cursor) => cursor,
            Err(_) => return Err(CVDataSourceError::DatabaseError),
        };
        let stream = cursor
            .map(|result| match result {
                Ok(doc) => Ok(doc),
                Err(_) => Err(CVDataSourceError::DatabaseError),
            })
            .boxed();
        Ok(stream)
    }

    async fn get_cv_by_id(&self, id: bson::oid::ObjectId) -> Result<cv::CV, CVDataSourceError> {
        let collection: mongodb::Collection<cv::CV> = self.db.collection(CV_COLLECTION);
        let filter = bson::doc! {"_id": id};
        let result = collection
            .find_one(filter, None)
            .await
            .expect("get cv failed");
        match result {
            Some(cv) => Ok(cv),
            None => Err(CVDataSourceError::IdNotFound(id)),
        }
    }

    async fn create_cv(&self, _input: cv::CreateCVInput) -> Result<cv::CV, CVDataSourceError> {
        let collection: mongodb::Collection<cv::CV> = self.db.collection(CV_COLLECTION);
        let collection_user: mongodb::Collection<users::User> = self.db.collection(USER_COLLECTION);
        let cv: cv::CV = cv::CV {
            id: bson::oid::ObjectId::new().into(),
            author_id: _input.author_id.into(),
            title: _input.title,
            description: _input.description,
            tags: _input.tags,
            comments: vec![],
            cv: Some(bson::Uuid::new()),
            created: DateTime::now(),
        };

        let filter = bson::doc! {"_id": _input.author_id};
        let result = collection_user
            .find_one(filter, None)
            .await
            .expect("find author cv failed");
        match result {
            Some(_) => {
                let cv_clone = cv.clone();
                collection
                    .insert_one(cv, None)
                    .await
                    .expect("insert CV failed");
                Ok(cv_clone)
            }
            None => Err(CVDataSourceError::AuthorIdNotFound(_input.author_id)),
        }
    }

    async fn update_cv_info(&self, _input: cv::UpdateCVInput) -> Result<cv::CV, CVDataSourceError> {
        unimplemented!()
    }

    async fn delete_cv(&self, id: bson::oid::ObjectId) -> Result<(), CVDataSourceError> {
        let collection: mongodb::Collection<cv::CV> = self.db.collection(CV_COLLECTION);
        let filter = bson::doc! {"_id": id};
        let result = collection.delete_one(filter, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(CVDataSourceError::IdNotFound(id)),
        }
    }

    async fn find_and_update_cv(
        &self,
        _cv_id: bson::oid::ObjectId,
        _input: cv::UpdateCVInput,
    ) -> Result<cv::CV, CVDataSourceError> {
        todo!()
    }
}

#[async_trait]
impl FriendsListDataSource for MongoDB {
    async fn add_friend_request(
        &self,
        _friend_request: FriendRequest,
    ) -> Result<(), FriendsListError> {
        let collection: mongodb::Collection<FriendRequest> =
            self.db.collection(FRIEND_REQUEST_COLLECTION);
        let filter = bson::doc! {"$or" : [
            {"_id.from": _friend_request.id.from, "_id.to": _friend_request.id.to},
            {"_id.from": _friend_request.id.to, "_id.to": _friend_request.id.from}
        ]};

        let find = collection.find_one(filter, None).await;
        match find {
            Ok(_) => match find.unwrap() {
                Some(_) => Err(FriendsListError::FriendRequestAlreadyExist),
                None => {
                    let result = collection.insert_one(&_friend_request, None).await;
                    match result {
                        Ok(_) => Ok(()),
                        Err(_) => Err(FriendsListError::DatabaseError),
                    }
                }
            },
            Err(_) => Err(FriendsListError::DatabaseError),
        }
    }

    async fn update_friend_request(
        &self,
        _friend_request: FriendRequest,
    ) -> Result<(), FriendsListError> {
        let collection = self
            .db
            .collection::<FriendRequest>(FRIEND_REQUEST_COLLECTION);
        let filter = bson::doc! {
            "$or": [
                {"_id.from": _friend_request.id.from, "_id.to": _friend_request.id.to},
                {"_id.from": _friend_request.id.to, "_id.to": _friend_request.id.from}
            ]
        };

        let find = collection.find_one(filter.clone(), None).await;
        match find {
            Ok(_) => match find.unwrap() {
                Some(_) => {
                    let update =
                        bson::doc! {"$set": {"status": _friend_request.status.to_string()}};
                    let result = collection
                        .find_one_and_update(
                            filter,
                            update,
                            FindOneAndUpdateOptions::builder()
                                .return_document(ReturnDocument::After)
                                .build(),
                        )
                        .await;
                    match result {
                        Ok(_) => Ok(()),
                        Err(_) => Err(FriendsListError::DatabaseError),
                    }
                }
                None => Err(FriendsListError::FriendRequestNotFound),
            },
            Err(_) => Err(FriendsListError::DatabaseError),
        }
    }

    async fn get_friend_request(
        &self,
        from: bson::oid::ObjectId,
        to: bson::oid::ObjectId,
    ) -> Result<FriendRequest, FriendsListError> {
        let collection = self
            .db
            .collection::<FriendRequest>(FRIEND_REQUEST_COLLECTION);
        let filter = bson::doc! {"$or": [
            {"_id.from": from, "_id.to": to},
            {"_id.from": to, "_id.to": from}
        ]};
        let result = collection.find_one(filter, None).await;
        match result {
            Ok(friend_request) => match friend_request {
                Some(friend_request) => Ok(friend_request),
                None => Err(FriendsListError::FriendRequestNotFound),
            },
            Err(_) => Err(FriendsListError::DatabaseError),
        }
    }

    /// Return the list of friend requests of the user.
    async fn friend_requests(
        &self,
        _user_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, FriendsListError>> {
        let collection: mongodb::Collection<FriendRequest> =
            self.db.collection(FRIEND_REQUEST_COLLECTION);
        let filter = bson::doc! {"_id.to" : _user_id};

        let cursor = collection.find(filter, None).await.unwrap();
        let stream = cursor
            .map(|result| match result {
                Ok(doc) => Ok(doc),
                Err(_) => Err(FriendsListError::DatabaseError),
            })
            .boxed();
        stream
    }

    /// Return the list of friend requests sent by the user.
    async fn friend_requests_sent(
        &self,
        _user_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, FriendsListError>> {
        let collection: mongodb::Collection<FriendRequest> =
            self.db.collection(FRIEND_REQUEST_COLLECTION);
        let filter = bson::doc! {"_id.from" : _user_id};

        let cursor = collection.find(filter, None).await.unwrap();
        let stream = cursor
            .map(|result| match result {
                Ok(doc) => Ok(doc),
                Err(_) => Err(FriendsListError::DatabaseError),
            })
            .boxed();
        stream
    }

    async fn accepted_friend_requests(
        &self,
        _friend_request_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, FriendsListError>> {
        let collection: mongodb::Collection<FriendRequest> =
            self.db.collection(FRIEND_REQUEST_COLLECTION);
        let filter = bson::doc! {"$or" : [
            {"_id.from": _friend_request_id, "status": "Accepted"},
            {"_id.to": _friend_request_id, "status": "Accepted"}
        ]};

        let cursor = collection.find(filter, None).await.unwrap();
        let stream = cursor
            .map(|result| match result {
                Ok(doc) => Ok(doc),
                Err(_) => Err(FriendsListError::DatabaseError),
            })
            .boxed();
        stream
    }
}

impl From<CVDataSourceError> for CVServiceError {
    fn from(error: CVDataSourceError) -> Self {
        match error {
            CVDataSourceError::DatabaseError => CVServiceError::DatabaseError,
            CVDataSourceError::IdNotFound(objectid) => CVServiceError::ObjectIdNotFound(objectid),
            CVDataSourceError::TooLongDescription => CVServiceError::TooLongDescription,
            CVDataSourceError::EmptyTitle => CVServiceError::EmptyTitle,
            CVDataSourceError::EmptyId => CVServiceError::EmptyId,
            CVDataSourceError::InvalidTitle(s) => CVServiceError::InvalidTitle(s),
            CVDataSourceError::InvalidId(objectid) => CVServiceError::InvalidId(objectid),
            CVDataSourceError::TooLongTitle => CVServiceError::TooLongTitle,
            CVDataSourceError::AuthorIdNotFound(objectid) => {
                CVServiceError::AuthorIdNotFound(objectid)
            }
            CVDataSourceError::QueryFail => CVServiceError::QueryFail,
            CVDataSourceError::AddCommentFailed => CVServiceError::AddCommentFailed,
            CVDataSourceError::RemoveCommentFailed => CVServiceError::RemoveCommentFailed,
        }
    }
}

impl std::error::Error for CVDataSourceError {}

#[async_trait]
impl CVDetailsDataSource for MongoDB {
    type Error = CVDataSourceError;
    async fn get_cvs_by_filter(&self, cv_details: CVDetails) -> Result<BoxStream<CV>, Self::Error> {
        let user_collection: mongodb::Collection<User> = self.db.collection("users");
        let cv_collection: mongodb::Collection<CV> = self.db.collection("cvs");

        let mut user_filter = bson::doc! {
            "country": cv_details.country,
            "city": cv_details.city,
            "personalities" : { "$in" : cv_details.personalities},
            "experiences" : cv_details.experiences,
            "sex": bson::to_bson::<Sex>(&cv_details.sex.unwrap()).unwrap()
        };
        if cv_details.major != None {
            user_filter.insert(
                "educations",
                bson::doc! { "$elemMatch" : {"major" : cv_details.major.unwrap()}},
            );
        }
        if cv_details.rating != None {
            let rating_query = bson::doc! {"$gte" : cv_details.rating.clone().unwrap().lower, "$lte" : cv_details.rating.unwrap().upper};
            user_filter.insert("rating", rating_query);
        }
        let user_cursor_result = user_collection.find(user_filter, None).await;
        match user_cursor_result {
            Ok(cursor) => {
                let list_author_id = cursor
                    .map(|user| bson::oid::ObjectId::from(user.unwrap().id))
                    .collect::<Vec<_>>()
                    .await;
                if list_author_id.is_empty() {
                    return Err(CVDataSourceError::QueryFail);
                }
                let cv_filter = bson::doc! {
                    "author_id": {"$in": list_author_id},
                    "$or" :[
                        {"tags": {"$in": cv_details.search_words.clone()}},
                        {"title": {"$in": cv_details.search_words.clone()}},
                        ],

                };
                let cv_cursor_result = cv_collection.find(cv_filter, None).await;
                match cv_cursor_result {
                    Ok(cursor) => Ok(Box::pin(cursor.map(|result| result.unwrap()))),
                    Err(_) => Err(CVDataSourceError::QueryFail),
                }
            }
            Err(_) => Err(CVDataSourceError::QueryFail),
        }
    }
}

impl From<CommentDataSourceError> for CommentServiceError {
    fn from(error: CommentDataSourceError) -> Self {
        match error {
            CommentDataSourceError::IdNotFound(id) => CommentServiceError::IdNotFound(id),
            CommentDataSourceError::EmptyContent => CommentServiceError::EmptyContent,
            CommentDataSourceError::NoLikes => CommentServiceError::NoLikes,
            CommentDataSourceError::NoBookmarks => CommentServiceError::NoBookmarks,
            CommentDataSourceError::CreateCommentFailed => CommentServiceError::CreateCommentFailed,
            CommentDataSourceError::UpdateCommentFailed => CommentServiceError::UpdateCommentFailed,
            CommentDataSourceError::DeleteCommentFailed => CommentServiceError::DeleteCommentFailed,
            CommentDataSourceError::DatabaseError => CommentServiceError::DatabaseError,
        }
    }
}

impl std::error::Error for CommentDataSourceError {}

#[async_trait]
impl CommentDataSource for MongoDB {
    type Error = CommentDataSourceError;
    async fn get_comment_by_id(&self, id: bson::oid::ObjectId) -> Result<Comment, Self::Error> {
        let collection = self.db.collection::<Comment>(COMMENT_COLLECTION);
        let filter = bson::doc! {"_id": id};
        let result = collection.find_one(filter, None).await;
        match result {
            Ok(comment) => match comment {
                Some(comment) => Ok(comment),
                None => Err(CommentDataSourceError::IdNotFound(id)),
            },
            Err(_) => Err(CommentDataSourceError::DatabaseError),
        }
    }

    async fn get_comments_list(
        &self,
        ids: Vec<ObjectId>,
    ) -> BoxStream<Result<Comment, Self::Error>> {
        let collection = self.db.collection::<Comment>(COMMENT_COLLECTION);
        let list_ids = ids;
        let filter = bson::doc! {"_id": {"$in": list_ids}};
        let cursor = collection.find(filter, None).await.unwrap();
        let stream = cursor
            .map(|result| match result {
                Ok(doc) => Ok(doc),
                Err(_) => Err(CommentDataSourceError::DatabaseError),
            })
            .boxed();
        stream
    }

    async fn create_comment(&self, input: CreateCommentInput) -> Result<Comment, Self::Error> {
        let comment: Comment = Comment::from(input);
        let result = self.add_comment(comment.clone()).await;
        match result {
            Ok(_) => Ok(comment),
            Err(_) => Err(CommentDataSourceError::CreateCommentFailed),
        }
    }

    async fn add_comment(&self, comment: Comment) -> Result<(), Self::Error> {
        let collection = self.db.collection::<Comment>(COMMENT_COLLECTION);
        let result = collection.insert_one(&comment, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(CommentDataSourceError::DatabaseError),
        }
    }

    async fn remove_comment(&self, id: bson::oid::ObjectId) -> Result<Comment, Self::Error> {
        let collection = self.db.collection::<Comment>(COMMENT_COLLECTION);
        let filter = bson::doc! {"_id": id};
        let result = collection.find_one_and_delete(filter, None).await;
        match result {
            Ok(comment) => match comment {
                Some(comment) => Ok(comment),
                None => Err(CommentDataSourceError::DeleteCommentFailed),
            },
            Err(_) => Err(CommentDataSourceError::DatabaseError),
        }
    }

    async fn find_and_update_comment(
        &self,
        id: bson::oid::ObjectId,
        input: UpdateCommentInput,
    ) -> Result<Comment, Self::Error> {
        let collection = self.db.collection::<Comment>(COMMENT_COLLECTION);
        let filter = bson::doc! {"_id": id};
        let update = bson::doc! {"$set": {"content": input.content, "likes": input.likes, "bookmarks": input.bookmarks, "shares": input.shares}};
        let result = collection
            .find_one_and_update(
                filter,
                update,
                FindOneAndUpdateOptions::builder()
                    .return_document(ReturnDocument::After)
                    .build(),
            )
            .await;
        match result {
            Ok(comment) => match comment {
                Some(comment) => Ok(comment),
                None => Err(CommentDataSourceError::UpdateCommentFailed),
            },
            Err(_) => Err(CommentDataSourceError::DatabaseError),
        }
    }

    async fn add_reply_to_comment(
        &self,
        comment_id: bson::oid::ObjectId,
        reply_id: bson::oid::ObjectId,
    ) -> Result<Comment, Self::Error> {
        let collection = self.db.collection::<Comment>(COMMENT_COLLECTION);
        let filter = bson::doc! {"_id": comment_id};
        let update = bson::doc! {"$push": {"replies": reply_id}};
        let result = collection
            .find_one_and_update(
                filter,
                update,
                FindOneAndUpdateOptions::builder()
                    .return_document(ReturnDocument::After)
                    .build(),
            )
            .await;
        match result {
            Ok(comment) => match comment {
                Some(comment) => Ok(comment),
                None => Err(CommentDataSourceError::IdNotFound(comment_id)),
            },
            Err(_) => Err(CommentDataSourceError::DatabaseError),
        }
    }

    async fn find_and_remove_reply(
        &self,
        comment_id: bson::oid::ObjectId,
        reply_id: bson::oid::ObjectId,
    ) -> Result<Comment, Self::Error> {
        let collection = self.db.collection::<Comment>(COMMENT_COLLECTION);
        let filter = bson::doc! {"_id": comment_id};
        let update = bson::doc! {"$pull": {"replies": reply_id}};
        let result = collection
            .find_one_and_update(
                filter,
                update,
                FindOneAndUpdateOptions::builder()
                    .return_document(ReturnDocument::After)
                    .build(),
            )
            .await;
        match result {
            Ok(comment) => match comment {
                Some(comment) => Ok(comment),
                None => Err(CommentDataSourceError::IdNotFound(comment_id)),
            },
            Err(_) => Err(CommentDataSourceError::DatabaseError),
        }
    }
}

impl std::error::Error for LikeDataSourceError {}

impl From<LikeDataSourceError> for CommentServiceError {
    fn from(value: LikeDataSourceError) -> Self {
        match value {
            LikeDataSourceError::AddLikesFail => CommentServiceError::UpdateCommentFailed,
            LikeDataSourceError::DeleteLikesFail => CommentServiceError::UpdateCommentFailed,
            LikeDataSourceError::InvalidCommentId(id) => CommentServiceError::IdNotFound(id),
            LikeDataSourceError::InvalidUserId(id) => CommentServiceError::IdNotFound(id),
            LikeDataSourceError::LikeNotFound => CommentServiceError::NoLikes,
            LikeDataSourceError::LikesNumberNotFound => CommentServiceError::NoLikes,
            LikeDataSourceError::LikeAlreadyExists => CommentServiceError::UpdateCommentFailed,
            LikeDataSourceError::QueryFail => CommentServiceError::UpdateCommentFailed,
        }
    }
}

#[async_trait]
impl LikeDataSource for MongoDB {
    type Error = LikeDataSourceError;

    async fn add_like(
        &self,
        user_id: bson::oid::ObjectId,
        comment_id: bson::oid::ObjectId,
    ) -> Result<(), Self::Error> {
        let comment_collection: mongodb::Collection<Comment> = self.db.collection(LIKE_COLLECTION);
        let like_collection: mongodb::Collection<Like> = self.db.collection(COMMENT_COLLECTION);
        let filter = bson::doc! {
            "key.user_id": user_id.clone(),
            "key.comment_id": comment_id.clone(),
        };
        let result_exist = like_collection.find_one(filter, None).await;
        match result_exist {
            Ok(like_option) => match like_option {
                Some(_) => Err(LikeDataSourceError::LikeAlreadyExists),
                None => {
                    let like = Like {
                        key: Key {
                            user_id: user_id.clone().into(),
                            comment_id: comment_id.clone().into(),
                        },
                        created: DateTime::now(),
                    };
                    let result_add = like_collection.insert_one(like, None).await;
                    match result_add {
                        Ok(_) => Ok(()),
                        Err(_) => Err(LikeDataSourceError::AddLikesFail),
                    }
                }
            },
            Err(err) => Err(LikeDataSourceError::QueryFail),
        }
    }

    async fn delete_like(
        &self,
        user_id: bson::oid::ObjectId,
        comment_id: bson::oid::ObjectId,
    ) -> Result<(), Self::Error> {
        let comment_collection: mongodb::Collection<Comment> = self.db.collection(LIKE_COLLECTION);
        let like_collection: mongodb::Collection<Like> = self.db.collection(COMMENT_COLLECTION);
        let filter = bson::doc! {
            "key.user_id": user_id,
            "key.comment_id": comment_id,
        };
        let result_delete = like_collection.find_one_and_delete(filter, None).await;
        match result_delete {
            Ok(like_option) => Ok(()),
            Err(err) => Err(LikeDataSourceError::DeleteLikesFail),
        }
    }

    async fn get_likes_count(&self, comment_id: bson::oid::ObjectId) -> Result<i32, Self::Error> {
        let collection: mongodb::Collection<Comment> = self.db.collection(COMMENT_COLLECTION);
        let filter = bson::doc! {"_id.comment_id": comment_id};
        let result = collection.count_documents(filter, None).await;
        match result {
            Ok(count) => Ok(count as i32),
            Err(err) => Err(LikeDataSourceError::QueryFail),
        }
    }

    async fn get_likes(
        &self,
        comment_id: bson::oid::ObjectId,
    ) -> Result<BoxStream<Like>, Self::Error> {
        let collection: mongodb::Collection<Like> = self.db.collection(LIKE_COLLECTION);
        let filter = bson::doc! {
            "key.comment_id": comment_id,
        };
        let cursor_result = collection.find(filter, None).await;
        match cursor_result {
            Ok(cursor) => Ok(cursor.map(|like| like.unwrap()).boxed()),
            Err(err) => Err(LikeDataSourceError::LikeNotFound),
        }
    }
}
