use async_graphql::futures_util::StreamExt;
use core::future::ready;
use serial_test::serial;

use crate::{
    data_source::{
        cv::like::LikeDataSource,
        mongo::{
            tests::{create_demo_cv_input, create_demo_user_input},
            MongoForTesting,
        },
        CVDataSource, UserDataSource,
    },
    object_id::ScalarObjectId,
};

#[tokio::test]
#[serial]
async fn test_add_like_and_get_like_counts() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb
        .add_like(user.id.clone().into(), cv.id.clone().into())
        .await;
    let count = mongodb.get_likes_count(cv.id.clone().into()).await.unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
#[serial]
async fn test_delete_like() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let _result_add = mongodb
        .add_like(user.id.clone().into(), cv.id.clone().into())
        .await
        .unwrap();
    let count = mongodb.get_likes_count(cv.id.clone().into()).await.unwrap();
    assert_eq!(count, 1);
    let _result_delete = mongodb
        .delete_like(user.id.clone().into(), cv.id.clone().into())
        .await
        .unwrap();
    let count2 = mongodb.get_likes_count(cv.id.clone().into()).await.unwrap();
    assert_eq!(count2, 0);
}

#[tokio::test]
#[serial]
async fn test_get_likes() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb
        .add_like(user.id.clone().into(), cv.id.clone().into())
        .await;
    let stream_like = mongodb.get_likes(cv.id.clone().into()).await.unwrap();
    let fn_test = stream_like.for_each(|like| {
        assert_eq!(ScalarObjectId::from_object_id(*like.user_id()), user.id);
        assert_eq!(ScalarObjectId::from_object_id(*like.cv_id()), cv.id);
        ready(())
    });
    fn_test.await;
}
