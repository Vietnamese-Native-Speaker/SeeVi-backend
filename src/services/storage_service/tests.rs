#[tokio::test]
async fn test_url_for_get() {
    use super::storage_service::StorageServer;
    use mongodb::bson::Uuid;
    let storage_server = StorageServer::new("./src/services/storage_service/credentials.json".to_string()).await;
    assert!(storage_server.is_ok());
    let storage_server = storage_server.unwrap();
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
    let storage_server = StorageServer::new("./src/services/storage_service/credentials.json".to_string()).await;
    assert!(storage_server.is_ok());
    let storage_server = storage_server.unwrap();
    let file_path = "C:/Users/ADMIN/Downloads/background/Elyyy.jpg".to_string();
    let file = File::open(file_path).await;
    assert!(file.is_ok());
    let user_id = Uuid::parse_str("07265f01-ba6f-4fc4-b72c-6ba8005272b0").unwrap();
    let file = file.unwrap();
    let result = storage_server.put_file(file, user_id).await;
    println!("{:?}", result);
    assert!(result.is_ok());
}
