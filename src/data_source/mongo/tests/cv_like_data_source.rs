use mongodb::bson::{Uuid, oid::ObjectId};

use crate::{models::{users::{CreateUserInput, create_user_input::CreateUserInputBuilder}, education::Education, sex::Sex, cv::{CreateCVInput, create_cv_input::CreateCVInputBuilder}}, data_source::{mongo::MongoForTesting, UserDataSource, CVDataSource, cv::like::LikeDataSource}, object_id::ScalarObjectId};

use serial_test::serial;
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
        .with_rating(4.0)
        .with_sex(Sex::Male)
        .with_experiences("year_of_experience")
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

#[tokio::test]
#[serial]
async fn test_add_like(){
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb.add_like(user.id.clone().into(), cv.id.clone().into()).await;
    let count = mongodb.get_likes_count(cv.id.clone().into()).await.unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
#[serial]
async fn test_get_likes_count(){
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb.add_like(user.id.clone().into(), cv.id.clone().into()).await;
    let count = mongodb.get_likes_count(cv.id.clone().into()).await.unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
#[serial]
async fn test_delete_like(){
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb.add_like(user.id.clone().into(), cv.id.clone().into()).await;
    let count = mongodb.get_likes_count(cv.id.clone().into()).await.unwrap();
    assert_eq!(count, 1);
    let result_delete = mongodb.delete_like(user.id.clone().into(), cv.id.clone().into()).await;
    let count2 = mongodb.get_likes_count(cv.id.clone().into()).await.unwrap();
    assert_eq!(count2, 0);
}

#[tokio::test]
#[serial]
async fn test_get_likes(){
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb.add_like(user.id.clone().into(), cv.id.clone().into()).await;
    let stream_like = mongodb.get_likes(cv.id.clone().into()).await.unwrap();
    let fn_test = stream_like.for_each(|like|{
        assert_eq!(ScalarObjectId::from_object_id(*like.user_id()), user.id);
        assert_eq!(ScalarObjectId::from_object_id(*like.cv_id()), cv.id);
        ready(())
    });
    fn_test.await;
}