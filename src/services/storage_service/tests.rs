#[tokio::test]
async fn test_url_for_get() {
    use super::storage_service::StorageServer;
    use mongodb::bson::Uuid;
    let storage_server = StorageServer::new("./src/services/storage_service/credentials.json".to_string()).await;
    assert!(storage_server.is_ok());
    let storage_server = storage_server.unwrap();
    let user_id = Uuid::new();
    let file_id = Uuid::new();
    let url = storage_server.url_for_get(user_id, file_id).await.unwrap();
    println!("{}", url);
    assert_eq!(1, 1)
}

#[tokio::test]
async fn test_url_for_put() {
    use super::storage_service::StorageServer;
    use mongodb::bson::Uuid;
    let storage_server = StorageServer::new("./src/services/storage_service/credentials.json".to_string()).await;
    assert!(storage_server.is_ok());
    let storage_server = storage_server.unwrap();
    let user_id = Uuid::new();
    let file_id = Uuid::new();
    let url = storage_server.url_for_put(user_id, file_id).await.unwrap();
    println!("{}", url);
    assert_eq!(1, 1)
}