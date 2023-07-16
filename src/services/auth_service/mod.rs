use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    data_source::{user_data_source::UserDataSource, user_data_source_error::UserDataSourceError},
    models::users::{CreateUserInput, UpdateUserInput, User},
};

use super::ResourceIdentifier;

#[cfg(test)]
mod tests;

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

pub struct AuthService;

impl AuthService {
    /// Function will receive a password as a string
    /// and return a hashed password as a string for security
    pub fn hash_password(s: String) -> String {
        let hash_cost = std::env::var("HASH_COST");
        let hash_cost = match hash_cost {
            Ok(hash_cost) => hash_cost.parse::<u32>().unwrap(),
            Err(_) => bcrypt::DEFAULT_COST,
        };
        let result = bcrypt::hash(s, hash_cost).expect("Hashing failed");
        result
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
        let binding = AuthService::fetch_secret_key();
        let secret_key = binding.as_bytes();
        let mut validation = Validation::new(Algorithm::HS256);
        // Set to check the audience of the token
        validation.set_audience(&["www.example.com"]);
        // Set to check the subject of the token
        validation.sub = Some(username);
        let token_data = match jsonwebtoken::decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret_key),
            &validation,
        ) {
            Ok(_) => return true,
            Err(_) => return false,
        };
    }

    pub fn decode_token(token: &str) -> Option<Claims> {
        let binding = AuthService::fetch_secret_key();
        let secret_key = binding.as_bytes();
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["www.example.com"]);
        match jsonwebtoken::decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret_key),
            &validation,
        ) {
            Ok(token) => Some(token.claims),
            Err(_) => None,
        }
    }

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
        let hash = AuthService::hash_password(user_input.password);
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
            let correct =
                bcrypt::verify(password, &user.password).expect("Error verifying password");
            if !correct {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
            let header = jsonwebtoken::Header::new(Algorithm::HS256);
            let claims = Claims {
                sub: user.username.to_owned(),
                // TODO: change the expiration time time request + amount of time and add audience address
                exp: 10000000000,
                aud: "www.example.com".to_string(),
            };
            let binding = AuthService::fetch_secret_key();
            let secret_key = binding.as_bytes();
            if let Ok(token) = jsonwebtoken::encode(
                &header,
                &claims,
                &jsonwebtoken::EncodingKey::from_secret(secret_key.as_ref()),
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
            let header = jsonwebtoken::Header::new(Algorithm::HS256);
            let claims = Claims {
                sub: user.username.to_owned(),
                // TODO: change the expiration time to time request + amount of time and add audience address
                exp: 10000000000,
                aud: "www.example.com".to_string(),
            };
            let binding = AuthService::fetch_secret_key();
            let secret_key = binding.as_bytes();
            if let Ok(token) = jsonwebtoken::encode(
                &header,
                &claims,
                &jsonwebtoken::EncodingKey::from_secret(secret_key.as_ref()),
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
        database: &(impl UserDataSource + std::marker::Sync),
        user_id: ResourceIdentifier,
        new_password: String,
    ) -> Result<User, UserDataSourceError> {
        let user = database.get_user_by_id(user_id.clone()).await;
        if user.is_err() {
            return Err(UserDataSourceError::UuidNotFound(user_id));
        }
        let new_hashed_password = AuthService::hash_password(new_password);
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
}
