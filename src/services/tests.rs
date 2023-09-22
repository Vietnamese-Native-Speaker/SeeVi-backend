use crate::data_source::user_data_source::UserDataSource;
use crate::data_source::user_data_source_error::UserDataSourceError;
use crate::models::users::{CreateUserInput, UpdateUserInput, User};
use async_trait::async_trait;
use mongodb::bson;
use std::sync::Mutex;

pub struct MockDatabase {
    users: Mutex<Vec<User>>,
}

impl MockDatabase {
    pub fn new() -> Self {
        MockDatabase {
            users: Mutex::new(Vec::new()),
        }
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
}
