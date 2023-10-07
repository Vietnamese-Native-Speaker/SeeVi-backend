use mongodb::bson::{self, oid::ObjectId};

use super::update_cv_input::UpdateCVInputBuilder;
use crate::models::cv::create_cv_input::CreateCVInputBuilder;

#[test]
fn test_cv_from_input() {
    use crate::models::cv::cv::CV;
    let id = ObjectId::new();
    let test_cv_input = CreateCVInputBuilder::default()
        .with_author_id(id)
        .with_title("title")
        .with_tag("tag".to_string())
        .build()
        .unwrap();
    assert_eq!(test_cv_input.tags, vec!["tag".to_string()]);
    let test_cv = CV::from(test_cv_input);
    assert_eq!(*test_cv.author_id, id);
    assert_eq!(test_cv.title, "title".to_string());
    assert_eq!(test_cv.tags, vec!["tag".to_string()]);
    assert_eq!(test_cv.description, None);
    assert_eq!(test_cv.comments, Vec::default());
    assert!(test_cv.cv.is_some());
}

#[test]
fn test_update_cv() {
    let id =  bson::oid::ObjectId::new().into();
    let author_id = bson::oid::ObjectId::new().into();
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
