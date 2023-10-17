use super::LikeService;
use crate::{
    models::{
        cv::{create_cv_input::CreateCVInputBuilder, Like as CVLike},
        users::create_user_input::CreateUserInputBuilder,
    },
    services::{cv_service::cv_service::CVService, user_service::UserService},
};
use async_graphql::futures_util::{self, StreamExt};
use futures_core::stream::BoxStream;
use mongodb::bson::oid::ObjectId;

use crate::{
    data_source::cv,
    services::tests::{CVInteractionError, MockDatabase},
};

#[async_trait::async_trait]
impl cv::like::LikeDataSource for MockDatabase {
    type Error = CVInteractionError;

    async fn add_like(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        let mut likes = self.cv_likes.lock().unwrap();
        for like in likes.iter() {
            if *like.user_id() == user_id && *like.cv_id() == cv_id {
                return Err(CVInteractionError::AlreadyExists);
            }
        }
        likes.push(CVLike::new(user_id, cv_id));
        Ok(())
    }

    async fn delete_like(&self, user_id: ObjectId, cv_id: ObjectId) -> Result<(), Self::Error> {
        let mut likes = self.cv_likes.lock().unwrap();
        println!("Likes: {:#?}", likes);
        for like in likes.iter() {
            if *like.user_id() == user_id && *like.cv_id() == cv_id {
                likes.retain(|like| *like.user_id() != user_id || *like.cv_id() != cv_id);
                return Ok(());
            }
        }
        Err(CVInteractionError::NotFound)
    }

    async fn get_likes_count(&self, cv_id: ObjectId) -> Result<i32, Self::Error> {
        let likes = self.cv_likes.lock().unwrap();
        Ok(likes.iter().filter(|like| *like.cv_id() == cv_id).count() as i32)
    }

    async fn get_likes(&self, cv_id: ObjectId) -> Result<BoxStream<CVLike>, Self::Error> {
        let likes = self.cv_likes.lock().unwrap().clone();
        let stream = futures_util::stream::iter(likes.into_iter());
        let stream = stream.filter(move |like| {
            let like = like.clone();
            async move { *like.cv_id() == cv_id }
        });
        Ok(stream.map(|like| like).boxed())
    }
}

#[tokio::test]
async fn basic() {
    let db = MockDatabase::new();
    let dummy_user_id = ObjectId::new();
    let dummy_cv_id = ObjectId::new();
    LikeService::like_cv(&db, dummy_user_id, dummy_cv_id)
        .await
        .expect_err("Should fail as no cv or user available");
    let user_id = UserService::create_user(
        &db,
        CreateUserInputBuilder::default()
            .with_primary_email("email1@email.com")
            .with_username("testuser1")
            .with_password("testuser1")
            .build()
            .unwrap(),
    )
    .await
    .unwrap()
    .id;
    let user_id2 = UserService::create_user(
        &db,
        CreateUserInputBuilder::default()
            .with_primary_email("email2@email.com")
            .with_username("testuser2")
            .with_password("testuser2")
            .build()
            .unwrap(),
    )
    .await
    .unwrap()
    .id;
    let user_id3 = UserService::create_user(
        &db,
        CreateUserInputBuilder::default()
            .with_primary_email("email3@email.com")
            .with_username("testuser3")
            .with_password("testuser3")
            .build()
            .unwrap(),
    )
    .await
    .unwrap()
    .id;
    let cv_id = CVService::create_cv(
        &db,
        CreateCVInputBuilder::default()
            .with_title("Test CV")
            .with_author_id(user_id)
            .build()
            .unwrap(),
    )
    .await
    .unwrap()
    .id;
    LikeService::like_cv(&db, user_id.into(), cv_id.into())
        .await
        .unwrap();
    LikeService::like_cv(&db, user_id2.into(), cv_id.into())
        .await
        .unwrap();
    LikeService::like_cv(&db, user_id3.into(), cv_id.into())
        .await
        .unwrap();

    let likes_count = LikeService::get_likes_count_of_cv(&db, cv_id.into())
        .await
        .unwrap();
    assert_eq!(likes_count, 3);
    let likes = LikeService::get_likes_by_cv(&db, cv_id.into())
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;
    let user_ids = likes
        .iter()
        .map(|like| like.as_ref().unwrap().user_id())
        .collect::<Vec<_>>();
    assert_eq!(user_ids.len(), 3);
    assert!(user_ids.contains(&&user_id.into()));
    assert!(user_ids.contains(&&user_id2.into()));
    assert!(user_ids.contains(&&user_id3.into()));

    LikeService::unlike_cv(&db, user_id.into(), cv_id.into())
        .await
        .unwrap();

    let likes_count = LikeService::get_likes_count_of_cv(&db, cv_id.into())
        .await
        .unwrap();
    assert_eq!(likes_count, 2);
    let likes = LikeService::get_likes_by_cv(&db, cv_id.into())
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;
    let user_ids = likes
        .iter()
        .map(|like| like.as_ref().unwrap().user_id())
        .collect::<Vec<_>>();
    assert_eq!(user_ids.len(), 2);
    assert!(!user_ids.contains(&&user_id.into()));
    assert!(user_ids.contains(&&user_id2.into()));
    assert!(user_ids.contains(&&user_id3.into()));
}
