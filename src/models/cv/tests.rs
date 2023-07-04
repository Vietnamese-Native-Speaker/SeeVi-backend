use mongodb::bson::Uuid;

use crate::models::cv::{create_cv_input::CreateCVInputBuilder};

use super::{CreateCVInput, update_cv_input::UpdateCVInputBuilder};

#[test]
fn test_create_cv_input_author_id() {
    use mongodb::bson::Uuid;
    let uuid = Uuid::new();
    let test_cv_input = CreateCVInputBuilder::default()
        .with_author_id(uuid)
        .with_title("title")
        .build()
        .unwrap();
    assert_eq!(test_cv_input.author_id, uuid);
}

#[test]
fn test_create_cv_input_title() {
    use mongodb::bson::Uuid;
    let uuid = Uuid::new();
    let test_cv_input = CreateCVInputBuilder::default()
        .with_author_id(uuid)
        .with_title("title")
        .build()
        .unwrap();
    assert_eq!(test_cv_input.title, "title".to_string());
}

#[test]
fn test_create_cv_input_description() {
    use mongodb::bson::Uuid;
    let uuid = Uuid::new();
    let test_cv_input = CreateCVInputBuilder::default()
        .with_author_id(uuid)
        .with_title("title")
        .with_description("description".to_string())
        .build()
        .unwrap();
    assert_eq!(test_cv_input.description, Some("description".to_string()));
}

#[test]
fn test_create_cv_input_tag() {
    use mongodb::bson::Uuid;
    let uuid = Uuid::new();
    let test_cv_input = CreateCVInputBuilder::default()
        .with_author_id(uuid)
        .with_title("title")
        .with_tag("tag".to_string())
        .build()
        .unwrap();
    assert_eq!(test_cv_input.tags, vec!["tag".to_string()]);
}

#[test]
fn test_update_cv() {
    let id = Uuid::new();
    let author_id = Uuid::new();
    let test_cv_update = UpdateCVInputBuilder::default()
        .with__id(id)
        .with_author_id(author_id)
        .with_description("description".to_string())
        .with_tags(vec!["tag1".to_string(), "tag2".to_string()])
        .with_title("title".to_string())
        .build()
        .unwrap();
    assert_eq!(test_cv_update._id, id);
    assert_eq!(test_cv_update.author_id, author_id);
    assert_eq!(test_cv_update.description, Some("description".to_string()));
    assert_eq!(test_cv_update.tags, Some(vec!["tag1".to_string(), "tag2".to_string()]));
    assert_eq!(test_cv_update.title, Some("title".to_string()));
}