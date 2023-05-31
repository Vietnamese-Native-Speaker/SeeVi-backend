use crate::models::cv::create_cv_input::CreateCVInputBuilder;

#[test]
fn test_create_cv_input(){
    use mongodb::bson::Uuid;
    let uuid = Uuid::new();
    let test_cv_input = CreateCVInputBuilder::new(
        uuid,
        "title".to_string(),
        vec!["tag1".to_string()],
    )
    .with_description("description".to_string())
    .with_tags("tag2".to_string())
    .build();
    assert_eq!(test_cv_input.author_id, uuid);
    assert_eq!(test_cv_input.title, "title".to_string());
    assert_eq!(test_cv_input.description, Some("description".to_string()));
    assert_eq!(test_cv_input.tags, vec!["tag1".to_string(), "tag2".to_string()]);
}
