use crate::{
    data_source::{
        cv::share::ShareDataSource,
        mongo::{
            cv_share_datasource::ShareError,
            tests::{create_demo_cv_input, create_demo_user_input},
            MongoForTesting,
        },
        CVDataSource, UserDataSource,
    },
    object_id::ScalarObjectId,
};
use async_graphql::futures_util::StreamExt;
use core::future::ready;
use mongodb::bson::oid::ObjectId;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_add_share_and_add_share() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb
        .add_share(user.id.clone().into(), cv.id.clone().into())
        .await;
    let check_share = mongodb
        .get_share(user.id.clone().into(), cv.id.clone().into())
        .await
        .unwrap();
    assert_eq!(ScalarObjectId::from_object_id(*check_share.cv_id()), cv.id);
    assert_eq!(
        ScalarObjectId::from_object_id(*check_share.user_id()),
        user.id
    );
}

#[tokio::test]
#[serial]
async fn test_delete_share() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb
        .add_share(user.id.clone().into(), cv.id.clone().into())
        .await;
    let result_delete = mongodb
        .delete_share(user.id.clone().into(), cv.id.clone().into())
        .await;
    let check_bookmark = mongodb
        .get_share(user.id.clone().into(), cv.id.clone().into())
        .await;
    assert_eq!(check_bookmark, Err(ShareError::ShareNotFound));
    let count = mongodb.get_shares_count_of_cv(cv.id.into()).await.unwrap();
    assert_eq!(count, 0);
}

#[tokio::test]
#[serial]
async fn test_get_shares_by_user_id() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb
        .add_share(user.id.clone().into(), cv.id.clone().into())
        .await;
    let stream_share = mongodb
        .get_shares_by_user_id(user.id.clone().into())
        .await
        .unwrap();
    let fn_test = stream_share.for_each(|check_share| {
        assert_eq!(
            ScalarObjectId::from_object_id(*check_share.clone().cv_id()),
            cv.id
        );
        assert_eq!(
            ScalarObjectId::from_object_id(*check_share.user_id()),
            user.id
        );
        ready(())
    });
    fn_test.await;
}

#[tokio::test]
#[serial]
async fn test_get_shared_cvs_by_user_id() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb
        .add_share(user.id.clone().into(), cv.id.clone().into())
        .await;
    let stream_cv = mongodb
        .get_shared_cvs_by_user_id(user.id.clone().into())
        .await
        .unwrap();
    let fn_test = stream_cv.for_each(|result_cv| {
        assert_eq!(result_cv.as_ref().unwrap().id, cv.id);
        assert_eq!(result_cv.as_ref().unwrap().author_id, user.id);
        assert_eq!(result_cv.as_ref().unwrap().title, "title".to_string());
        assert_eq!(
            result_cv.as_ref().unwrap().description,
            Some("description".to_string())
        );
        assert_eq!(
            result_cv.as_ref().unwrap().tags,
            vec!["tag".to_string(), "tag2".to_string()]
        );
        ready(())
    });
    fn_test.await;
}

#[tokio::test]
#[serial]
async fn test_get_shares_of_cv() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb
        .add_share(user.id.clone().into(), cv.id.clone().into())
        .await;
    let stream_share = mongodb
        .get_shares_of_cv(cv.id.clone().into())
        .await
        .unwrap();
    let fn_test = stream_share.for_each(|check_share| {
        assert_eq!(
            ScalarObjectId::from_object_id(*check_share.clone().unwrap().cv_id()),
            cv.id
        );
        assert_eq!(
            ScalarObjectId::from_object_id(*check_share.unwrap().user_id()),
            user.id
        );
        ready(())
    });
    fn_test.await;
}

#[tokio::test]
#[serial]
async fn test_get_shares_count_of_cv() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb
        .add_share(user.id.clone().into(), cv.id.clone().into())
        .await;
    let user_id2 = ObjectId::new();
    let result_add2 = mongodb.add_share(user_id2, cv.id.clone().into()).await;
    let count = mongodb.get_shares_count_of_cv(cv.id.into()).await.unwrap();
    assert_eq!(count, 2);
}

