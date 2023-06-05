use mongodb::{options::ClientOptions, Client, Database};

use crate::data_source::user_data_source::UserDataSource;

use crate::data_source::user_data_source_error::UserDataSourceError;

use async_trait::async_trait;

use mongodb::bson;

use crate::models::users;

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
        let db = client.database("users");
        MongoDB { client, db }
    }
}

// Implement datasource for MongoDB
#[async_trait]
impl UserDataSource for MongoDB {
    async fn get_user_by_id(&self, id: bson::Uuid) -> Result<users::User, UserDataSourceError> {
        let collection = self.db.collection("users");
        let filter = bson::doc! {"id": id};
        let result = collection.find_one(filter, None).await;
        match result {
            Ok(user) => match user {
                Some(user) => {
                    let user: users::User = bson::from_document(user).unwrap();
                    Ok(user)
                }
                None => Err(UserDataSourceError::UuidNotFound(id)),
            },
            Err(_) => Err(UserDataSourceError::UuidNotFound(id)),
        }
    }

    async fn get_user_by_username(&self, username: String) -> Result<users::User, UserDataSourceError> {
        let collection = self.db.collection("users");
        let filter = bson::doc! {"username": username.clone()};
        let result = collection.find_one(filter, None).await;
        match result {
            Ok(user) => match user {
                Some(user) => {
                    let user: users::User = bson::from_document(user).unwrap();
                    Ok(user)
                }
                None => Err(UserDataSourceError::UsernameNotFound(username.clone())),
            },
            Err(_) => Err(UserDataSourceError::UsernameNotFound(username.clone())),
        }
    }

    async fn create_user(&self, input: users::CreateUserInput) -> Result<(), UserDataSourceError> {
        let collection = self.db.collection("users");
        let user: users::User = users::User {
            user_id: bson::Uuid::new(),
            username: input.username.clone(),
            first_name: input.first_name,
            last_name: input.last_name,
            country: input.country,
            skills: input.skills,
            cv: input.cv,
            primary_email: input.primary_email,
            other_mails: input.other_mails,
            about: input.about,
            avatar: input.avatar,
            cover_photo: input.cover_photo,
            friends_list: input.friends_list,
            education: input.education,
        };
        let result = collection.insert_one(user, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(UserDataSourceError::UsernameTaken(input.username.clone())),
        }
    }

    async fn update_user_info(&self, input: users::UpdateUserInput) -> Result<users::User, UserDataSourceError> {
        let collection: mongodb::Collection<users::User> = self.db.collection("users");
        let filter = bson::doc! {"id": input.username.clone()};
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
                let return_user = self.get_user_by_username(input.username.clone().unwrap()).await;
                match return_user {
                    Ok(user) => Ok(user),
                    Err(_) => Err(UserDataSourceError::UsernameNotFound(input.username.clone().unwrap())),
                }
            }
            Err(_) => Err(UserDataSourceError::UsernameTaken(input.username.clone().unwrap())),
        }
    }

    async fn delete_user(&self, id: bson::Uuid) -> Result<(), UserDataSourceError> {
        let collection: mongodb::Collection<users::User> = self.db.collection("users");
        let filter = bson::doc! {"id": id};
        let result = collection.delete_one(filter, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(UserDataSourceError::UuidNotFound(id)),
        }
    }
    
    async fn update_avatar(&self, _photo_id: bson::Uuid) -> Result<(), UserDataSourceError> {
        unimplemented!()
        // let collection: mongodb::Collection<users::User> = self.db.collection("users");
        // let filter = bson::doc! {"id": photo_id};
        // let update = bson::doc! {"$set": {
        //     "avatar": photo_id,
        // }};
        // let result = collection.update_one(filter, update, None).await;
        // match result {
        //     Ok(_) => Ok(()),
        //     Err(_) => Err(UserDataSourceError::UuidNotFound(photo_id)),
        // }
    }

    async fn update_cover_photo(&self, _photo_id: bson::Uuid) -> Result<(), UserDataSourceError> {
        unimplemented!()
        // let collection: mongodb::Collection<users::User> = self.db.collection("users");
    }

    async fn add_other_email(&self, _email: String) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }

    async fn delete_other_email(&self, _email: String) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }
}
