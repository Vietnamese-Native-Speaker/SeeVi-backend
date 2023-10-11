use super::super::{cv_data_source::CVDataSource, cv_data_source_error::CVDataSourceError};
use crate::data_source::cv_details_data_source::CVDetailsDataSource;
use crate::data_source::mongo::MongoDB;
use crate::data_source::user_data_source::UserDataSource;
use crate::models::cv::create_cv_input::CreateCVInputBuilder;
use crate::models::cv::CreateCVInput;
use crate::models::cv_details::CVDetails;
use crate::models::cv_details::cv_details::CVDetailsBuilder;
use crate::models::{education::Education, sex::Sex, range_values::RangeValues};
use crate::models::users::create_user_input::CreateUserInputBuilder;
use crate::models::users::CreateUserInput;
use mongodb::bson::Uuid;
use serial_test::serial;
use futures_core::stream::BoxStream;
use tokio_stream::StreamExt;

fn create_demo_user_input(test_uuid: Uuid) -> CreateUserInput {
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
            end_date: None
        })
        .with_education(Education {
            school: "school 2".to_string(),
            major: "major 2".to_string(),
            minor: Some("minor 2".to_string()),
            degree: "degree 2".to_string(),
            start_date: None,
            end_date: None
        })
        .with_about("about".to_string())
        .with_avatar(test_uuid)
        .with_cover_photo(test_uuid)
        .with_city("city")
        .with_personalities("personality")
        .with_rating(4.0)
        .with_sex(Sex::Male)
        .with_year_of_experience("year_of_experience")
        .build()
        .unwrap()
}

fn create_demo_cv_input(test_uuid: Uuid) -> CreateCVInput {
    CreateCVInputBuilder::default()
        .with_author_id(test_uuid)
        .with_title("title")
        .with_description("description")
        .with_tag("tag")
        .with_tag("tag2")
        .build()
        .unwrap()
}

fn create_demo_cv_details() -> CVDetails{
    CVDetailsBuilder::default()
        .with_country("country")
        .with_city("city")
        .with_major("major 1")
        .with_personalities("personality")
        .with_rating(RangeValues{lower:0.0, upper: 5.0})
        .with_search_words("title")
        .with_search_words("tag3")
        .with_sex(Sex::Male)
        .with_year_of_experience("year_of_experience")
        .build()
        .unwrap()
}
#[tokio::test]
#[serial]
async fn test_create_cv() {
    let mongodb = MongoDB::init_test().await;
    let uuid = Uuid::new();
    let user = create_demo_user_input(uuid);
    let input_user = mongodb.create_user(user).await.unwrap();
    let input = create_demo_cv_input(input_user.user_id);
    let check_input = mongodb.create_cv(input).await.unwrap();

    assert_eq!(check_input.author_id, input_user.user_id);
    assert_eq!(check_input.title, "title".to_string());
    assert_eq!(check_input.description, Some("description".to_string()));
    assert_eq!(
        check_input.tags,
        vec!["tag".to_string(), "tag2".to_string()]
    );
    assert_eq!(check_input.comments, vec![]);
}

#[tokio::test]
#[serial]
async fn get_cv_by_id() {
    let mongodb = MongoDB::init_test().await;
    let uuid = Uuid::new();
    let user = create_demo_user_input(uuid);
    let input_user = mongodb.create_user(user).await.unwrap();
    let input = create_demo_cv_input(input_user.user_id);
    let check_input = mongodb.get_cv_by_id(uuid).await;
    assert_eq!(check_input, Err(CVDataSourceError::UuidNotFound(uuid)));
    let check_id = mongodb.create_cv(input).await.unwrap()._id;
    let check_input2 = mongodb.get_cv_by_id(check_id).await.unwrap();
    assert_eq!(check_input2.author_id, input_user.user_id);
    assert_eq!(check_input2.title, "title".to_string());
    assert_eq!(check_input2.description, Some("description".to_string()));
    assert_eq!(
        check_input2.tags,
        vec!["tag".to_string(), "tag2".to_string()]
    );
    assert_eq!(check_input2.comments, vec![]);
}

// #[tokio::test]
// fn test_delete_cv(){

// }

#[tokio::test]
#[serial]
async fn test_get_cvs_by_filter() {
    let mongodb = MongoDB::init_test().await;
    let uuid = Uuid::new();
    let user = create_demo_user_input(uuid);
    let input_user = mongodb.create_user(user).await.unwrap();
    let input = create_demo_cv_input(input_user.user_id);
    let check_input = mongodb.create_cv(input).await.unwrap();
    let cv_filter = create_demo_cv_details();
    let mut stream_cv = mongodb.get_cvs_by_filter(cv_filter).await.unwrap();
    let vec_cv = stream_cv.collect::<Vec<_>>().await;
    assert_eq!(vec_cv.len(), 1);
    assert_eq!(vec_cv[0].tags, vec!["tag".to_string(), "tag2".to_string()]);
}