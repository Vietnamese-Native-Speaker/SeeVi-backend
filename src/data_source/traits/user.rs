use crate::data_source::DataSourceError;
use async_graphql::futures_util::stream::BoxStream;
use async_trait::async_trait;
use mongodb::bson;

use crate::models::users::{CreateUserInput, UpdateUserInput, User};

/// Primary abstraction for User Data Source. Ones should implement this trait for
/// different type of database in order to provide that data source to services
#[async_trait]
pub trait UserDataSource {
    /// Return the user using the provided `id`
    async fn get_user_by_id(&self, _id: bson::oid::ObjectId) -> Result<User, DataSourceError>;

    /// Return the user using the provided `username`, the `username` is
    /// asummed to be unique.
    async fn get_user_by_username(&self, _username: &str) -> Result<User, DataSourceError>;

    /// Return the user using the provided primary `email`, the `email` is
    /// asummed to be unique.
    async fn get_user_by_email(&self, _email: &str) -> Result<User, DataSourceError>;

    /// Create new user in the database using the provided input, implementer should check
    /// for uniqueness of the username.
    async fn create_user(&self, _input: CreateUserInput) -> Result<User, DataSourceError>;

    /// Update the user in the database using the provided input user in the database using the
    /// provided input.
    async fn update_user_info(&self, _input: UpdateUserInput) -> Result<User, DataSourceError>;

    /// Delete the user in the database with the provided id.
    async fn delete_user(&self, _id: bson::oid::ObjectId) -> Result<User, DataSourceError>;

    async fn get_users_by_ids(
        &self,
        _ids: BoxStream<'async_trait, bson::oid::ObjectId>,
    ) -> BoxStream<Result<User, DataSourceError>>;
}
