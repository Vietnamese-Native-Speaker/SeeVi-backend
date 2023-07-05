use crate::{
    data_source::{user_data_source::UserDataSource, user_data_source_error::UserDataSourceError},
    models::{
        education::Education,
        users::{UpdateUserInput, User},
    },
};

use super::ResourceIdentifier;

pub struct UserService;

impl UserService {
    pub async fn get_user_by_id(
        database: &(impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id).await;
        match user {
            Ok(user) => {
                return Ok(user);
            }
            Err(_) => {
                return Err(UserDataSourceError::UuidNotFound(user_id));
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

    /// Receive a user id and a new email as a string
    /// and will change the email of the user with the given id
    /// and return the user with the new email
    pub async fn change_primary_email(
        database: &mut (impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        new_email: String,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id.clone()).await;
        if user.is_err() {
            return Err(UserDataSourceError::UuidNotFound(user_id));
        }
        let new_user = UpdateUserInput::builder()
            .with_user_id(user_id)
            .with_primary_email(new_email)
            .build()
            .unwrap();
        let user = database.update_user_info(new_user).await;
        match user {
            Ok(_) => {
                return user;
            }
            Err(_) => {
                return Err(UserDataSourceError::UpdateUserFailed);
            }
        }
    }

    /// Receive a user id and new other mails as a vector of strings
    /// and update the other mails of the user with the given id
    /// and return the user with the new other mails
    pub async fn change_other_mails(
        database: &mut (impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        new_other_mails: Vec<String>,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id.clone()).await;
        if user.is_err() {
            return Err(UserDataSourceError::UuidNotFound(user_id));
        }
        let new_user = UpdateUserInput::builder()
            .with_user_id(user_id)
            .with_other_mails(new_other_mails)
            .build()
            .unwrap();
        let user = database.update_user_info(new_user).await;
        match user {
            Ok(_) => {
                return user;
            }
            Err(_) => {
                return Err(UserDataSourceError::UpdateUserFailed);
            }
        }
    }

    /// Receive a user id and a new username as a string
    /// and will change the username of the user with the given id
    /// and return the user with the new username
    pub async fn change_username(
        database: &mut (impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        new_username: String,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id.clone()).await;
        if user.is_err() {
            return Err(UserDataSourceError::UuidNotFound(user_id));
        }
        let new_user = UpdateUserInput::builder()
            .with_user_id(user_id)
            .with_username(new_username)
            .build()
            .unwrap();
        let user = database.update_user_info(new_user).await;
        match user {
            Ok(_) => {
                return user;
            }
            Err(_) => {
                return Err(UserDataSourceError::UpdateUserFailed);
            }
        }
    }

    /// Receive a user id and a new first name + last name as a string
    /// and will change the name of the user with the given id
    /// and return the user with the new name
    pub async fn change_name(
        database: &mut (impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        new_first_name: Option<String>,
        new_last_name: Option<String>,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id.clone()).await;
        if user.is_err() {
            return Err(UserDataSourceError::UuidNotFound(user_id));
        }
        let new_user = UpdateUserInput::builder()
            .with_user_id(user_id)
            .with_first_name(new_first_name.unwrap())
            .with_last_name(new_last_name.unwrap())
            .build()
            .unwrap();
        let user = database.update_user_info(new_user).await;
        match user {
            Ok(_) => {
                return user;
            }
            Err(_) => {
                return Err(UserDataSourceError::UpdateUserFailed);
            }
        }
    }

    /// Receive a user id and a new country as a string
    /// and will change the country of the user with the given id
    /// and return the user with the new country
    pub async fn change_country(
        database: &mut (impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        new_country: String,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id.clone()).await;
        if user.is_err() {
            return Err(UserDataSourceError::UuidNotFound(user_id));
        }
        let new_user = UpdateUserInput::builder()
            .with_user_id(user_id)
            .with_country(new_country)
            .build()
            .unwrap();
        let user = database.update_user_info(new_user).await;
        match user {
            Ok(_) => {
                return user;
            }
            Err(_) => {
                return Err(UserDataSourceError::UpdateUserFailed);
            }
        }
    }

    pub async fn change_skills(
        database: &mut (impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        new_skills: Vec<String>,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id.clone()).await;
        if user.is_err() {
            return Err(UserDataSourceError::UuidNotFound(user_id));
        }
        let new_user = UpdateUserInput::builder()
            .with_user_id(user_id)
            .with_skills(new_skills)
            .build()
            .unwrap();
        let user = database.update_user_info(new_user).await;
        match user {
            Ok(_) => {
                return user;
            }
            Err(_) => {
                return Err(UserDataSourceError::UpdateUserFailed);
            }
        }
    }

    /// Receive a user id and a new cv Uuid
    /// and will add the cv to the user with the given id
    /// and return the user with the new cv
    pub async fn add_cv(
        database: &mut (impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        new_cv: ResourceIdentifier,
    ) -> Result<User, UserDataSourceError> {
        todo!()
    }

    /// Receive a user id and a cv Uuid
    /// and will remove the cv from the user with the given id
    /// and return the user without the cv
    pub async fn remove_cv(
        database: &mut (impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        to_remove_cv: ResourceIdentifier,
    ) -> Result<User, UserDataSourceError> {
        todo!()
    }

    /// Receive a user id and a new about as a string
    /// and will change the about of the user with the given id
    /// and return the user with the new about
    pub async fn change_about(
        database: &mut (impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        new_about: String,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id.clone()).await;
        if user.is_err() {
            return Err(UserDataSourceError::UuidNotFound(user_id));
        }
        let new_user = UpdateUserInput::builder()
            .with_user_id(user_id)
            .with_about(new_about)
            .build()
            .unwrap();
        let user = database.update_user_info(new_user).await;
        match user {
            Ok(_) => {
                return user;
            }
            Err(_) => {
                return Err(UserDataSourceError::UpdateUserFailed);
            }
        }
    }

    /// Receive a user id and a new avatar Uuid
    /// and will change the avatar of the user with the given id
    /// and return the user with the new avatar
    pub async fn change_avatar(
        database: &mut (impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        new_avatar: ResourceIdentifier,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id.clone()).await;
        if user.is_err() {
            return Err(UserDataSourceError::UuidNotFound(user_id));
        }
        let new_user = UpdateUserInput::builder()
            .with_user_id(user_id)
            .with_avatar(new_avatar)
            .build()
            .unwrap();
        let user = database.update_user_info(new_user).await;
        match user {
            Ok(_) => {
                return user;
            }
            Err(_) => {
                return Err(UserDataSourceError::UpdateUserFailed);
            }
        }
    }

    /// Receive a user id and a new cover photo Uuid
    /// and will change the cover photo of the user with the given id
    /// and return the user with the new cover photo
    pub async fn change_cover_photo(
        database: &mut (impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        new_cover_photo: ResourceIdentifier,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id.clone()).await;
        if user.is_err() {
            return Err(UserDataSourceError::UuidNotFound(user_id));
        }
        let new_user = UpdateUserInput::builder()
            .with_user_id(user_id)
            .with_cover_photo(new_cover_photo)
            .build()
            .unwrap();
        let user = database.update_user_info(new_user).await;
        match user {
            Ok(_) => {
                return user;
            }
            Err(_) => {
                return Err(UserDataSourceError::UpdateUserFailed);
            }
        }
    }

    /// Receive a user id and a new friend list
    /// and will change the friend list of the user with the given id
    /// and return the user with the new friend list
    pub async fn update_friend_list(
        database: &mut (impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        friend_list: Vec<ResourceIdentifier>,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id.clone()).await;
        if user.is_err() {
            return Err(UserDataSourceError::UuidNotFound(user_id));
        }
        let new_user = UpdateUserInput::builder()
            .with_user_id(user_id)
            .with_friends_list(friend_list)
            .build()
            .unwrap();
        let user = database.update_user_info(new_user).await;
        match user {
            Ok(_) => {
                return user;
            }
            Err(_) => {
                return Err(UserDataSourceError::UpdateUserFailed);
            }
        }
    }

    /// Receive a user id and a new education list
    /// and will change the education list of the user with the given id
    /// and return the user with the new education list
    pub async fn update_education(
        database: &mut (impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        new_education: Vec<Education>,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id.clone()).await;
        if user.is_err() {
            return Err(UserDataSourceError::UuidNotFound(user_id));
        }
        let new_user = UpdateUserInput::builder()
            .with_user_id(user_id)
            .with_education(new_education)
            .build()
            .unwrap();
        let user = database.update_user_info(new_user).await;
        match user {
            Ok(_) => {
                return user;
            }
            Err(_) => {
                return Err(UserDataSourceError::UpdateUserFailed);
            }
        }
    }
}
