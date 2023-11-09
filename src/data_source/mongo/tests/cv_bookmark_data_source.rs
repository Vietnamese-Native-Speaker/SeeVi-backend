use crate::{
    data_source::{
        cv::bookmark::BookmarkDataSource,
        mongo::{
            cv_bookmark_datasource::BookmarkError,
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
async fn test_add_bookmark_and_get_bookmark() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb
        .add_bookmark(user.id.clone().into(), cv.id.clone().into())
        .await;
    let check_bookmark = mongodb
        .get_bookmark(user.id.clone().into(), cv.id.clone().into())
        .await
        .unwrap();
    assert_eq!(
        ScalarObjectId::from_object_id(*check_bookmark.cv_id()),
        cv.id
    );
    assert_eq!(
        ScalarObjectId::from_object_id(*check_bookmark.user_id()),
        user.id
    );
}

#[tokio::test]
#[serial]
async fn test_delete_bookmark() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb
        .add_bookmark(user.id.clone().into(), cv.id.clone().into())
        .await;
    let count = mongodb
        .get_bookmarks_count_of_cv(cv.id.clone().into())
        .await
        .unwrap();
    assert_eq!(count, 1);
    let result_delete = mongodb
        .delete_bookmark(user.id.clone().into(), cv.id.clone().into())
        .await;
    let check_bookmark = mongodb
        .get_bookmark(user.id.clone().into(), cv.id.clone().into())
        .await;
    assert_eq!(check_bookmark, Err(BookmarkError::BookmarkNotFound));
    let count = mongodb
        .get_bookmarks_count_of_cv(cv.id.clone().into())
        .await
        .unwrap();
    assert_eq!(count, 0);
}

#[tokio::test]
#[serial]
async fn test_get_bookmarks_of_cv() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb
        .add_bookmark(user.id.clone().into(), cv.id.clone().into())
        .await;
    let stream_bookmark = mongodb
        .get_bookmarks_of_cv(cv.id.clone().into())
        .await
        .unwrap();
    let fn_test = stream_bookmark.for_each(|check_bookmark| {
        assert_eq!(
            ScalarObjectId::from_object_id(*check_bookmark.clone().unwrap().cv_id()),
            cv.id
        );
        assert_eq!(
            ScalarObjectId::from_object_id(*check_bookmark.unwrap().user_id()),
            user.id
        );
        ready(())
    });
    fn_test.await;
}

#[tokio::test]
#[serial]
async fn test_get_bookmarks_of_user() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb
        .add_bookmark(user.id.clone().into(), cv.id.clone().into())
        .await;
    let stream_bookmark = mongodb
        .get_bookmarks_of_user(user.id.clone().into())
        .await
        .unwrap();
    let fn_test = stream_bookmark.for_each(|check_bookmark| {
        assert_eq!(
            ScalarObjectId::from_object_id(*check_bookmark.clone().cv_id()),
            cv.id
        );
        assert_eq!(
            ScalarObjectId::from_object_id(*check_bookmark.user_id()),
            user.id
        );
        ready(())
    });
    fn_test.await;
}

#[tokio::test]
#[serial]
async fn test_get_bookmarked_cvs_of_user() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb
        .add_bookmark(user.id.clone().into(), cv.id.clone().into())
        .await;
    let stream_cv = mongodb
        .get_bookmarked_cvs_of_user(user.id.clone().into())
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
async fn test_get_bookmarks_count_of_cv() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb
        .add_bookmark(user.id.clone().into(), cv.id.clone().into())
        .await;
    let user_id2 = ObjectId::new();
    let result_add2 = mongodb.add_bookmark(user_id2, cv.id.clone().into()).await;
    let count = mongodb
        .get_bookmarks_count_of_cv(cv.id.into())
        .await
        .unwrap();
    assert_eq!(count, 2);
}
