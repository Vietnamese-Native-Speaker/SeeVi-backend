use super::user_data_source_error::UserDataSourceError;
use async_trait::async_trait;
use mongodb::bson::Uuid;

use crate::models::users::{CreateUserInput, UpdateUserInput, User};

/// Primary abstraction for User Data Source. Ones should implement this trait for
/// different type of database in order to provide that data source to services
#[async_trait]
pub trait UserDataSource {
    /// Return the user using the provided `id`
    async fn get_user_by_id(&self, _id: Uuid) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }

    /// Return the user using the provided `username`, the `username` is
    /// asummed to be unique.
    async fn get_user_by_username(&self, _username: &str) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }

    /// Create new user in the database using the provided input, implementer should check
    /// for uniqueness of the username.
    async fn create_user(&self, _input: CreateUserInput) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }

    /// Add new user into the database, implementer should check for the uniqueness
    /// of the username
    async fn add_user(&self, _user: User) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }

    /// Update the user in the database using the provided input user in the database using the
    /// provided input.
    async fn update_user_info(&self, _input: UpdateUserInput) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }

    /// Delete the user in the database with the provided id.
    async fn delete_user(&self, _id: Uuid) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }

    /// Change the user's avatar to the corresponding photo provided by the uuid.
    /// Note that the job of storing the actual photo is not the responsibility of this trait.
    async fn update_avatar(&self, _photo_id: Uuid) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }

    /// Change the user's cover photo to the corresponding photo provided by the uuid.
    /// Note that the job of storing the actual photo is not the responsibility of this trait.
    async fn update_cover_photo(&self, _photo_id: Uuid) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }

    /// Add new email to the `other_email` list.
    async fn add_other_email(&self, _email: String) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }

    /// Delete a email from the `other_email` list.
    async fn delete_other_email(&self, _email: String) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }
}
