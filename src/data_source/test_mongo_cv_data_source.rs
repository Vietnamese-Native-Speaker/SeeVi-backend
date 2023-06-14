use mongodb::{options::ClientOptions, Client, Database, bson::serde_helpers::timestamp_as_u32};
use mongodb::bson::Uuid;
use crate::models::cv::{
    cv::CV,
    create_cv_input::{CreateCVInput, CreateCVInputBuilder},
};
use super::{cv_data_source::CVDataSource, cv_data_source_error::CVDataSourceError};
use super::mongo::MongoDB;
#[tokio::test]
async fn test_create_cv(){
    let mongodb = MongoDB::init().await;
    let uuid = Uuid::new();
    let input = CreateCVInputBuilder::default()
        .with_author_id(uuid)
        .with_title("title")
        .with_description("description")
        .with_tag("tag")
        .with_tag("tag2")
        .build()
        .unwrap();
    let input_clone = input.clone();
    let check_input = mongodb.create_cv(input).await.unwrap();
    assert_eq!(check_input.author_id, uuid);
    assert_eq!(check_input.title, "title".to_string());
    assert_eq!(check_input.description, Some("description".to_string()));
    assert_eq!(check_input.tags, vec!["tag".to_string(), "tag2".to_string()]);
    assert_eq!(check_input.comments, vec![]);

    let check_input2 = mongodb.create_cv(input_clone).await;
    assert_eq!(check_input2, Err(CVDataSourceError::UuidNotFound(uuid)));
}

#[tokio::test]
async fn get_cv_by_id(){
    let mongodb = MongoDB::init().await;
    let uuid = Uuid::new();
    let input = CreateCVInputBuilder::default()
        .with_author_id(uuid)
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
    assert_eq!(check_input2.author_id, uuid);
    assert_eq!(check_input2.title, "title".to_string());
    assert_eq!(check_input2.description, Some("description".to_string()));
    assert_eq!(check_input2.tags, vec!["tag".to_string(), "tag2".to_string()]);
    assert_eq!(check_input2.comments, vec![]);
}

// #[tokio::test]
// fn test_delete_cv(){

// }
