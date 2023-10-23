use crate::{models::{users::{CreateUserInput, create_user_input::CreateUserInputBuilder}, education::Education, sex::Sex, cv::{CreateCVInput, create_cv_input::CreateCVInputBuilder}}, data_source::{mongo::{MongoForTesting, cv_share_datasource::ShareError}, cv::share::ShareDataSource, UserDataSource, CVDataSource}, object_id::ScalarObjectId};
use mongodb::bson::{oid::ObjectId, Uuid};
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
async fn test_add_share_and_add_share(){
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb.add_share(user.id.clone().into(), cv.id.clone().into()).await;
    let check_share = mongodb.get_share(user.id.clone().into(), cv.id.clone().into()).await.unwrap();
    assert_eq!(ScalarObjectId::from_object_id(*check_share.cv_id()), cv.id);
    assert_eq!(ScalarObjectId::from_object_id(*check_share.user_id()), user.id);
}

#[tokio::test]
#[serial]
async fn test_delete_share(){
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb.add_share(user.id.clone().into(), cv.id.clone().into()).await;
    let result_delete = mongodb.delete_share(user.id.clone().into(), cv.id.clone().into()).await;
    let check_bookmark = mongodb.get_share(user.id.clone().into(), cv.id.clone().into()).await;
    assert_eq!(check_bookmark, Err(ShareError::ShareNotFound));
}

#[tokio::test]
#[serial]
async fn test_get_shares_by_user_id(){
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb.add_share(user.id.clone().into(), cv.id.clone().into()).await;
    let stream_share = mongodb.get_shares_by_user_id(user.id.clone().into()).await.unwrap();
    let fn_test = stream_share.for_each(|check_share|{
        assert_eq!(ScalarObjectId::from_object_id(*check_share.clone().cv_id()), cv.id);
        assert_eq!(ScalarObjectId::from_object_id(*check_share.user_id()), user.id);
        ready(())
    });
    fn_test.await;
}

#[tokio::test]
#[serial]
async fn test_get_shared_cvs_by_user_id(){
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb.add_share(user.id.clone().into(), cv.id.clone().into()).await;
    let stream_cv = mongodb.get_shared_cvs_by_user_id(user.id.clone().into()).await.unwrap();
    let fn_test = stream_cv.for_each(|result_cv|{
        assert_eq!(result_cv.as_ref().unwrap().id, cv.id);
        assert_eq!(result_cv.as_ref().unwrap().author_id, user.id);
        assert_eq!(result_cv.as_ref().unwrap().title, "title".to_string());
        assert_eq!(result_cv.as_ref().unwrap().description, Some("description".to_string()));
        assert_eq!(result_cv.as_ref().unwrap().tags, vec!["tag".to_string(), "tag2".to_string()]);
        ready(())
    });
    fn_test.await;
}

#[tokio::test]
#[serial]
async fn test_get_shares_of_cv(){
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb.add_share(user.id.clone().into(), cv.id.clone().into()).await;
    let stream_share = mongodb.get_shares_of_cv(cv.id.clone().into()).await.unwrap();
    let fn_test = stream_share.for_each(|check_share|{
        assert_eq!(ScalarObjectId::from_object_id(*check_share.clone().unwrap().cv_id()), cv.id);
        assert_eq!(ScalarObjectId::from_object_id(*check_share.unwrap().user_id()), user.id);
        ready(())
    });
    fn_test.await;
}

#[tokio::test]
#[serial]
async fn test_get_shares_count_of_cv(){
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.clone().into());
    let cv = mongodb.create_cv(cv_input).await.unwrap();
    let result_add = mongodb.add_share(user.id.clone().into(), cv.id.clone().into()).await;
    let user_id2 = ObjectId::new();
    let result_add2 = mongodb.add_share(user_id2, cv.id.clone().into()).await;
    let count = mongodb.get_shares_count_of_cv(cv.id.into()).await.unwrap();
    assert_eq!(count, 2);
}