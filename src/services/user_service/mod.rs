#[cfg(test)]
mod tests;
use mongodb::bson::oid::ObjectId;

use crate::{
    data_source::{user_data_source::UserDataSource, user_data_source_error::UserDataSourceError},
    models::users::{CreateUserInput, UpdateUserInput, User},
};

pub struct UserService;

impl UserService {
    pub async fn get_user_by_id(
        database: &(impl UserDataSource + std::marker::Sync),
        user_id: ObjectId,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id).await;
        match user {
            Ok(user) => {
                return Ok(user);
            }
            Err(_) => {
                return Err(UserDataSourceError::IdNotFound(user_id));
            }
        }
    }

    pub async fn update_user(
        database: &(impl UserDataSource + std::marker::Sync),
        update_user_input: UpdateUserInput,
    ) -> Result<User, UserDataSourceError> {
        let user = database.update_user_info(update_user_input).await;
        match user {
            Ok(user) => {
                return Ok(user);
            }
            Err(_) => {
                return Err(UserDataSourceError::UpdateUserFailed);
            }
        }
    }

    pub async fn create_user(
        database: &(impl UserDataSource + std::marker::Sync),
        user: CreateUserInput,
    ) -> Result<User, UserDataSourceError> {
        let user = database.create_user(user).await;
        match user {
            Ok(user) => {
                return Ok(user);
            }
            Err(_) => {
                return Err(UserDataSourceError::CreateUserFailed);
            }
        }
    }

    pub async fn get_user_by_username(
        database: &(impl UserDataSource + std::marker::Sync),
        username: String,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_username(&username).await;
        match user {
            Ok(user) => {
                return Ok(user);
            }
            Err(_) => {
                return Err(UserDataSourceError::UsernameNotFound(username));
            }
        }
    }
}
