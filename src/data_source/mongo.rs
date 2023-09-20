use mongodb::bson::DateTime;
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::{options::ClientOptions, Client, Database};

use crate::data_source::user_data_source::UserDataSource;

use crate::data_source::user_data_source_error::UserDataSourceError;
use crate::models::education::Education;

use async_trait::async_trait;

use mongodb::bson;

use crate::models::cv;
use crate::models::users::{self, User};

use super::cv_data_source::CVDataSource;
use super::cv_data_source_error::CVDataSourceError;

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
        client_options.app_name = Some("SeeVi".to_string());
        let client = Client::with_options(client_options).expect("Failed to initialize database!");
        let db = client.database("tmp");
        MongoDB { client, db }
    }

    pub async fn init_test() -> MongoDB {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017")
            .await
            .expect("Failed to parse options!");
        client_options.app_name = Some("SeeVi".to_string());
        let client = Client::with_options(client_options).expect("Failed to initialize database!");
        let db = client.database("test");
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
    ) -> Result<users::User, UserDataSourceError> {
        let collection: mongodb::Collection<users::User> = self.db.collection("users");
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

    async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<users::User, UserDataSourceError> {
        let collection: mongodb::Collection<users::User> = self.db.collection("users");
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

    async fn create_user(
        &self,
        input: users::CreateUserInput,
    ) -> Result<users::User, UserDataSourceError> {
        let collection = self.db.collection::<User>("users");
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
    ) -> Result<users::User, UserDataSourceError> {
        let collection: mongodb::Collection<users::User> = self.db.collection("users");
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

    async fn delete_user(
        &self,
        id: bson::oid::ObjectId,
    ) -> Result<users::User, UserDataSourceError> {
        let collection: mongodb::Collection<users::User> = self.db.collection("users");
        let filter = bson::doc! {"_id": id};
        let user = self.get_user_by_id(id).await;
        let result = collection.delete_one(filter, None).await;
        match result {
            Ok(_) => user,
            Err(_) => Err(UserDataSourceError::IdNotFound(id)),
        }
    }

    async fn update_avatar(&self, _photo_id: bson::Uuid) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }

    async fn update_cover_photo(&self, _photo_id: bson::Uuid) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }

    async fn add_other_email(&self, _email: String) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }

    async fn delete_other_email(&self, _email: String) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }

    async fn get_user_by_email(&self, _email: &str) -> Result<users::User, UserDataSourceError> {
        unimplemented!()
    }
}

#[async_trait]
impl CVDataSource for MongoDB {
    async fn get_cv_by_id(&self, id: bson::oid::ObjectId) -> Result<cv::CV, CVDataSourceError> {
        let collection: mongodb::Collection<cv::CV> = self.db.collection("cvs");
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
        let collection: mongodb::Collection<cv::CV> = self.db.collection("cvs");
        let collection_user: mongodb::Collection<users::User> = self.db.collection("users");
        let cv: cv::CV = cv::CV {
            id: bson::oid::ObjectId::new(),
            author_id: _input.author_id,
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
        let collection: mongodb::Collection<cv::CV> = self.db.collection("cvs");
        let filter = bson::doc! {"_id": id};
        let result = collection.delete_one(filter, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(CVDataSourceError::IdNotFound(id)),
        }
    }
}
