use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

use crate::{models::{users::{CreateUserInput, User, UpdateUserInput}, education::Education}, data_source::{user_data_source::UserDataSource, user_data_source_error::UserDataSourceError}};

use super::ResourceIdentifier;

pub struct UserService;
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    password: String, 
}

pub fn hash_password(s: String) -> String {
    let result = bcrypt::hash(s, bcrypt::DEFAULT_COST);
    match result {
        Ok(result) => {
            return result;
        }
        Err(_) => return "Failed to hash".to_string(),
    }
}   

impl UserService {
    pub async fn register(database: &mut (impl UserDataSource + std::marker::Sync), user_input: CreateUserInput) -> Result<User, UserDataSourceError> {
        fn check_invalid_characters(s: &str) -> bool {
            if s.len() == 0 {
                return true;
            }
            let invalid_chars = [
                ' ', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '+', '=', '[', ']', '{', '}', '|', '\\',
                '/', '<', '>', '?', ';', ':', '"', '\'', ',', '.'
            ];
            for ch in s.chars() {
                if invalid_chars.contains(&ch) {
                    return true;
                }
            }
            false
        }

        if check_invalid_characters(user_input.username.as_str()) {
            return Err(UserDataSourceError::InvalidUsername(user_input.username))
        }
        if check_invalid_characters(user_input.password.as_str()) {
            return Err(UserDataSourceError::InvalidPassword)
        }
        if check_invalid_characters(user_input.primary_email.as_str()) {
            return Err(UserDataSourceError::InvalidEmail(user_input.primary_email))
        }
        if check_invalid_characters(user_input.first_name.as_str())  {
            return Err(UserDataSourceError::InvalidNameField(user_input.first_name))
        }
        if check_invalid_characters(user_input.last_name.as_str()) {
            return Err(UserDataSourceError::InvalidNameField(user_input.last_name))
        }

        let username = user_input.username.clone();
        if database.get_user_by_username(username.clone()).await.is_ok() {
            return Err(UserDataSourceError::UsernameTaken(username))
        }
        let email = user_input.primary_email.clone();
        if database.get_user_by_email(email.clone()).await.is_ok() {
            return Err(UserDataSourceError::EmailTaken(email))
        }
        let hash = hash_password(user_input.password);
        let user_input = CreateUserInput {
            password: hash,
            ..user_input
        };
        let user = database.create_user(user_input).await;
        match user {
            Ok(_) => {
                return user;
            }
            Err(_) => {
                return Err(UserDataSourceError::CreateUserFailed);
            }
        }
    }
    //Function will return a token as a string that can be used for authentication
    pub async fn authenticate(database: &mut (impl UserDataSource + std::marker::Sync), username: Option<String>, email: Option<String>, password: String) -> Result<String, UserDataSourceError> { 
        if username.is_none() && email.is_none() {
            return Err(UserDataSourceError::EmptyUsername);
        }
        if let Some(username) = username.clone() {
            let user = database.get_user_by_username(username.clone()).await;
            if user.is_err() {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
            let user = user.unwrap();
            if let Err(_) = bcrypt::verify(password, user.password.as_str()) {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
            let header = Header::new(Algorithm::HS256);
            let claims = Claims {
                username: user.username.to_owned(),
                password: user.password.to_owned(),
            };
            let secret_key = "secret";
            if let Ok(token) = encode(&header, &claims, &EncodingKey::from_secret(secret_key.as_ref())) {
                return Ok(token);
            }
            else {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
        }
        else if let Some(email) = email.clone() {
            let user = database.get_user_by_email(email.clone()).await;
            if user.is_err() {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
            let user = user.unwrap();
            if let Err(_) = bcrypt::verify(password, user.password.as_str()) {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
            let header = Header::new(Algorithm::HS256);
            let claims = Claims {
                username: user.username.to_owned(),
                password: user.password.to_owned(),
            };
            let secret_key = "secret"; 
            if let Ok(token) = encode(&header, &claims, &EncodingKey::from_secret(secret_key.as_ref())) {
                return Ok(token);
            }
            else {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
        }
        return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
    }
    //Forget password = change password
    pub async fn change_password(database: &mut (impl UserDataSource + std::marker::Sync), user_id: ResourceIdentifier, new_password: String) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id.clone()).await;
        if user.is_err() {
            return Err(UserDataSourceError::UuidNotFound(user_id));
        }
        let new_hashed_password = hash_password(new_password);
        let new_user = UpdateUserInput::builder()
            .with_user_id(user_id)
            .with_password(new_hashed_password)
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
    pub async fn change_primary_email(database: &mut (impl UserDataSource + std::marker::Sync), user_id: ResourceIdentifier, new_email: String) -> Result<User, UserDataSourceError> {
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
    pub async fn change_other_mails(database: &mut (impl UserDataSource + std::marker::Sync), user_id: ResourceIdentifier, new_other_mails: Vec<String>) -> Result<User, UserDataSourceError> {
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
    pub async fn change_username(database: &mut (impl UserDataSource + std::marker::Sync), user_id: ResourceIdentifier, new_username: String) -> Result<User, UserDataSourceError> {
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
    pub async fn change_name(database: &mut (impl UserDataSource + std::marker::Sync), user_id: ResourceIdentifier, new_first_name: Option<String>, new_last_name: Option<String>) -> Result<User, UserDataSourceError> {
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
    pub async fn change_country(database: &mut (impl UserDataSource + std::marker::Sync), user_id: ResourceIdentifier, new_country: String) -> Result<User, UserDataSourceError> {
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
    pub async fn change_skills(database: &mut (impl UserDataSource + std::marker::Sync), user_id: ResourceIdentifier, new_skills: Vec<String>) -> Result<User, UserDataSourceError> {
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
    pub async fn add_cv(database: &mut (impl UserDataSource + std::marker::Sync), user_id: ResourceIdentifier, new_cv: ResourceIdentifier) -> Result<User, UserDataSourceError> {
        todo!()
    }
    pub async fn remove_cv(database: &mut (impl UserDataSource + std::marker::Sync), user_id: ResourceIdentifier, to_remove_cv:ResourceIdentifier) -> Result<User, UserDataSourceError> {
        todo!()
    }
    pub async fn change_about(database: &mut (impl UserDataSource + std::marker::Sync), user_id: ResourceIdentifier, new_about: String) -> Result<User, UserDataSourceError> {
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
    pub async fn change_avatar(database: &mut (impl UserDataSource + std::marker::Sync), user_id: ResourceIdentifier, new_avatar: ResourceIdentifier) -> Result<User, UserDataSourceError> {
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
    pub async fn change_cover_photo(database: &mut (impl UserDataSource + std::marker::Sync), user_id: ResourceIdentifier, new_cover_photo: ResourceIdentifier) -> Result<User, UserDataSourceError> {
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
    pub async fn update_friend_list(database: &mut (impl UserDataSource + std::marker::Sync), user_id: ResourceIdentifier, friend_list: Vec<ResourceIdentifier>) -> Result<User, UserDataSourceError> {
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
    pub async fn update_education(database: &mut (impl UserDataSource + std::marker::Sync), user_id: ResourceIdentifier, new_education: Vec<Education>) -> Result<User, UserDataSourceError> {
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