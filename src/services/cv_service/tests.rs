use async_graphql::futures_util::StreamExt;
use mongodb::bson::{self, bson};

use crate::{
    models::{comment::CreateCommentInput, cv::CreateCVInput},
    services::tests::MockDatabase,
};

use super::comment_service::CommentService;
use super::cv_service::CVService;

fn mock_comment_input() -> CreateCommentInput {
    CreateCommentInput {
        content: "test".to_string(),
        author: bson::oid::ObjectId::new().into(),
    }
}

#[tokio::test]
async fn test_get_comment_by_id() {
    let db = MockDatabase::new();
    let input = mock_comment_input();
    let rs = CommentService::create_comment(&db, input).await.unwrap();
    let comment = CommentService::get_comment_by_id(&db, *rs.id)
        .await
        .unwrap();
    assert_eq!("test", comment.content);
}

#[tokio::test]
async fn test_comment_services() {
    // create a cv
    let db = MockDatabase::new();
    let cv_input = CreateCVInput {
        title: "some_title".to_string(),
        author_id: bson::oid::ObjectId::new().into(),
        description: Some("some_description".to_string()),
        tags: vec!["tag1".to_string(), "tag2".to_string()],
    };
    let test_cv = CVService::create_cv(&db, cv_input).await.unwrap();
    assert_eq!("some_title", test_cv.title);

    let user_id = bson::oid::ObjectId::new();

    // test add comment to cv
    let cv = CVService::add_comment(
        &db,
        *test_cv.id,
        bson::oid::ObjectId::new(),
        "test".to_string(),
    )
    .await
    .unwrap();
    assert_eq!(1, cv.comments.len());

    // test add like to comment
    let comment = CommentService::add_like_comment(&db, user_id, cv.comments[0])
        .await
        .unwrap();
    let total_like = CommentService::get_likes_count(&db, comment.id.into())
        .await
        .unwrap();
    assert_eq!(1, total_like);

    // test remove like from comment
    let comment = CommentService::remove_like_comment(&db, user_id, cv.comments[0])
        .await
        .unwrap();
    let total_like = CommentService::get_likes_count(&db, comment.id.into())
        .await
        .unwrap();
    assert_eq!(0, total_like);

    // test add bookmark to comment
    let comment = CommentService::add_bookmark(&db, user_id, cv.comments[0])
        .await
        .unwrap();
    let total_bookmark = CommentService::get_bookmarks_count(&db, comment.id.into())
        .await
        .unwrap();
    assert_eq!(1, total_bookmark);

    // test remove bookmark from comment
    let comment = CommentService::remove_bookmark(&db, user_id, cv.comments[0])
        .await
        .unwrap();
    let total_bookmark = CommentService::get_bookmarks_count(&db, comment.id.into())
        .await
        .unwrap();
    assert_eq!(0, total_bookmark);

    // test add reply to comment
    let comment = CommentService::add_reply_comment(
        &db,
        *comment.id,
        bson::oid::ObjectId::new(),
        "test_reply".to_string(),
    )
    .await
    .unwrap();
    assert_eq!(1, comment.replies.len());
    let reply = CommentService::get_comment_by_id(&db, comment.replies[0].into())
        .await
        .unwrap();
    assert_eq!("test_reply", reply.content);

    // test remove reply from comment
    let comment = CommentService::remove_reply_comment(&db, *comment.id, comment.replies[0].into())
        .await
        .unwrap();
    assert_eq!(0, comment.replies.len());

    // test update comment content
    let comment =
        CommentService::update_content_comment(&db, *comment.id, "test_update".to_string())
            .await
            .unwrap();
    assert_eq!("test_update", comment.content);

    // add 1 more comment to cv
    let cv = CVService::add_comment(
        &db,
        *cv.id,
        bson::oid::ObjectId::new(),
        "test 2".to_string(),
    )
    .await
    .unwrap();
    assert_eq!(cv.comments.len(), 2);

    // test get comments by cv id
    let comments = CommentService::get_comments_list_by_cv_id(&db, *cv.id)
        .await
        .collect::<Vec<_>>()
        .await;
    assert_eq!(2, comments.len());
    assert_eq!("test 2", comments[1].as_ref().unwrap().content);

    // test remove comment from cv
    let cv = CVService::remove_comment(&db, *cv.id, cv.comments[0])
        .await
        .unwrap();
    assert_eq!(1, cv.comments.len());
}

#[tokio::test]
async fn test_cv_services() {
    // create a cv
    let db = MockDatabase::new();
    let cv_input = CreateCVInput {
        title: "some_title".to_string(),
        author_id: bson::oid::ObjectId::new().into(),
        description: Some("some_description".to_string()),
        tags: vec!["tag1".to_string(), "tag2".to_string()],
    };
    let test_cv = CVService::create_cv(&db, cv_input).await.unwrap();
    assert_eq!("some_title", test_cv.title);

    // test update cv title
    let cv = CVService::change_title(&db, *test_cv.id, "new_title".to_string())
        .await
        .unwrap();
    assert_eq!("new_title", cv.title);

    // test update cv description
    let cv = CVService::change_description(&db, *test_cv.id, "new_description".to_string())
        .await
        .unwrap();
    assert_eq!("new_description", cv.description.unwrap());

    // test add tag to cv
    let cv = CVService::add_tag(&db, *test_cv.id, "tag3".to_string())
        .await
        .unwrap();
    assert_eq!(3, cv.tags.len());

    // test remove tag from cv
    let cv = CVService::remove_tag(&db, *test_cv.id, "tag3".to_string())
        .await
        .unwrap();
    assert_eq!(2, cv.tags.len());
}
