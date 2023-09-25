use mongodb::bson::DateTime;
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::{options::ClientOptions, Client, Database};

use crate::data_source::{DataSourceError, FriendsListDataSource, UserDataSource, CVDataSource};
use crate::models::education::Education;
use crate::models::friend_request::FriendRequest;

use async_graphql::futures_util::stream::BoxStream;
use async_graphql::futures_util::stream::StreamExt;

use async_trait::async_trait;

use mongodb::bson;

use crate::models::cv;
use crate::models::users::{self, User};

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

// Implement datasource for MongoDB
#[async_trait]
impl UserDataSource for MongoDB {
    async fn get_user_by_id(
        &self,
        id: bson::oid::ObjectId,
    ) -> Result<users::User, DataSourceError> {
        let collection: mongodb::Collection<users::User> = self.db.collection(USER_COLLECTION);
        let filter = bson::doc! {"_id": id};
        let result = collection.find_one(filter, None).await;
        match result {
            Ok(user) => match user {
                Some(user) => Ok(user),
                None => Err(DataSourceError::NotFound),
            },
            Err(e) => Err(DataSourceError::InternalServerError(e)),
        }
    }

    async fn get_user_by_username(&self, username: &str) -> Result<users::User, DataSourceError> {
        let collection: mongodb::Collection<users::User> = self.db.collection(USER_COLLECTION);
        let filter = bson::doc! {"username": username.clone()};
        let result = collection.find_one(filter, None).await;
        match result {
            Ok(user) => match user {
                Some(user) => Ok(user),
                None => Err(DataSourceError::NotFound),
            },
            Err(e) => Err(DataSourceError::InternalServerError(e)),
        }
    }

    async fn create_user(
        &self,
        input: users::CreateUserInput,
    ) -> Result<users::User, DataSourceError> {
        let collection = self.db.collection::<User>(USER_COLLECTION);
        let username = input.username.clone();
        let user: users::User = users::User::from(input);
        let filter = bson::doc! {"username" : &username};
        let check_username_already_exist = collection
            .find_one(filter, None)
            .await
            .expect("find one user failed");
        match check_username_already_exist {
            Some(_) => Err(DataSourceError::custom(format!(
                "Username {} already exist",
                username
            ))),
            None => {
                let result = collection.insert_one(&user, None).await;
                match result {
                    Ok(_) => Ok(user),
                    Err(e) => Err(DataSourceError::InternalServerError(e)),
                }
            }
        }
    }

    async fn update_user_info(
        &self,
        input: users::UpdateUserInput,
    ) -> Result<users::User, DataSourceError> {
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
            .await?;

        // let updated_user = collection.find_one(filter.clone(), None).await.unwrap();
        // .expect("could not find user after updating");
        match result {
            Some(user) => Ok(user),
            None => Err(DataSourceError::NotFound),
        }
    }

    async fn delete_user(&self, id: bson::oid::ObjectId) -> Result<users::User, DataSourceError> {
        let collection: mongodb::Collection<users::User> = self.db.collection(USER_COLLECTION);
        let filter = bson::doc! {"_id": id};
        let user = self.get_user_by_id(id).await;
        let result = collection.delete_one(filter, None).await?;
    }
}

#[async_trait]
impl CVDataSource for MongoDB {
    async fn get_cv_by_id(&self, id: bson::oid::ObjectId) -> Result<cv::CV, DataSourceError> {
        let collection: mongodb::Collection<cv::CV> = self.db.collection(CV_COLLECTION);
        let filter = bson::doc! {"_id": id};
        let result = collection.find_one(filter, None).await?;
        match result {
            Some(cv) => Ok(cv),
            None => Err(DataSourceError::NotFound),
        }
    }

    async fn create_cv(&self, _input: cv::CreateCVInput) -> Result<cv::CV, DataSourceError> {
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
        let result = collection_user.find_one(filter, None).await?;
        match result {
            Some(_) => {
                let cv_clone = cv.clone();
                collection.insert_one(cv, None).await?;
                Ok(cv_clone)
            }
            None => Err(DataSourceError::NotFound),
        }
    }

    async fn update_cv_info(&self, _input: cv::UpdateCVInput) -> Result<cv::CV, DataSourceError> {
        unimplemented!()
    }

