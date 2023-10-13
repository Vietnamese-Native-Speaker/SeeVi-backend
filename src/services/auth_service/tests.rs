use super::super::tests::MockDatabase;
use crate::models::education::Education;
use crate::models::users::create_user_input::CreateUserInputBuilder;
use crate::models::users::{CreateUserInput, UpdateUserInput, User};
use crate::services::auth_service::{AuthService, Claims};
use mongodb::bson::Uuid;

pub fn create_demo_user_input(test_uuid: Uuid) -> CreateUserInput {
    let demo = CreateUserInputBuilder::default()
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
        .with_avatar(test_uuid)
        .with_cover_photo(test_uuid)
        .with_education(Education {
            school: "school 1".to_string(),
            major: "major 1".to_string(),
            minor: Some("minor 1".to_string()),
            degree: "degree 1".to_string(),
            start_date: None,
            end_date: None
        })
        .with_education(Education {
            school: "school 2".to_string(),
            major: "major 2".to_string(),
            minor: Some("minor 2".to_string()),
            degree: "degree 2".to_string(),
            start_date: None,
            end_date: None
        })
        .build()
        .unwrap();
    demo
}

#[tokio::test]
async fn duplicate_register_user_test() {
    dotenv::dotenv().ok();
    let mut db = MockDatabase::new();
    let uuid = Uuid::new();
    let user = create_demo_user_input(uuid);
    let user_clone = user.clone();
    AuthService::register(&mut db, user).await.unwrap();
    AuthService::register(&mut db, user_clone)
        .await
        .expect_err("Should fail to register duplicate user");
}

#[tokio::test]
async fn register_user_test() {
    dotenv::dotenv().ok();
    let mut db = MockDatabase::new();
    let uuid = Uuid::new();
    let user = create_demo_user_input(uuid);
    let user2 = AuthService::register(&mut db, user).await.unwrap();
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
            school: "school 1".to_string(),
            major: "major 1".to_string(),
            minor: Some("minor 1".to_string()),
            degree: "degree 1".to_string(),
            start_date: None,
            end_date: None
        },
        Education {
            school: "school 2".to_string(),
            major: "major 2".to_string(),
            minor: Some("minor 2".to_string()),
            degree: "degree 2".to_string(),
            start_date: None,
            end_date: None
        }]
    );
}

#[tokio::test]
async fn authenticate_user_test() {
    dotenv::dotenv().ok();
    let mut db = MockDatabase::new();
    let uuid = Uuid::new();
    let user = create_demo_user_input(uuid);
    let user2 = AuthService::register(&mut db, user).await.unwrap();
    let key = b"secret";
    let token = AuthService::authenticate(
        &mut db,
        Some(user2.username),
        None,
        "test_password".to_string(),
    )
    .await
    .unwrap();
    let token_data = jsonwebtoken::decode::<Claims>(
        &token.0,
        &jsonwebtoken::DecodingKey::from_secret(key.as_ref()),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .unwrap();
    assert_eq!(token_data.claims.sub, "test_user");
}

#[tokio::test]
async fn invalid_authenticate() {
    dotenv::dotenv().ok();
    let mut db = MockDatabase::new();
    let uuid = Uuid::new();
    let user = create_demo_user_input(uuid);
    let user2 = AuthService::register(&mut db, user).await.unwrap();
    let _token = AuthService::authenticate(
        &mut db,
        Some(user2.username),
        None,
        // Introduce wrong password
        "test_password123".to_string(),
    )
    .await
    .expect_err("Should return error due to wrong password");
}
