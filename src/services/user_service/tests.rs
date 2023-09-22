use async_graphql::futures_util::StreamExt;

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
    let user = UserService::get_user_by_id(&db, *rs.id).await.unwrap();
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

#[tokio::test]
async fn test_send_then_accept_friend_request() {
    let db = MockDatabase::new();
    let input = mock_user_input();
    let rs = UserService::create_user(&db, input).await.unwrap();
    let input2 = mock_user_input();
    let rs2 = UserService::create_user(&db, input2).await.unwrap();
    let friends_list = UserService::friend_lists(&db, *rs.id)
        .await
        .collect::<Vec<_>>()
        .await;
    assert_eq!(0, friends_list.len());
    UserService::send_friend_request(&db, *rs.id, *rs2.id, Some("hello"))
        .await
        .unwrap();
    let friends_list = UserService::friend_lists(&db, *rs.id)
        .await
        .collect::<Vec<_>>()
        .await;
    assert_eq!(0, friends_list.len());

    UserService::accept_friend_request(&db, *rs.id, *rs2.id)
        .await
        .unwrap_err();

    let friends_list = UserService::friend_lists(&db, *rs.id)
        .await
        .collect::<Vec<_>>()
        .await;
    assert_eq!(0, friends_list.len());

    UserService::accept_friend_request(&db, *rs2.id, *rs.id)
        .await
        .unwrap();

    let friends_list = UserService::friend_lists(&db, *rs.id)
        .await
        .collect::<Vec<_>>()
        .await;
    assert_eq!(1, friends_list.len());
}

#[tokio::test]
async fn test_send_then_decline_friend_request() {
    let db = MockDatabase::new();
    let input = mock_user_input();
    let rs = UserService::create_user(&db, input).await.unwrap();
    let input2 = mock_user_input();
    let rs2 = UserService::create_user(&db, input2).await.unwrap();
    let friends_list = UserService::friend_lists(&db, *rs.id)
        .await
        .collect::<Vec<_>>()
        .await;
    assert_eq!(0, friends_list.len());
    UserService::send_friend_request(&db, *rs.id, *rs2.id, Some("hello"))
        .await
        .unwrap();
    let friends_list = UserService::friend_lists(&db, *rs.id)
        .await
        .collect::<Vec<_>>()
        .await;
    assert_eq!(0, friends_list.len());

    UserService::reject_friend_request(&db, *rs.id, *rs2.id)
        .await
        .unwrap_err();

    let friends_list = UserService::friend_lists(&db, *rs.id)
        .await
        .collect::<Vec<_>>()
        .await;
    assert_eq!(0, friends_list.len());

    UserService::reject_friend_request(&db, *rs2.id, *rs.id)
        .await
        .unwrap();

    let friends_list = UserService::friend_lists(&db, *rs.id)
        .await
        .collect::<Vec<_>>()
        .await;
    assert_eq!(0, friends_list.len());
}
