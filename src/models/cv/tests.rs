use crate::models::cv::create_cv_input::CreateCVInputBuilder;
#[test]
fn test_create_cv_input_author_id(){
    use mongodb::bson::Uuid;
    let uuid = Uuid::new();
    let test_cv_input = CreateCVInputBuilder::new(
        uuid,
        "title".to_string(),
    )
    .build();
    assert_eq!(test_cv_input.author_id, uuid);
}

#[test]
fn test_create_cv_input_title(){
    use mongodb::bson::Uuid;
    let uuid = Uuid::new();
    let test_cv_input = CreateCVInputBuilder::new(
        uuid,
        "title".to_string(),
    )
    .build();
    assert_eq!(test_cv_input.title, "title".to_string());
}

#[test]
fn test_create_cv_input_description(){
    use mongodb::bson::Uuid;
    let uuid = Uuid::new();
    let test_cv_input = CreateCVInputBuilder::new(
        uuid,
        "title".to_string(),
    )
    .with_description("description".to_string())
    .build();
    assert_eq!(test_cv_input.description, Some("description".to_string()));
}

#[test]
fn test_create_cv_input_tag(){
    use mongodb::bson::Uuid;
    let uuid = Uuid::new();
    let test_cv_input = CreateCVInputBuilder::new(
        uuid,
        "title".to_string(),
    )
    .with_tag("tag".to_string())
    .build();
    assert_eq!(test_cv_input.tags, vec!["tag".to_string()]);
}
