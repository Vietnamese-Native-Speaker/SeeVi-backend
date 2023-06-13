use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

pub struct UserService;
/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    password: String, 
}

impl UserService {
    pub async fn register(database: &mut impl UserDataSource, user_input: CreateUserInput) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        fn check_invalid_characters(s: &str) -> bool {
            if s.len() == 0 {
                return true;
            }
            let invalid_chars = [
                ' ', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '+', '=', '[', ']', '{', '}', '|', '\\',
                '/', '<', '>', '?', ';', ':', '"', '\'', ',', '.'
            ];
            for ch in username.chars() {
                if invalid_chars.contains(&ch) {
                    return true;
                }
            }
            false
        }

        if(check_invalid_characters(user_input.username.as_str())) {
            return Err(Box::new(UserError::InvalidUsername(user_input.username)))
        }
        if(check_invalid_characters(user_input.email.as_str())) {
            return Err(Box::new(UserError::InvalidEmail(user_input.email)))
        }
        if(check_invalid_characters(user_input.first_name.as_str())) {
            return Err(Box::new(UserError::InvalidNameField(user_input.first_name)))
        }
        if(check_invalid_characters(user_input.last_name.as_str())) {
            return Err(Box::new(UserError::InvalidNameField(user_input.last_name)))
        }

        let username = user_input.username;
        if(database.find_user_by_username(username.clone()).await.is_ok()) {
            return Err(Box::new(UserError::UsernameTaken(username)))
        }
        let email = user_input.email;
        if(database.find_user_by_email(email.clone()).await.is_ok()) {
            return Err(Box::new(UserError::EmailTaken(email)))
        }
        if(database.create_user(user_input).await.is_ok()) {
            return Ok(Box::new("User created successfully"));
        }
        return Err(Box::new("User creation failed"));
    }
    //Function will return a token as a string that can be used for authentication
    pub async fn authenticate(database: &mut impl UserDataSource, username: Option<String>, email: Option<String>, password: String) -> Result<String, dyn Box<impl Debug + Clone + Copy>> {
        if (username.is_none() || email.is_none()) {
            return Err(Box::new("Username or email must be provided"));
        }
        if (username.is_some()) {
            let username = username.unwrap();
            let user = database.find_user_by_username(username.clone()).await;
            if (user.is_err()) {
                return Err(Box::new("Wrong username or password"));
            }
            let user = user.unwrap();
            if (user.password != password) {
                return Err(Box::new("Wrong username or password"));
            }
            let header = Header::new(Algorithm::HS256);
            let claims = Claims {
                username: user.username.to_owned(),
                password: user.password.tơ_owned(),
            };
            let secret_key = "secret";
            let token = (&header, &claims, &EncodingKey::from_secret(secret_key.as_ref())).encode();
            return Ok(token);
        }
        if(email.is_some()) {
            let email = email.unwrap();
            let user = database.find_user_by_email(email.clone()).await;
            if (user.is_err()) {
                return Err(Box::new("Wrong email or password"));
            }
            let user = user.unwrap();
            if (user.password != password) {
                return Err(Box::new("Wrong email or password"));
            }
            return Ok(user);
        }
        return Err(Box::new("Wrong username or password"));
    }
    //Forget password = change password
    pub async fn change_password(database: &mut impl UserDataSource, user_id: Uuid, new_password: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_email(database: &mut impl UserDataSource, user_id: Uuid, new_email: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_username(database: &mut impl UserDataSource, user_id: Uuid, new_username: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_name(database: &mut impl UserDataSource, user_id: Uuid, new_first_name: String, new_last_name: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_country(database: &mut impl UserDataSource, user_id: Uuid, new_country: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_skills(database: &mut impl UserDataSource, user_id: Uuid, new_skill: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_skills(database: &mut impl UserDataSource, user_id: Uuid, to_remove_skill: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_cv(database: &mut impl UserDataSource, user_id: Uuid, new_cv: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_cv(database: &mut impl UserDataSource, user_id: Uuid, to_remove_cv:Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_email(database: &mut impl UserDataSource, user_id: Uuid, new_email: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_email(database: &mut impl UserDataSource, user_id: Uuid, to_remove_email: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_about(database: &mut impl UserDataSource, user_id: Uuid, new_about: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_avatar(database: &mut impl UserDataSource, user_id: Uuid, new_avatar: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_cover_photo(database: &mut impl UserDataSource, user_id: Uuid, new_cover_photo: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_friend(database: &mut impl UserDataSource, user_id: Uuid, new_friend: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_friend(database: &mut impl UserDataSource, user_id: Uuid, to_remove_friend: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_education(database: &mut impl UserDataSource, user_id: Uuid, new_education: Education) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_education(database: &mut impl UserDataSource, user_id: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn find_user_by_username(database: &mut impl UserDataSource, _username: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn find_user_by_email(database: &mut impl UserDataSource, _email: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn find_user_by_id(database: &mut impl UserDataSource, user_id: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn get_friends_of_user(database: &mut impl UserDataSource, user_id: Uuid) -> Result<Vec<User>, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
}