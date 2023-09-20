use crate::{
    models::users::{CreateUserInput, UpdateUserInput},
    services::tests::MockDatabase,
};

use super::UserService;

fn mock_user_input() -> CreateUserInput {
    CreateUserInput::builder()
        .with_username("test")
        .with_password("test")
        .with_primary_email("test@mail.com")
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_get_user_by_id() {
    let db = MockDatabase::new();
    let input = mock_user_input();
    let rs = UserService::create_user(&db, input).await.unwrap();
    let user = UserService::get_user_by_id(&db, rs.id).await.unwrap();
    assert_eq!("test", user.username);
    assert_eq!("test", user.password);
}

#[tokio::test]
async fn test_update_user() {
    let db = MockDatabase::new();
    let input = mock_user_input();
    let rs = UserService::create_user(&db, input).await.unwrap();
    let update_input = UpdateUserInput::builder()
        .with_user_id(rs.id)
        .with_password("test2")
        .with_primary_email("test2@gmail.com")
        .build()
        .unwrap();
    let user = UserService::update_user(&db, update_input).await.unwrap();
    assert_eq!("test", user.username);
    assert_eq!("test2", user.password);
    assert_eq!("test2@gmail.com", user.primary_email);
}
