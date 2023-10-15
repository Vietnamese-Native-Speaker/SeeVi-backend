use crate::data_source::mongo::MongoForTesting;
use crate::data_source::{CVDataSource, CVDataSourceError, CVDetailsDataSource, UserDataSource};
use crate::models::cv::create_cv_input::CreateCVInputBuilder;
use crate::models::cv::CreateCVInput;
use crate::models::cv_details::cv_details::CVDetailsBuilder;
use crate::models::cv_details::CVDetails;
use crate::models::education::Education;
use crate::models::range_values::RangeValues;
use crate::models::sex::Sex;
use crate::models::users::create_user_input::CreateUserInputBuilder;
use crate::models::users::CreateUserInput;
use async_graphql::futures_util::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Uuid;
use serial_test::serial;
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
        .with_year_of_experience("year_of_experience")
        .build()
        .unwrap()
}
fn create_demo_cv_details() -> CVDetails {
    CVDetailsBuilder::default()
        .with_country("country")
        .with_city("city")
        .with_major("major 1")
        .with_personalities("personality")
        .with_rating(RangeValues {
            lower: 0.0,
            upper: 5.0,
        })
        .with_search_words("title")
        .with_search_words("tag3")
        .with_sex(Sex::Male)
        .with_year_of_experience("year_of_experience")
        .build()
        .unwrap()
}

fn create_demo_user_input2() -> CreateUserInput {
    let id = Uuid::new();
    CreateUserInputBuilder::default()
        .with_password("password")
        .with_username("username2")
        .with_first_name("First Name 2")
        .with_last_name("Last Name 2")
        .with_country("country")
        .with_skill("skill")
        .with_primary_email("primary_email")
        .with_other_mail("other_mails")
        .with_education(Education {
            school: "school 3".to_string(),
            major: "major 3".to_string(),
            minor: Some("minor 3".to_string()),
            degree: "degree 3".to_string(),
            start_date: None,
            end_date: None,
        })
        .with_education(Education {
            school: "school 4".to_string(),
            major: "major 4".to_string(),
            minor: Some("minor 4".to_string()),
            degree: "degree 4".to_string(),
            start_date: None,
            end_date: None,
        })
        .with_other_mail("other_mails2")
        .with_about("about".to_string())
        .with_avatar(id)
        .with_cover_photo(id)
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
async fn test_get_cvs_by_author_id() {
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user_input2 = create_demo_user_input2();
    let user = mongodb.create_user(user_input).await.unwrap();
    let user2 = mongodb.create_user(user_input2).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.into());
    let cv1 = mongodb.create_cv(cv_input).await.unwrap();
    let cv_input = create_demo_cv_input(user.id.into());
    let cv2 = mongodb.create_cv(cv_input).await.unwrap();
    let cv_input = create_demo_cv_input(user2.id.into());
    let cv3 = mongodb.create_cv(cv_input).await.unwrap();
    let cvs = mongodb
        .get_cvs_by_user_id(user.id.into())
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;
    let cvs = cvs.into_iter().map(|cv| cv.unwrap()).collect::<Vec<_>>();
    assert_eq!(cvs.len(), 2);
    assert_eq!(cvs[0].id, cv1.id);
    assert_eq!(cvs[1].id, cv2.id);
    let cvs = mongodb
        .get_cvs_by_user_id(user2.id.into())
        .await
        .unwrap()
        .collect::<Vec<_>>()
        .await;
    let cvs = cvs.into_iter().map(|cv| cv.unwrap()).collect::<Vec<_>>();
    assert_eq!(cvs.len(), 1);
    assert_eq!(cvs[0].id, cv3.id);
}

#[tokio::test]
#[serial]
async fn test_create_cv() {
    let mongodb = MongoForTesting::init().await;
    let input = create_demo_user_input();
    let user = mongodb.create_user(input).await.unwrap();
    let input = create_demo_cv_input(user.id.into());
    let check_input = mongodb.create_cv(input).await.unwrap();

    assert_eq!(check_input.author_id, user.id.into());
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
    let mongodb = MongoForTesting::init().await;
    let user_input = create_demo_user_input();
    let user = mongodb.create_user(user_input).await.unwrap();
    println!("{}", *user.id);
    println!(
        "{}",
        *mongodb.get_user_by_id(user.id.into()).await.unwrap().id
    );
    let cv_input = create_demo_cv_input(user.id.into());

    let fake_id = ObjectId::new();
    let check_input = mongodb.get_cv_by_id(fake_id).await;
    assert_eq!(check_input, Err(CVDataSourceError::IdNotFound(fake_id)));

    let cv_id = mongodb.create_cv(cv_input).await.unwrap().id;
    println!("{}", *cv_id);
    let check_input2 = mongodb.get_cv_by_id(cv_id.into()).await.unwrap();
    assert_eq!(check_input2.author_id, user.id);
    assert_eq!(check_input2.title, "title".to_string());
    assert_eq!(check_input2.description, Some("description".to_string()));
    assert_eq!(
        check_input2.tags,
        vec!["tag".to_string(), "tag2".to_string()]
    );
    assert_eq!(check_input2.comments, vec![]);
}

#[tokio::test]
#[serial]
async fn test_get_cvs_by_filter() {
    let mongodb = MongoForTesting::init().await;
    let user = create_demo_user_input();
    let input_user = mongodb.create_user(user).await.unwrap();
    let input = create_demo_cv_input(input_user.id.into());
    let check_input = mongodb.create_cv(input).await.unwrap();
    let cv_filter = create_demo_cv_details();
    let stream_cv = mongodb.get_cvs_by_filter(cv_filter).await.unwrap();
    let vec_cv = stream_cv.collect::<Vec<_>>().await;
    assert_eq!(vec_cv.len(), 1);
    assert_eq!(vec_cv[0].tags, vec!["tag".to_string(), "tag2".to_string()]);
}

