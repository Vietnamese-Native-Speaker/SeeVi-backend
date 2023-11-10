use mongodb::bson::{oid::ObjectId, Uuid};
use serial_test::serial;

use crate::{
    data_source::{
        comment::LikeDataSource, mongo::{MongoForTesting, tests::create_demo_user_input}, CommentDataSource, UserDataSource,
    },
    models::comment::Comment,
    object_id::ScalarObjectId,
};

use async_graphql::futures_util::StreamExt;
use core::future::ready;

fn create_test_comment(
    comment_id: ScalarObjectId,
    author_id: ScalarObjectId,
    content: String,
) -> Comment {
    Comment {
        id: comment_id,
        author: author_id,
        content,
        created: mongodb::bson::DateTime::now(),
        replies: vec![],
    }
}

#[tokio::test]
#[serial]
async fn test_add_like_and_get_likes_count() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let comment_id = ScalarObjectId::from(ObjectId::new());
    let comment = create_test_comment(comment_id.clone(), user.id.clone(), "content".to_string());
    let check_comment = mongodb.add_comment(comment).await;
    let check_add = mongodb
        .add_like(user.id.clone().into(), comment_id.clone().into())
        .await;
    let check_like = mongodb
        .get_likes_count_of_comment(comment_id.clone().into())
        .await
        .unwrap();
    assert_eq!(check_like, 1);
}

#[tokio::test]
#[serial]
async fn test_delete_like() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let comment_id = ScalarObjectId::from(ObjectId::new());
    let comment = create_test_comment(comment_id.clone(), user.id.clone(), "content".to_string());
    let check_comment = mongodb.add_comment(comment).await;
    let check_add = mongodb
        .add_like(user.id.clone().into(), comment_id.clone().into())
        .await;
    let check_delete = mongodb
        .delete_like(user.id.clone().into(), comment_id.clone().into())
        .await;
    let check_like = mongodb
        .get_likes_count_of_comment(comment_id.clone().into())
        .await
        .unwrap();
    assert_eq!(check_like, 0);
}

#[tokio::test]
#[serial]
async fn test_get_likes() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let comment_id = ScalarObjectId::from(ObjectId::new());
    let comment = create_test_comment(comment_id.clone(), user.id.clone(), "content".to_string());
    let _check_comment = mongodb.add_comment(comment).await;
    let _check_add = mongodb
        .add_like(user.id.clone().into(), comment_id.clone().into())
        .await;
    let stream_like = mongodb.get_likes(comment_id.into()).await.unwrap();
    let fn_test = stream_like.for_each(|like_result| {
        assert_eq!(like_result.clone().key.user_id, user.id);
        assert_eq!(like_result.clone().key.comment_id, comment_id);
        ready(())
    });
    fn_test.await;
}
