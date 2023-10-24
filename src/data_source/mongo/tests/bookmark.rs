use mongodb::bson::{oid::ObjectId, Uuid};
use serial_test::serial;

use crate::{
    data_source::{
        comment::BookmarkDataSource, mongo::MongoForTesting, CommentDataSource, UserDataSource,
    },
    models::{
        comment::{Comment},
        cv::{create_cv_input::CreateCVInputBuilder, CreateCVInput},
        education::Education,
        experience::{ExperienceBuilder},
        sex::Sex,
        users::{create_user_input::CreateUserInputBuilder, CreateUserInput},
    },
    object_id::ScalarObjectId,
};

use async_graphql::futures_util::StreamExt;
use core::future::ready;

fn create_demo_user_input() -> CreateUserInput {
    let id = Uuid::new();
    CreateUserInputBuilder::default()
        .with_password("password")
        .with_username("username")
        .with_first_name("first_name")
        .with_last_name("last_name")
        .with_country("country")
        .with_skill("skill")
        .with_primary_email("primary_email")
        .with_other_mail("other_mails")
        .with_other_mail("other_mails2")
        .with_education(Education {
            school: "school 1".to_string(),
            major: "major 1".to_string(),
            minor: Some("minor 1".to_string()),
            degree: "degree 1".to_string(),
            start_date: None,
            end_date: None,
        })
        .with_education(Education {
            school: "school 2".to_string(),
            major: "major 2".to_string(),
            minor: Some("minor 2".to_string()),
            degree: "degree 2".to_string(),
            start_date: None,
            end_date: None,
        })
        .with_about("about".to_string())
        .with_avatar(id)
        .with_cover_photo(id)
        .with_city("city")
        .with_personalities("personality")
        .with_experience(
            ExperienceBuilder::default()
                .with_title("title")
                .with_company("company")
                .with_description("description")
                .with_employment_type("employment_type")
                .with_location("location")
                .build()
                .unwrap(),
        )
        .with_rating(4.0)
        .with_sex(Sex::Male)
        .build()
        .unwrap()
}

fn create_demo_cv_input(author_id: ObjectId) -> CreateCVInput {
    CreateCVInputBuilder::default()
        .with_author_id(author_id)
        .with_title("title")
        .with_description("description")
        .with_tag("tag")
        .with_tag("tag2")
        .build()
        .unwrap()
}
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
async fn test_add_bookmark_and_get_bookmark() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let comment_id = ScalarObjectId::from(ObjectId::new());
    let comment = create_test_comment(comment_id.clone(), user.id.clone(), "content".to_string());
    let check_comment = mongodb.add_comment(comment).await;
    let check_add = mongodb
        .add_bookmark(user.id.clone().into(), comment_id.clone().into())
        .await;
    let check_bookmark = mongodb
        .get_bookmark(user.id.clone().into(), comment_id.clone().into())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(*check_bookmark.user_id(), user.id);
    assert_eq!(*check_bookmark.comment_id(), comment_id);
}

#[tokio::test]
#[serial]
async fn test_delete_bookmark() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let comment_id = ScalarObjectId::from(ObjectId::new());
    let comment = create_test_comment(comment_id.clone(), user.id.clone(), "content".to_string());
    let check_comment = mongodb.add_comment(comment).await;
    let check_add = mongodb
        .add_bookmark(user.id.clone().into(), comment_id.clone().into())
        .await;
    let check_delete = mongodb
        .delete_bookmark(user.id.clone().into(), comment_id.clone().into())
        .await;
    let check_bookmark = mongodb
        .get_bookmark(user.id.clone().into(), comment_id.clone().into())
        .await
        .unwrap();
    assert_eq!(check_bookmark, None);
}

#[tokio::test]
#[serial]
async fn test_get_bookmarks_of_user() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let comment_id = ScalarObjectId::from(ObjectId::new());
    let comment = create_test_comment(comment_id.clone(), user.id.clone(), "content".to_string());
    let check_comment = mongodb.add_comment(comment).await;
    let check_add = mongodb
        .add_bookmark(user.id.clone().into(), comment_id.clone().into())
        .await;
    let stream_bookmark = mongodb.get_bookmarks_of_user(user.id.into()).await.unwrap();
    let fn_test = stream_bookmark.for_each(|bookmark_result| {
        assert_eq!(*bookmark_result.clone().unwrap().user_id(), user.id);
        assert_eq!(*bookmark_result.clone().unwrap().comment_id(), comment_id);
        ready(())
    });
    fn_test.await;
}
