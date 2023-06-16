use mongodb::bson::DateTime;
use mongodb::{options::ClientOptions, Client, Database};

use crate::data_source::user_data_source::UserDataSource;

use crate::data_source::user_data_source_error::UserDataSourceError;

use async_trait::async_trait;

use mongodb::bson;

use crate::models::cv;
use crate::models::users;

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

    #[cfg(test)]
    pub async fn init_test() -> MongoDB {
        use crate::models::{cv::CV, users::User};

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
    async fn get_user_by_id(&self, id: bson::Uuid) -> Result<users::User, UserDataSourceError> {
        let collection: mongodb::Collection<users::User> = self.db.collection("users");
        let filter = bson::doc! {"user_id": id};
        let result = collection.find_one(filter, None).await;
        match result {
            Ok(user) => match user {
                Some(user) => Ok(user),
                None => Err(UserDataSourceError::UuidNotFound(id)),
            },
            Err(_) => Err(UserDataSourceError::UuidNotFound(id)),
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
        let user: users::User = users::User {
            user_id: bson::Uuid::new(),
            username: input.username.clone(),
            first_name: input.first_name,
            last_name: input.last_name,
            country: input.country,
            skills: input.skills,
            primary_email: input.primary_email,
            other_mails: input.other_mails,
            about: input.about,
            avatar: input.avatar,
            cover_photo: input.cover_photo,
            education: input.education,
            cv: vec![],
            friends_list: vec![],
            rating: None,
            level: None,
            shared_cvs: vec![],
            saved_cvs: vec![],
            liked_cvs: vec![],
        };
        let user_clone = user.clone();
        let filter = bson::doc! {"username" : input.username.clone()};
        let check_username_already_exist = collection.find_one(filter, None).await;
        match check_username_already_exist
        {
            Ok(temp_user) => Err(UserDataSourceError::UsernameTaken(input.username)),
            Err(_) =>{
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
        let filter = bson::doc! {"username": input.username.clone()};
        let update = bson::doc! {"$set": {
            "username": input.username.clone(),
            "first_name": input.first_name,
            "last_name": input.last_name,
            "country": input.country,
            "skills": input.skills,
            "primary_email": input.primary_email,
            "about": input.about,
            // "education": input.education,
        }};
        let result = collection.update_one(filter, update, None).await;
        match result {
            Ok(_) => {
                let return_user = self
                    .get_user_by_username(input.username.clone().unwrap().as_str())
                    .await;
                match return_user {
                    Ok(user) => Ok(user),
                    Err(_) => Err(UserDataSourceError::UsernameNotFound(
                        input.username.clone().unwrap(),
                    )),
                }
            }
            Err(_) => Err(UserDataSourceError::UsernameTaken(
                input.username.clone().unwrap(),
            )),
        }
    }

    async fn delete_user(&self, id: bson::Uuid) -> Result<users::User, UserDataSourceError> {
        let collection: mongodb::Collection<users::User> = self.db.collection("users");
        let filter = bson::doc! {"user_id": id};
        let user = self.get_user_by_id(id).await;
        let result = collection.delete_one(filter, None).await;
        match result {
            Ok(_) => user,
            Err(_) => Err(UserDataSourceError::UuidNotFound(id)),
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
    async fn get_cv_by_id(&self, id: bson::Uuid) -> Result<cv::CV, CVDataSourceError> {
        let collection: mongodb::Collection<cv::CV> = self.db.collection("cvs");
        let filter = bson::doc! {"_id": id};
        let result = collection.find_one(filter, None).await;
        match result {
            Ok(cv) => match cv {
                Some(cv) => Ok(cv),
                None => Err(CVDataSourceError::UuidNotFound(id)),
            },
            Err(_) => Err(CVDataSourceError::UuidNotFound(id)),
        }
    }

    async fn create_cv(&self, _input: cv::CreateCVInput) -> Result<cv::CV, CVDataSourceError> {
        let collection: mongodb::Collection<cv::CV> = self.db.collection("cvs");
        let cv: cv::CV = cv::CV {
            _id: bson::Uuid::new(),
            author_id: _input.author_id,
            title: _input.title,
            description: _input.description,
            tags: _input.tags,
            comments: vec![],
            cv: bson::Uuid::new(),
            created: DateTime::now(),
        };
        let cv_clone = cv.clone();
        let result = collection.insert_one(cv, None).await;
        match result {
            Ok(_) => Ok(cv_clone),
            Err(_) => Err(CVDataSourceError::UuidNotFound(_input.author_id)),
        }
    }

    async fn update_cv_info(&self, _input: cv::CV) -> Result<cv::CV, CVDataSourceError> {
        unimplemented!()
    }

    async fn delete_cv(&self, id: bson::Uuid) -> Result<(), CVDataSourceError> {
        let collection: mongodb::Collection<cv::CV> = self.db.collection("cvs");
        let filter = bson::doc! {"_id": id};
        let result = collection.delete_one(filter, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(CVDataSourceError::UuidNotFound(id)),
        }
    }
}