    async fn delete_cv(&self, id: bson::oid::ObjectId) -> Result<(), DataSourceError> {
        let collection: mongodb::Collection<cv::CV> = self.db.collection(CV_COLLECTION);
        let filter = bson::doc! {"_id": id};
        let result = collection.delete_one(filter, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DataSourceError::NotFound),
        }
    }
}

#[async_trait]
impl FriendsListDataSource for MongoDB {
    async fn add_friend_request(
        &self,
        _friend_request: FriendRequest,
    ) -> Result<(), DataSourceError> {
        let collection: mongodb::Collection<FriendRequest> =
            self.db.collection(FRIEND_REQUEST_COLLECTION);
        let filter = bson::doc! {"$or" : [
            {"_id.from": _friend_request._id.from, "_id.to": _friend_request._id.to},
            {"_id.from": _friend_request._id.to, "_id.to": _friend_request._id.from}
        ]};

        let find = collection.find_one(filter, None).await?;
        match find.unwrap() {
            Some(_) => Err(DataSourceError::custom(format!(
                "Friend request already exist"
            ))),
            None => {
                let result = collection.insert_one(&_friend_request, None).await?;
                Ok(())
            }
        }
    }

    async fn update_friend_request(
        &self,
        _friend_request: FriendRequest,
    ) -> Result<(), DataSourceError> {
        let collection = self
            .db
            .collection::<FriendRequest>(FRIEND_REQUEST_COLLECTION);
        let filter = bson::doc! {
            "$or": [
                {"_id.from": _friend_request._id.from, "_id.to": _friend_request._id.to},
                {"_id.from": _friend_request._id.to, "_id.to": _friend_request._id.from}
            ]
        };

        let find = collection.find_one(filter.clone(), None).await?;
        match find.unwrap() {
            Some(_) => {
                let update = bson::doc! {"$set": {"status": _friend_request.status.to_string()}};
                let result = collection
                    .find_one_and_update(
                        filter,
                        update,
                        FindOneAndUpdateOptions::builder()
                            .return_document(ReturnDocument::After)
                            .build(),
                    )
                    .await?;
                Ok(())
            }
            None => Err(DataSourceError::NotFound),
        }
    }

    async fn get_friend_request(
        &self,
        from: bson::oid::ObjectId,
        to: bson::oid::ObjectId,
    ) -> Result<FriendRequest, DataSourceError> {
        let collection = self
            .db
            .collection::<FriendRequest>(FRIEND_REQUEST_COLLECTION);
        let filter = bson::doc! {"$or": [
            {"_id.from": from, "_id.to": to},
            {"_id.from": to, "_id.to": from}
        ]};
        let friend_request = collection.find_one(filter, None).await?;
        match friend_request {
            Some(friend_request) => Ok(friend_request),
            None => Err(DataSourceError::NotFound),
        }
    }

    /// Return the list of friend requests of the user.
    async fn friend_requests(
        &self,
        _user_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, DataSourceError>> {
        let collection: mongodb::Collection<FriendRequest> =
            self.db.collection(FRIEND_REQUEST_COLLECTION);
        let filter = bson::doc! {"_id.to" : _user_id};

        let cursor = collection.find(filter, None).await.unwrap();
        let stream = cursor
            .map(|result| match result {
                Ok(doc) => Ok(doc),
                Err(err) => Err(DataSourceError::InternalServerError(err)),
            })
            .boxed();
        stream
    }

    /// Return the list of friend requests sent by the user.
    async fn friend_requests_sent(
        &self,
        _user_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, DataSourceError>> {
        let collection: mongodb::Collection<FriendRequest> =
            self.db.collection(FRIEND_REQUEST_COLLECTION);
        let filter = bson::doc! {"_id.from" : _user_id};

        let cursor = collection.find(filter, None).await.unwrap();
        let stream = cursor
            .map(|result| match result {
                Ok(doc) => Ok(doc),
                Err(err) => Err(DataSourceError::InternalServerError(err)),
            })
            .boxed();
        stream
    }

    async fn accepted_friend_requests(
        &self,
        _friend_request_id: bson::oid::ObjectId,
    ) -> BoxStream<Result<FriendRequest, DataSourceError>> {
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
                Err(err) => Err(DataSourceError::InternalServerError(err)),
            })
            .boxed();
        stream
    }
}
