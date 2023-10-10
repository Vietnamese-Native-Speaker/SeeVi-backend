use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::{options::ClientOptions, Client, Database};

use crate::data_source::{UserDataSource, CommentDataSource};

use crate::data_source::FriendsListDataSource;
use crate::data_source::FriendsListError;
use crate::data_source::UserDataSourceError;
use crate::models::comment::{Comment, CreateCommentInput, UpdateCommentInput};
use crate::models::education::Education;
use crate::models::friend_request::FriendRequest;
use crate::mongo::mongo::bson::doc;
use crate::services::user_service::error::UserServiceError;
use crate::services::cv_service::comment_service::CommentServiceError;

use async_graphql::futures_util::stream::BoxStream;
use async_graphql::futures_util::stream::StreamExt;

use async_trait::async_trait;

use mongodb::bson;

use crate::models::cv;
use crate::models::users::{self, User};

use crate::data_source::CVDataSource;
use crate::data_source::CVDataSourceError;

const FRIEND_REQUEST_COLLECTION: &str = "friend_requests";
const CV_COLLECTION: &str = "cvs";
const USER_COLLECTION: &str = "users";
const APP_NAME: &str = "SeeVi";

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
    input.education.map(|education| {
        update.insert(
            "education",
            bson::to_bson::<Vec<Education>>(&education).unwrap(),
        )
    });
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
                Err(err) => Err(UserDataSourceError::DatabaseError),
            })
            .boxed();
        stream
    }
}

#[async_trait]
impl CVDataSource for MongoDB {
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
                Err(err) => Err(FriendsListError::DatabaseError),
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
                Err(err) => Err(FriendsListError::DatabaseError),
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
                Err(err) => Err(FriendsListError::DatabaseError),
            })
            .boxed();
        stream
    }
}

#[async_trait]
impl CommentDataSource for MongoDB {
    async fn get_comment_by_id(&self, id: bson::oid::ObjectId) -> Result<Comment, Self::Error> {
        todo!()
    }

    async fn get_comments_list(
        &self,
        ids: Vec<ObjectId>
    ) -> BoxStream<Result<Comment, Self::Error>> {
        todo!()
    }

    async fn create_comment(&self, input: CreateCommentInput) -> Result<Comment, Self::Error> {
        todo!()
    }

    async fn add_comment(&self, comment: Comment) -> Result<(), Self::Error> {
        todo!()
    }

    async fn update_comment(&self, input: UpdateCommentInput) -> Result<Comment, Self::Error> {
        todo!()
    }

    async fn remove_comment(&self, id: bson::oid::ObjectId) -> Result<Comment, Self::Error> {
        todo!()
    }

    async fn find_and_update_comment(
        &self,
        id: bson::oid::ObjectId,
        input: UpdateCommentInput,
    ) -> Result<Comment, Self::Error> {
        todo!()
    }

    async fn add_reply_to_comment(
        &self,
        comment_id: bson::oid::ObjectId,
        reply_id: bson::oid::ObjectId,
    ) -> Result<Comment, Self::Error> {
        todo!()
    }

    async fn remove_reply_from_comment(
        &self,
        comment_id: bson::oid::ObjectId,
        reply_id: bson::oid::ObjectId,
    ) -> Result<Comment, Self::Error> {
        todo!()
    }

    async fn find_and_remove_reply(
        &self,
        comment_id: bson::oid::ObjectId,
        reply_id: bson::oid::ObjectId,
    ) -> Result<Comment, Self::Error> {
        todo!()
    }
}
