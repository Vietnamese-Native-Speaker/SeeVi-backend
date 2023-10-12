use mongodb::bson::oid::ObjectId;
use mongodb::bson::{DateTime, Uuid};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::{options::ClientOptions, Client, Database};
use std::pin::Pin;
use crate::data_source::user_data_source::UserDataSource;

use crate::data_source::user_data_source_error::UserDataSourceError;
use crate::models::cv_details::cv_details::CVDetails;
use crate::models::education::Education;
use crate::models::sex::Sex;
use crate::services::cv_service::error::CVServiceError;

use async_trait::async_trait;

use mongodb::bson;

use crate::models::cv::{self, CV};
use crate::models::users::{self, User};
use crate::models::cv_details;
use crate::models::range_values::RangeValues;
use super::cv_data_source::CVDataSource;
use super::cv_data_source_error::CVDataSourceError;
use super::cv_details_data_source::CVDetailsDataSource;
use futures_core::stream::BoxStream;
use tokio_stream::{StreamExt, Stream};

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

// Implement datasource for MongoDB
#[async_trait]
impl UserDataSource for MongoDB {
    async fn get_user_by_id(&self, id: ObjectId) -> Result<users::User, UserDataSourceError> {
        let collection: mongodb::Collection<users::User> = self.db.collection("users");
        let filter = bson::doc! {"user_id": id};
        let result = collection.find_one(filter, None).await;
        match result {
            Ok(user) => match user {
                Some(user) => Ok(user),
                None => Err(UserDataSourceError::ObjectIdNotFound(id)),
            },
            Err(_) => Err(UserDataSourceError::ObjectIdNotFound(id)),
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
        let collection = self.db.collection("users");
        let user: users::User = users::User::from(input.clone());
        let user_clone = user.clone();
        let filter = bson::doc! {"username" : input.username.clone()};
        let check_username_already_exist = collection.find_one(filter, None).await.expect("find one user failed");
        match check_username_already_exist
        {
            Some(_) => Err(UserDataSourceError::UsernameTaken(input.username)),
            None =>{
                let result = collection.insert_one(user, None).await;
                match result {
                    Ok(_) => Ok(user_clone),
                    Err(_) => Err(UserDataSourceError::InvalidUsername(input.username.clone())),
                }
            },
        }
    }

    async fn update_user_info(
        &self,
        input: users::UpdateUserInput,
    ) -> Result<users::User, UserDataSourceError> {
        let collection: mongodb::Collection<users::User> = self.db.collection("users");
        let filter = bson::doc! {"user_id": input.user_id};
        let update = bson::doc! {"$set": {
            "user_id": input.user_id,
            "username": input.username.clone(),
            "first_name": input.first_name,
            "last_name": input.last_name,
            "country": input.country,
            "skills": input.skills,
            "primary_email": input.primary_email,
            "about": input.about,
            // TODO: load the struct "education" into the document
            "education": bson::to_bson::<Vec<Education>>(&input.education.unwrap()).unwrap(),
        }};
        
        println!("{}",update.get("$set").unwrap());
        let result = collection.find_one_and_update(filter.clone(), update, FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build()).await.expect("update user failed");
        
        // let updated_user = collection.find_one(filter.clone(), None).await.unwrap();
        // .expect("could not find user after updating");
        match result{
            Some(user) => Ok(user),
            None => Err(UserDataSourceError::ObjectIdNotFound(input.user_id))
        }
    }

    async fn delete_user(&self, id: ObjectId) -> Result<users::User, UserDataSourceError> {
        let collection: mongodb::Collection<users::User> = self.db.collection("users");
        let filter = bson::doc! {"user_id": id};
        let user = self.get_user_by_id(id).await;
        let result = collection.delete_one(filter, None).await;
        match result {
            Ok(_) => user,
            Err(_) => Err(UserDataSourceError::ObjectIdNotFound(id)),
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
    async fn get_cv_by_id(&self, id: ObjectId) -> Result<cv::CV, CVDataSourceError> {
        let collection: mongodb::Collection<cv::CV> = self.db.collection("cvs");
        let filter = bson::doc! {"_id": id};
        let result = collection.find_one(filter, None).await.expect("get cv failed");
        match result {
            Some(cv) => Ok(cv),
            None => Err(CVDataSourceError::ObjectIdNotFound(id))
        }
    }

    async fn create_cv(&self, _input: cv::CreateCVInput) -> Result<cv::CV, CVDataSourceError> {
        let collection: mongodb::Collection<cv::CV> = self.db.collection("cvs");
        let collection_user: mongodb::Collection<users::User> = self.db.collection("users");
        let cv: cv::CV = cv::CV {
            _id: ObjectId::new(),
            author_id: _input.author_id,
            title: _input.title,
            description: _input.description,
            tags: _input.tags,
            comments: vec![],
            cv: Some(bson::Uuid::new()),
            created: DateTime::now(),
        };
        
        let filter = bson::doc!{"user_id": _input.author_id};
        let result = collection_user.find_one(filter, None).await.expect("find author cv failed");
        match result {
            Some(_) => {
                let cv_clone = cv.clone();
                collection.insert_one(cv, None).await.expect("insert CV failed");
                Ok(cv_clone)
            },  
            None => Err(CVDataSourceError::AuthorIdNotFound(_input.author_id)),
        }
    }

    async fn update_cv_info(&self, _input: cv::CV) -> Result<cv::CV, CVDataSourceError> {
        unimplemented!()
    }

    async fn delete_cv(&self, id: ObjectId) -> Result<(), CVDataSourceError> {
        let collection: mongodb::Collection<cv::CV> = self.db.collection("cvs");
        let filter = bson::doc! {"_id": id};
        let result = collection.delete_one(filter, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(CVDataSourceError::ObjectIdNotFound(id)),
        }
    }
}

impl From<CVDataSourceError> for CVServiceError{
    fn from(error: CVDataSourceError) -> Self {
        match error{
            CVDataSourceError::ObjectIdNotFound(objectid) => {
                CVServiceError::ObjectIdNotFound(objectid)
            }
            CVDataSourceError::TooLongDescription => {
                CVServiceError::TooLongDescription
            }
            CVDataSourceError::EmptyTitle => {
                CVServiceError::EmptyTitle
            }
            CVDataSourceError::EmptyId => {
                CVServiceError::EmptyId
            }
            CVDataSourceError::InvalidTitle(s) => {
                CVServiceError::InvalidTitle(s)
            }
            CVDataSourceError::InvalidId(objectid) => {
                CVServiceError::InvalidId(objectid)
            }
            CVDataSourceError::TooLongTitle => {
                CVServiceError::TooLongTitle
            }
            CVDataSourceError::AuthorIdNotFound(objectid) => {
                CVServiceError::AuthorIdNotFound(objectid)
            }
            CVDataSourceError::QueryFail => {
                CVServiceError::QueryFail
            }
        }
    }
}

impl std::error::Error for CVDataSourceError{}
#[async_trait]
impl CVDetailsDataSource for MongoDB{
    type Error = CVDataSourceError;
    async fn get_cvs_by_filter(&self, cv_details: CVDetails) -> Result<Pin<Box<dyn Stream<Item = CV>>>, Self::Error> {
        let user_collection: mongodb::Collection<User> = self.db.collection("users");
        let cv_collection: mongodb::Collection<CV> = self.db.collection("cvs");

        let mut user_filter = bson::doc!{
            "country": cv_details.country,
            "city": cv_details.city,
            "personalities" : { "$in" : cv_details.personalities},
            "year_of_experience" : cv_details.year_of_experience,
            "sex": bson::to_bson::<Sex>(&cv_details.sex.unwrap()).unwrap()
        };
        if (cv_details.major != None){
            user_filter.insert("education", bson::doc!{ "$elemMatch" : {"major" : cv_details.major.unwrap()}});
        }
        if (cv_details.rating != None){
            let rating_query = bson::doc!{"$gte" : cv_details.rating.clone().unwrap().lower, "$lte" : cv_details.rating.unwrap().upper};
            user_filter.insert("rating", rating_query);
        }
        let user_cursor_result = user_collection.find(user_filter, None).await;
        match user_cursor_result {
            Ok(cursor) =>{
                let list_author_id = cursor.map(|user|user.unwrap().user_id).collect::<Vec<_>>().await;
                let cv_filter = bson::doc!{
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
                
            },
            Err(_) => Err(CVDataSourceError::QueryFail),
        }
        
    }
}
