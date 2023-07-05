use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    data_source::{user_data_source::UserDataSource, user_data_source_error::UserDataSourceError},
    models::{
        education::Education,
        users::{CreateUserInput, UpdateUserInput, User},
    },
};

use super::super::ResourceIdentifier;

pub struct UserService;

/// The struct Claims is used to store
/// the data of the token needed to authenticate services
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// sub is the subject of the token,
    /// here we choose to use the username since it is unique
    pub sub: String,
    /// exp is the expiration time of the token
    /// from 1-1-1970, count in seconds
    pub exp: usize,
    /// aud is the audience of the token,
    /// here we choose to use the website url
    pub aud: String,
}

/// Function will receive a password as a string
/// and return a hashed password as a string for security
pub fn hash_password(s: String) -> String {
    let hash_cost = std::env::var("HASH_COST");
    let hash_cost = match hash_cost {
        Ok(hash_cost) => hash_cost.parse::<u32>().unwrap(),
        Err(_) => bcrypt::DEFAULT_COST,
    };
    let result = bcrypt::hash(s, hash_cost);
    match result {
        Ok(result) => {
            return result;
        }
        Err(_) => return "Failed to hash".to_string(),
    }
}

/// Fetch the secret key from the environment variable
pub fn fetch_secret_key() -> String {
    let secret_key = std::env::var("SECRET_KEY");
    match secret_key {
        Ok(_) => return secret_key.unwrap(),
        Err(_) => panic!("Cannot found secret key"),
    }
}

/// Receive a username as a string and a token as a string
/// and return a boolean value to indicate whether the token is valid
pub fn validate_token(username: String, token: &str) -> bool {
    let binding = fetch_secret_key();
    let secret_key = binding.as_bytes();
    let mut validation = Validation::new(Algorithm::HS256);
    // Set to check the audience of the token
    validation.set_audience(&["www.example.com"]);
    // Set to check the subject of the token
    validation.sub = Some(username);
    let token_data =
        match decode::<Claims>(&token, &DecodingKey::from_secret(secret_key), &validation) {
            Ok(_) => return true,
            Err(_) => return false,
        };
}

impl UserService {
    /// Receive user input as CreateUserInput struct
    /// to register a new user on the database and return the user
    // TODO: add register by email, add checking for invalid characters
    pub async fn register(
        database: &(impl UserDataSource + std::marker::Sync),
        user_input: CreateUserInput,
    ) -> Result<User, UserDataSourceError> {
        /// Function will check whether the input string contains invalid characters
        // TODO: re-consider the invalid characters
        fn check_invalid_characters(s: &str) -> bool {
            if s.len() == 0 {
                return true;
            }
            let invalid_chars = [
                ' ', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '+', '=', '[', ']', '{',
                '}', '|', '\\', '/', '<', '>', '?', ';', ':', '"', '\'', ',', '.',
            ];
            for ch in s.chars() {
                if invalid_chars.contains(&ch) {
                    return true;
                }
            }
            false
        }

        // if check_invalid_characters(user_input.username.as_str()) {
        //     return Err(UserDataSourceError::InvalidUsername(user_input.username));
        // }
        // if check_invalid_characters(user_input.password.as_str()) {
        //     return Err(UserDataSourceError::InvalidPassword);
        // }
        // if check_invalid_characters(user_input.primary_email.as_str()) {
        //     return Err(UserDataSourceError::InvalidEmail(user_input.primary_email));
        // }
        // if check_invalid_characters(user_input.first_name.as_str()) {
        //     return Err(UserDataSourceError::InvalidNameField(user_input.first_name));
        // }
        // if check_invalid_characters(user_input.last_name.as_str()) {
        //     return Err(UserDataSourceError::InvalidNameField(user_input.last_name));
        // }

        let username = user_input.username.clone();
        if database.get_user_by_username(&username).await.is_ok() {
            return Err(UserDataSourceError::UsernameTaken(username));
        }
        // let email = user_input.primary_email.clone();
        // if database.get_user_by_email(&email).await.is_ok() {
        //     return Err(UserDataSourceError::EmailTaken(email))
        // }
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

    /// Authenticate a user
    /// Return a token as a string if the authentication is successful
    /// otherwise return an error
    pub async fn authenticate(
        database: &(impl UserDataSource + std::marker::Sync),
        username: Option<String>,
        email: Option<String>,
        password: String,
    ) -> Result<String, UserDataSourceError> {
        if username.is_none() && email.is_none() {
            return Err(UserDataSourceError::EmptyUsername);
        }
        if let Some(username) = username.clone() {
            let user = database.get_user_by_username(&username).await;
            if user.is_err() {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
            let user = user.unwrap();
            if let Err(_) = bcrypt::verify(password, user.password.as_str()) {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
            let header = Header::new(Algorithm::HS256);
            let claims = Claims {
                sub: user.username.to_owned(),
                // TODO: change the expiration time time request + amount of time and add audience address
                exp: 10000000000,
                aud: "www.example.com".to_string(),
            };
            let binding = fetch_secret_key();
            let secret_key = binding.as_bytes();
            if let Ok(token) = encode(
                &header,
                &claims,
                &EncodingKey::from_secret(secret_key.as_ref()),
            ) {
                return Ok(token);
            } else {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
        } else if let Some(email) = email.clone() {
            let user = database.get_user_by_email(&email).await;
            if user.is_err() {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
            let user = user.unwrap();
            if let Err(_) = bcrypt::verify(password, user.password.as_str()) {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
            let header = Header::new(Algorithm::HS256);
            let claims = Claims {
                sub: user.username.to_owned(),
                // TODO: change the expiration time to time request + amount of time and add audience address
                exp: 10000000000,
                aud: "www.example.com".to_string(),
            };
            let binding = fetch_secret_key();
            let secret_key = binding.as_bytes();
            if let Ok(token) = encode(
                &header,
                &claims,
                &EncodingKey::from_secret(secret_key.as_ref()),
            ) {
                return Ok(token);
            } else {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
        }
        return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
    }

    /// Change the password of the user with the given id
    /// and return the user with the new password
    pub async fn change_password(
        database: &mut (impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        new_password: String,
    ) -> Result<User, UserDataSourceError> {
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
