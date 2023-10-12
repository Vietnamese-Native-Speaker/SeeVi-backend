use google_cloud_storage::http::objects::Object;
use mongodb::bson::{Uuid, oid::ObjectId};

use super::{update_cv_input::UpdateCVInputBuilder, CreateCVInput};
use crate::models::cv::create_cv_input::CreateCVInputBuilder;

fn create_demo_cv_input(test_objectid: ObjectId) -> CreateCVInput {
    CreateCVInputBuilder::default()
        .with_author_id(test_objectid)
        .with_title("title")
        .with_description("description".to_string())
        .with_tag("tag".to_string())
        .build()
        .unwrap()
}

#[test]
fn test_create_cv_input_author_id() {
    let objectid = ObjectId::new();
    let test_cv_input = create_demo_cv_input(objectid);
    assert_eq!(test_cv_input.author_id, objectid);
}

#[test]
fn test_create_cv_input_title() {
    let objectid = ObjectId::new();
    let test_cv_input = create_demo_cv_input(objectid);
    assert_eq!(test_cv_input.title, "title".to_string());
}

#[test]
fn test_create_cv_input_description() {
    let objectid = ObjectId::new();
    let test_cv_input = create_demo_cv_input(objectid);
    assert_eq!(test_cv_input.description, Some("description".to_string()));
}

#[test]
fn test_create_cv_input_tag() {
    let objectid = ObjectId::new();
    let test_cv_input = create_demo_cv_input(objectid);
    assert_eq!(test_cv_input.tags, vec!["tag".to_string()]);
}

#[test]
fn test_cv_from_input() {
    use crate::models::cv::cv::CV;
    let objectid = ObjectId::new();
    let test_cv_input = CreateCVInputBuilder::default()
        .with_author_id(objectid)
        .with_title("title")
        .with_tag("tag".to_string())
        .build()
        .unwrap();
    assert_eq!(test_cv_input.tags, vec!["tag".to_string()]);
    let test_cv = CV::from(test_cv_input);
    assert_eq!(test_cv.author_id, objectid);
    assert_eq!(test_cv.title, "title".to_string());
    assert_eq!(test_cv.tags, vec!["tag".to_string()]);
    assert_eq!(test_cv.description, None);
    assert_eq!(test_cv.comments, Vec::default());
    assert!(test_cv.cv.is_some());
}

#[test]
fn test_update_cv() {
    let id = ObjectId::new();
    let author_id = ObjectId::new();
    let test_cv_update = UpdateCVInputBuilder::default()
        .with_id(id)
        .with_author_id(author_id)
        .with_description("description".to_string())
        .with_tags(vec!["tag1".to_string(), "tag2".to_string()])
        .with_title("title".to_string())
        .build()
        .unwrap();
    assert_eq!(test_cv_update.id, id);
    assert_eq!(test_cv_update.author_id, author_id);
    assert_eq!(test_cv_update.description, Some("description".to_string()));

    assert_eq!(
        test_cv_update.tags,
        Some(vec!["tag1".to_string(), "tag2".to_string()])
    );
    assert_eq!(test_cv_update.title, Some("title".to_string()));
}
