use crate::data_source::CommentDataSource;
use crate::models::comment::update_comment_input::UpdateCommentInputBuilder;
use crate::models::comment::Comment;
use crate::mongo::MongoForTesting;
use crate::object_id::ScalarObjectId;
use async_graphql::futures_util::StreamExt;
use mongodb::bson::oid::ObjectId;

fn create_test_comment(
    comment_id: ScalarObjectId,
    author_id: ScalarObjectId,
    content: String,
) -> Comment {
    Comment {
        id: comment_id,
        author: author_id,
        content: content,
        created: mongodb::bson::DateTime::now(),
        bookmarks: 0,
        shares: 0,
        replies: vec![],
    }
}

#[tokio::test]
async fn test_create_find_comment() {
    let mongodb = MongoForTesting::init().await;
    let comment_id: ScalarObjectId = ObjectId::new().into();
    let author_id: ScalarObjectId = ObjectId::new().into();
    let comment = create_test_comment(comment_id.clone(), author_id.clone(), "content".to_string());
    mongodb.add_comment(comment).await.unwrap();
    let find_comment = mongodb.get_comment_by_id(comment_id.clone().into()).await;
    assert_eq!(find_comment.unwrap().id, comment_id);
}

#[tokio::test]
async fn test_create_find_list_comments() {
    let mongodb = MongoForTesting::init().await;
    let comment_id: ScalarObjectId = ObjectId::new().into();
    let comment_id2: ScalarObjectId = ObjectId::new().into();
    let comment_id3: ScalarObjectId = ObjectId::new().into();
    let author_id: ScalarObjectId = ObjectId::new().into();
    let comment1 =
        create_test_comment(comment_id.clone(), author_id.clone(), "content".to_string());
    let comment2 = create_test_comment(
        comment_id2.clone(),
        author_id.clone(),
        "content2".to_string(),
    );
    let comment3 = create_test_comment(
        comment_id3.clone(),
        author_id.clone(),
        "content3".to_string(),
    );
    mongodb.add_comment(comment1).await.unwrap();
    mongodb.add_comment(comment2).await.unwrap();
    mongodb.add_comment(comment3).await.unwrap();
    let mut find_comments = mongodb
        .get_comments_list(vec![
            comment_id.clone().into(),
            comment_id2.clone().into(),
            comment_id3.clone().into(),
        ])
        .await
        .collect::<Vec<_>>()
        .await;
    assert_eq!(find_comments.len(), 3);
    assert_eq!(find_comments[0].as_mut().unwrap().id, comment_id);
    assert_eq!(find_comments[1].as_mut().unwrap().id, comment_id2);
    assert_eq!(find_comments[2].as_mut().unwrap().id, comment_id3);
    assert_eq!(find_comments[0].as_mut().unwrap().content, "content");
    assert_eq!(find_comments[1].as_mut().unwrap().content, "content2");
    assert_eq!(find_comments[2].as_mut().unwrap().content, "content3");
}

#[tokio::test]
async fn test_remove_comment() {
    let mongodb = MongoForTesting::init().await;
    let comment_id: ScalarObjectId = ObjectId::new().into();
    let author_id: ScalarObjectId = ObjectId::new().into();
    let comment = create_test_comment(comment_id.clone(), author_id.clone(), "content".to_string());
    mongodb.add_comment(comment).await.unwrap();
    let find_comment = mongodb.get_comment_by_id(comment_id.clone().into()).await;
    assert_eq!(find_comment.unwrap().id, comment_id);
    mongodb
        .remove_comment(comment_id.clone().into())
        .await
        .unwrap();
    let find_deleted_comment = mongodb.get_comment_by_id(comment_id.into()).await;
    assert_eq!(find_deleted_comment.is_err(), true);
}

#[tokio::test]
async fn test_find_and_update_comment() {
    let mongodb = MongoForTesting::init().await;
    let comment_id: ScalarObjectId = ObjectId::new().into();
    let author_id: ScalarObjectId = ObjectId::new().into();
    let comment = create_test_comment(comment_id.clone(), author_id.clone(), "content".to_string());
    mongodb.add_comment(comment).await.unwrap();
    let find_comment = mongodb.get_comment_by_id(comment_id.clone().into()).await;
    assert_eq!(find_comment.unwrap().id, comment_id);
    let update_comment = UpdateCommentInputBuilder::default()
        .with_id(comment_id.clone())
        .with_content("updated content".to_string())
        .with_likes(3 as u32)
        .with_bookmarks(4 as u32)
        .with_shares(5 as u32)
        .build()
        .unwrap();
    mongodb
        .find_and_update_comment(comment_id.clone().into(), update_comment)
        .await
        .unwrap();
    let find_updated_comment = mongodb.get_comment_by_id(comment_id.into()).await;
    assert_eq!(
        find_updated_comment.clone().unwrap().content,
        "updated content"
    );
    assert_eq!(find_updated_comment.clone().unwrap().bookmarks, 4);
    assert_eq!(find_updated_comment.clone().unwrap().shares, 5);
}

#[tokio::test]
async fn test_add_reply_to_comment() {
    let mongodb = MongoForTesting::init().await;
    let comment_id: ScalarObjectId = ObjectId::new().into();
    let comment_id2: ScalarObjectId = ObjectId::new().into();
    let author_id: ScalarObjectId = ObjectId::new().into();
    let comment = create_test_comment(comment_id.clone(), author_id.clone(), "content".to_string());
    mongodb.add_comment(comment).await.unwrap();
    mongodb
        .add_reply_to_comment(comment_id.clone().into(), comment_id2.clone().into())
        .await
        .unwrap();
    let find_comment = mongodb.get_comment_by_id(comment_id.into()).await;
    assert_eq!(find_comment.clone().unwrap().replies.len(), 1);
    assert_eq!(find_comment.unwrap().replies[0], comment_id2);
}

#[tokio::test]
async fn test_remove_reply_from_comment() {
    let mongodb = MongoForTesting::init().await;
    let comment_id: ScalarObjectId = ObjectId::new().into();
    let comment_id2: ScalarObjectId = ObjectId::new().into();
    let author_id: ScalarObjectId = ObjectId::new().into();
    let comment = create_test_comment(comment_id.clone(), author_id.clone(), "content".to_string());
    mongodb.add_comment(comment).await.unwrap();
    mongodb
        .add_reply_to_comment(comment_id.clone().into(), comment_id2.clone().into())
        .await
        .unwrap();
    let find_comment = mongodb.get_comment_by_id(comment_id.clone().into()).await;
    assert_eq!(find_comment.clone().unwrap().replies.len(), 1);
    assert_eq!(find_comment.unwrap().replies[0], comment_id2);
    mongodb
        .find_and_remove_reply(comment_id.clone().into(), comment_id2.clone().into())
        .await
        .unwrap();
    let find_comment = mongodb.get_comment_by_id(comment_id.into()).await;
    assert_eq!(find_comment.unwrap().replies.len(), 0);
}
