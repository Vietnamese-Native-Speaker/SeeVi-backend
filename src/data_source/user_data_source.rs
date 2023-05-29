use mongodb::bson::Uuid;

use crate::models::users::{CreateUserInput, UpdateUserInput, User};

/// Primary abstraction for User Data Source. Ones should implement this trait for
/// different type of database in order to provide that data source to services
trait UserDataSource {
    /// Return the user using the provided `id`
    fn get_user_by_id(&self, id: Uuid) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }

    /// Return the user using the provided `username`, the `username` is
    /// asummed to be unique.
    fn get_user_by_username(&self, username: String) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }

    /// Add new user into the database using the provided input, implementer should check
    /// for uniqueness of the username.
    fn create_user(&self, input: CreateUserInput) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }

    /// Update the user in the database using the provided inputuser in the database using the
    /// provided input.
    fn update_user_info(&self, input: UpdateUserInput) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }

    /// Delete the user in the database with the provided id.
    fn delete_user(&self, id: Uuid) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }

    /// Change the user's avatar to the corresponding photo provided by the uuid.
    /// Note that the job of storing the actual photo is not the responsibility of this trait.
    fn update_avatar(&self, photo_id: Uuid) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }

    /// Change the user's cover photo to the corresponding photo provided by the uuid.
    /// Note that the job of storing the actual photo is not the responsibility of this trait.
    fn update_cover_photo(&self, photo_id: Uuid) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }

    /// Add new email to the `other_email` list.
    fn add_other_email(&self, email: String) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }

    /// Delete a email from the `other_email` list.
    fn delete_other_email(&self, email: String) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }
}

pub enum UserDataSourceError {}

