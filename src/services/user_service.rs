use mongodb::bson::Uuid;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

use crate::{models::{users::{CreateUserInput, User}, education::Education}, data_source::{user_data_source::UserDataSource, user_data_source_error::UserDataSourceError}};

pub struct UserService;
/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    password: String, 
}

pub fn hashPassword(s: String) -> String {
    let result = bcrypt::hash(s, bcrypt::DEFAULT_COST);
    match result {
        Ok(result) => {
            return result;
        }
        Err(_) => return ("Failed to hash".to_string()),
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

        let username = user_input.username;
        if database.get_user_by_username(username.clone()).await.is_ok() {
            return Err(UserDataSourceError::UsernameTaken(username))
        }
        let email = user_input.primary_email;
        if database.get_user_by_email(email.clone()).await.is_ok() {
            return Err(UserDataSourceError::EmailTaken(email))
        }
        let hash = hashPassword(user_input.password);
        let user_input = CreateUserInput {
            username: user_input.username,
            password: hash,
            first_name: user_input.first_name,
            last_name: user_input.last_name,
            country: user_input.country,
            skills: user_input.skills,
            primary_email: user_input.primary_email,
            other_mails: user_input.other_mails,
            about: user_input.about,
            avatar: user_input.avatar,
            cover_photo: user_input.cover_photo,
            education: user_input.education,
            rating: user_input.rating,
            level: user_input.level
        };
        match database.create_user(user_input).await {
            Ok(user) => return Ok(user),
            Err(err) => return Err(err),
        }
    }
    //Function will return a token as a string that can be used for authentication
    pub async fn authenticate(database: &mut (impl UserDataSource + std::marker::Sync), username: Option<String>, email: Option<String>, password: String) -> Result<String, UserDataSourceError> {
        if username.is_none() && email.is_none() {
            return Err(UserDataSourceError::EmptyUsername);
        }
        if username.is_some() {
            let username = username.unwrap();
            let user = database.get_user_by_username(username.clone()).await;
            if user.is_err() {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
            let user = user.unwrap();
            if user.password != password {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
            let header = Header::new(Algorithm::HS256);
            let claims = Claims {
                username: user.username.to_owned(),
                password: user.password.to_owned(),
            };
            let secret_key = "secret";
            let token = encode(&header, &claims, &EncodingKey::from_secret(secret_key.as_ref()));
            match token {
                Ok(token) => return Ok(token),
                Err(_) => return Err(UserDataSourceError::WrongEmailUsernameOrPassword),
            }
        }
        else if email.is_some() {
            let email = email.unwrap();
            let user = database.get_user_by_email(email.clone()).await;
            if user.is_err() {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
            let user = user.unwrap();
            if user.password != password {
                return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
            }
            let header = Header::new(Algorithm::HS256);
            let claims = Claims {
                username: user.username.to_owned(),
                password: user.password.to_owned(),
            };
            let secret_key = "secret";
            let token = encode(&header, &claims, &EncodingKey::from_secret(secret_key.as_ref()));
            match token {
                Ok(token) => return Ok(token),
                Err(_) => return Err(UserDataSourceError::WrongEmailUsernameOrPassword),
            }
        }
        return Err(UserDataSourceError::WrongEmailUsernameOrPassword);
    }
    //Forget password = change password
    pub async fn change_password(database: &mut impl UserDataSource, user_id: Uuid, new_password: String) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn change_email(database: &mut impl UserDataSource, user_id: Uuid, new_email: String) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn change_username(database: &mut impl UserDataSource, user_id: Uuid, new_username: String) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn change_name(database: &mut impl UserDataSource, user_id: Uuid, new_first_name: String, new_last_name: String) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn change_country(database: &mut impl UserDataSource, user_id: Uuid, new_country: String) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn add_skills(database: &mut impl UserDataSource, user_id: Uuid, new_skill: String) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn remove_skills(database: &mut impl UserDataSource, user_id: Uuid, to_remove_skill: String) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn add_cv(database: &mut impl UserDataSource, user_id: Uuid, new_cv: Uuid) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn remove_cv(database: &mut impl UserDataSource, user_id: Uuid, to_remove_cv:Uuid) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn add_email(database: &mut impl UserDataSource, user_id: Uuid, new_email: String) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn remove_email(database: &mut impl UserDataSource, user_id: Uuid, to_remove_email: String) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn change_about(database: &mut impl UserDataSource, user_id: Uuid, new_about: String) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn change_avatar(database: &mut impl UserDataSource, user_id: Uuid, new_avatar: Uuid) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn change_cover_photo(database: &mut impl UserDataSource, user_id: Uuid, new_cover_photo: Uuid) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn add_friend(database: &mut impl UserDataSource, user_id: Uuid, new_friend: Uuid) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn remove_friend(database: &mut impl UserDataSource, user_id: Uuid, to_remove_friend: Uuid) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn add_education(database: &mut impl UserDataSource, user_id: Uuid, new_education: Education) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn remove_education(database: &mut impl UserDataSource, user_id: Uuid) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn find_user_by_username(database: &mut impl UserDataSource, _username: String) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn find_user_by_email(database: &mut impl UserDataSource, _email: String) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn find_user_by_id(database: &mut impl UserDataSource, user_id: Uuid) -> Result<User, UserDataSourceError> {
        unimplemented!()
    }
    pub async fn get_friends_of_user(database: &mut impl UserDataSource, user_id: Uuid) -> Result<Vec<User>, UserDataSourceError> {
        unimplemented!()
    }
}