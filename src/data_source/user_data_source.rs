use mongodb::bson::Uuid;

use crate::models::users::{User, CreateUserInput, UpdateUserInput};

trait UserDataSource {
    fn get_user_by_id(&self, id: Uuid) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    fn get_user_by_username(&self, username: String) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    fn create_user(&self, input: CreateUserInput) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }
    fn update_user_info(&self, input: UpdateUserInput) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    fn delete_user(&self, id: Uuid) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }
    fn update_avatar(&self, photo_id: Uuid) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }
    fn update_cover_photo(&self, photo_id: Uuid) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }
    fn add_other_email(&self, email: String) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }
    fn delete_other_email(&self, email: String) -> Result<(), UserDataSourceError> {
        unimplemented!()
    }
}

pub enum UserDataSourceError {}