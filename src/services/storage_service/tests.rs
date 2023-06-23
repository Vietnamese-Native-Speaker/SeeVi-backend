#[tokio::test]
async fn test_url_for_get() {
    use super::storage_service::StorageServer;
    use mongodb::bson::Uuid;
    let storage_server = StorageServer::new().await.unwrap();
    let user_id = Uuid::parse_str("07265f01-ba6f-4fc4-b72c-6ba8005272b0").unwrap();
    let file_id = Uuid::parse_str("f9d52206-d17b-4c4d-b18d-b7c3d886f1ac").unwrap();
    let url = storage_server.url_for_get(user_id, file_id).await.unwrap();
    println!("{}", url);
    assert_eq!(1, 1)
}

#[tokio::test]
async fn test_put_file() {
    use super::storage_service::StorageServer;
    use tokio::fs::File;
    use mongodb::bson::Uuid;
    use super::storage_service::UploadFileType;
    use std::path::Path;
    
    let storage_server = StorageServer::new().await.unwrap();
    
    let test_directory = String::from("./testing_files/");
    let test_file_name = String::from("test.txt");
    let test_upload_type = UploadFileType::Text;

    let test_file_path_str = String::from(test_directory + &test_file_name);

    let test_file_path = Path::new(test_file_path_str.as_str());

    let test_file = File::open(test_file_path).await.unwrap();

    let user_id = Uuid::parse_str("07265f01-ba6f-4fc4-b72c-6ba8005272b0").unwrap();
 
    let result = storage_server.put_file(test_file, user_id, test_upload_type).await;
    println!("{:?}", result);
    assert!(result.is_ok());
}