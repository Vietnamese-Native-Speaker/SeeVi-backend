use super::super::{cv_data_source::CVDataSource, cv_data_source_error::CVDataSourceError};
use crate::data_source::mongo::MongoDB;
use crate::data_source::user_data_source::UserDataSource;
use crate::models::cv::{
    create_cv_input::{CreateCVInput, CreateCVInputBuilder},
    cv::CV,
};
use crate::models::education::Education;
use crate::models::users::create_user_input::CreateUserInputBuilder;
use mongodb::bson::Uuid;
use mongodb::{bson::serde_helpers::timestamp_as_u32, options::ClientOptions, Client, Database};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_create_cv() {
    let mongodb = MongoDB::init_test().await;
    let uuid = Uuid::new();
    let user = CreateUserInputBuilder::default()
        .with_username("username")
        .with_first_name("first_name")
        .with_last_name("last_name")
        .with_country("country")
        .with_skill("skill")
        .with_primary_email("primary_email")
        .with_other_mail("other_mails")
        .with_other_mail("other_mails2")
        .with_education(Education {
            institution: "University of Example 1".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        })
        .with_education(Education {
            institution: "University of Example 2".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        })
        .with_about("about".to_string())
        .with_avatar(uuid.clone())
        .with_cover_photo(uuid.clone())
        .build()
        .unwrap();
    let input_user = mongodb.create_user(user).await.unwrap();
    let input = CreateCVInputBuilder::default()
        .with_author_id(input_user.user_id)
        .with_title("title")
        .with_description("description")
        .with_tag("tag")
        .with_tag("tag2")
        .build()
        .unwrap();
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
    let user = CreateUserInputBuilder::default()
        .with_username("username")
        .with_first_name("first_name")
        .with_last_name("last_name")
        .with_country("country")
        .with_skill("skill")
        .with_primary_email("primary_email")
        .with_other_mail("other_mails")
        .with_other_mail("other_mails2")
        .with_education(Education {
            institution: "University of Example 1".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        })
        .with_education(Education {
            institution: "University of Example 2".to_string(),
            course: Some("Computer Science".to_string()),
            degree: Some("Bachelor's Degree".to_string()),
        })
        .with_about("about".to_string())
        .with_avatar(uuid.clone())
        .with_cover_photo(uuid.clone())
        .build()
        .unwrap();
    let input_user = mongodb.create_user(user).await.unwrap();
    let input = CreateCVInputBuilder::default()
        .with_author_id(input_user.user_id)
        .with_title("title")
        .with_description("description")
        .with_tag("tag")
        .with_tag("tag2")
        .build()
        .unwrap();
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
