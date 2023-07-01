use crate::data_source::user_data_source::UserDataSource;
use crate::data_source::user_data_source_error::UserDataSourceError;
use crate::models::education::Education;
use crate::models::users::create_user_input::CreateUserInputBuilder;
use crate::models::users::user::Level;
use crate::models::users::{self, CreateUserInput, UpdateUserInput, User};
use crate::services::user_service::Claims;
use crate::services::user_service::UserService;
use async_trait::async_trait;
use mongodb::bson::Uuid;
use std::cell::Cell;
use std::sync::Mutex;
struct MockDatabase {
    users: Mutex<Vec<User>>,
}

#[async_trait]
impl UserDataSource for MockDatabase {
    async fn get_user_by_username(&self, username: &str) -> Result<User, UserDataSourceError> {
        let users = self.users.lock().unwrap();
        for user in users.iter() {
            if user.username == username {
                return Ok(user.clone());
            }
        }
        Err(UserDataSourceError::UsernameNotFound(username.to_string()))
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User, UserDataSourceError> {
        let users = self.users.lock().unwrap();
        for user in users.iter() {
            if user.primary_email == email {
                return Ok(user.clone());
            }
        }
        Err(UserDataSourceError::EmailNotFound(email.to_string()))
    }
    async fn get_user_by_id(&self, id: Uuid) -> Result<User, UserDataSourceError> {
        let mut users = self.users.lock().unwrap();
        for user in users.iter() {
            if user.user_id == id {
                return Ok(user.clone());
            }
        }
        Err(UserDataSourceError::UuidNotFound(id.clone()))
    }
    async fn update_user_info(
        &self,
        updated_user: UpdateUserInput,
    ) -> Result<User, UserDataSourceError> {
        let mut users = self.users.lock().unwrap();
        for user in users.iter_mut() {
            if user.user_id == updated_user.user_id {
                user.username = updated_user
                    .username
                    .clone()
                    .unwrap_or(user.username.clone());
                user.password = updated_user
                    .password
                    .clone()
                    .unwrap_or(user.password.clone());
                user.first_name = updated_user
                    .first_name
                    .clone()
                    .unwrap_or(user.first_name.clone());
                user.last_name = updated_user
                    .last_name
                    .clone()
                    .unwrap_or(user.last_name.clone());
                user.country = updated_user.country.clone().or(user.country.clone());
                user.skills = updated_user.skills.clone().unwrap_or(user.skills.clone());
                user.primary_email = updated_user
                    .primary_email
                    .clone()
                    .unwrap_or(user.primary_email.clone());
                user.other_mails = updated_user
                    .other_mails
                    .clone()
                    .unwrap_or(user.other_mails.clone());
                user.about = updated_user.about.clone().or(user.about.clone());
                user.avatar = updated_user.avatar.clone().or(user.avatar.clone());
                user.cover_photo = updated_user
                    .cover_photo
                    .clone()
                    .or(user.cover_photo.clone());
                user.friends_list = updated_user
                    .friends_list
                    .clone()
                    .unwrap_or(user.friends_list.clone());
                user.education = updated_user
                    .education
                    .clone()
                    .unwrap_or(user.education.clone());
                return Ok(user.clone());
            }
        }
        return Err(UserDataSourceError::UuidNotFound(
            updated_user.user_id.clone(),
        ));
    }

    async fn create_user(&self, _input: CreateUserInput) -> Result<User, UserDataSourceError> {
        let mut users = self.users.lock().unwrap();
        let user = User {
            user_id: Uuid::new(),
            username: _input.username,
            password: _input.password,
            first_name: _input.first_name,
            last_name: _input.last_name,
            country: _input.country,
            skills: _input.skills,
            primary_email: _input.primary_email,
            other_mails: _input.other_mails,
            about: _input.about,
            avatar: _input.avatar,
            cover_photo: _input.cover_photo,
            friends_list: Vec::new(),
            education: _input.education,
            cv: vec![Uuid::new()],
            rating: _input.rating,
            level: Some(Level::Fresher),
            shared_cvs: vec![Uuid::new()],
            saved_cvs: vec![Uuid::new()],
            liked_cvs: vec![Uuid::new()],
        };
        users.push(user.clone());
        Ok(user)
    }
}

#[tokio::test]
async fn register_user_test() {
    dotenv::dotenv().ok();
    let mut db = MockDatabase {
        users: Mutex::new(Vec::new()),
    };
    let uuid = Uuid::new();
    let user = CreateUserInputBuilder::default()
        .with_username("test_user")
        .with_password("test_password")
        .with_first_name("test_first_name")
        .with_last_name("test_last_name")
        .with_country("test_country")
        .with_skill("test_skill_1")
        .with_skill("test_skill_2")
        .with_primary_email("test_primary_email")
        .with_other_mail("test_mail1")
        .with_other_mail("test_mail2")
        .with_about("test_about")
        .with_avatar(uuid.clone())
        .with_cover_photo(uuid.clone())
        .with_education(Education {
            institution: "University of Example 1".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        })
        .build()
        .unwrap();
    let user2 = UserService::register(&mut db, user).await.unwrap();
    assert_eq!(user2.username, "test_user");
    assert_eq!(
        bcrypt::verify("test_password", &user2.password).unwrap(),
        true
    );
    assert_eq!(user2.first_name, "test_first_name");
    assert_eq!(user2.last_name, "test_last_name");
    assert_eq!(user2.country, Some("test_country".to_string()));
    assert_eq!(
        user2.skills,
        vec!["test_skill_1".to_string(), "test_skill_2".to_string()]
    );
    assert_eq!(user2.primary_email, "test_primary_email");
    assert_eq!(
        user2.other_mails,
        vec!["test_mail1".to_string(), "test_mail2".to_string()]
    );
    assert_eq!(user2.about, Some("test_about".to_string()));
    assert_eq!(user2.avatar, Some(uuid.clone()));
    assert_eq!(user2.cover_photo, Some(uuid.clone()));
    assert_eq!(
        user2.education,
        vec![Education {
            institution: "University of Example 1".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        }]
    );
}

#[tokio::test]
async fn authenticate_user_test() {
    dotenv::dotenv().ok();
    let mut db = MockDatabase {
        users: Mutex::new(Vec::new()),
    };
    let uuid = Uuid::new();
    let user = CreateUserInputBuilder::default()
        .with_username("test_user")
        .with_password("test_password")
        .with_first_name("test_first_name")
        .with_last_name("test_last_name")
        .with_country("test_country")
        .with_skill("test_skill_1")
        .with_skill("test_skill_2")
        .with_primary_email("test_primary_email")
        .with_other_mail("test_mail1")
        .with_other_mail("test_mail2")
        .with_about("test_about")
        .with_avatar(uuid.clone())
        .with_cover_photo(uuid.clone())
        .with_education(Education {
            institution: "University of Example 1".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        })
        .build()
        .unwrap();
    let user2 = UserService::register(&mut db, user).await.unwrap();
    let key = b"secret";
    let token = UserService::authenticate(
        &mut db,
        Some(user2.username),
        None,
        "test_password".to_string(),
    )
    .await
    .unwrap();
    let token_data = jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(key.as_ref()),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .unwrap();
    assert_eq!(token_data.claims.sub, "test_user");
}
