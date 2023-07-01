use mongodb::bson::Uuid;

use crate::models::cv::create_cv_input::CreateCVInputBuilder;

use super::CreateCVInput;

fn create_demo_cv_input(test_uuid: Uuid) -> CreateCVInput {
    CreateCVInputBuilder::default()
        .with_author_id(test_uuid)
        .with_title("title")
        .with_description("description".to_string())
        .with_tag("tag".to_string())
        .build()
        .unwrap()
}

#[test]
fn test_create_cv_input_author_id() {
    let uuid = Uuid::new();
    let test_cv_input = create_demo_cv_input(uuid);
    assert_eq!(test_cv_input.author_id, uuid);
}

#[test]
fn test_create_cv_input_title() {
    let uuid = Uuid::new();
    let test_cv_input = create_demo_cv_input(uuid);
    assert_eq!(test_cv_input.title, "title".to_string());
}

#[test]
fn test_create_cv_input_description() {
    let uuid = Uuid::new();
    let test_cv_input = create_demo_cv_input(uuid);
    assert_eq!(test_cv_input.description, Some("description".to_string()));
}

#[test]
fn test_create_cv_input_tag() {
    let uuid = Uuid::new();
    let test_cv_input = create_demo_cv_input(uuid);
    assert_eq!(test_cv_input.tags, vec!["tag".to_string()]);
}

#[test] 
fn test_cv_from_input() {
    use mongodb::bson::Uuid;
    use crate::models::cv::cv::CV;
    let uuid = Uuid::new();
    let test_cv_input = CreateCVInputBuilder::default()
        .with_author_id(uuid)
        .with_title("title")
        .with_tag("tag".to_string())
        .build()
        .unwrap();
    let test_cv = CV::from(test_cv_input);
    assert_eq!(test_cv.author_id, uuid);
    assert_eq!(test_cv.title, "title".to_string());
    assert_eq!(test_cv.tags, vec!["tag".to_string()]);
    assert_eq!(test_cv.description, None);
    assert_eq!(test_cv.comments, Vec::default());
    assert!(test_cv.cv.is_some());
}