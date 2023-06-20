use google_cloud_auth::credentials::CredentialsFile;
use google_cloud_storage::{client::{ClientConfig, Client}, sign::{SignedURLOptions, SignedURLMethod}, http::objects::upload::{UploadType, Media, UploadObjectRequest}};
use google_cloud_default::WithAuthExt;
use google_cloud_storage::sign::SignedURLError;
use google_cloud_auth::error::Error;
use tokio::fs::File;
use google_cloud_storage::http::Error as httpError;


use mongodb::bson::Uuid;

pub struct StorageServer {
    client: Client,
}

impl StorageServer {
    /// Create a new StorageServer instance.
    /// The credentials_path is the path of the credentials.json file.
    pub async fn new(credentials_path: String) -> Result<StorageServer, Error> {
        let cred = CredentialsFile::new_from_file(credentials_path).await;
        match cred {
            Ok(cred) => {
                let config = ClientConfig::default().with_credentials(cred).await.unwrap();
                let client = Client::new(config);
                Ok(StorageServer {
                    client,
                })
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    /// Return the signed url for download files. When calling, please use the following format:
    /// let url = url_for_get(user_id, file_id).await.unwrap();
    /// user_id and file_id are both Uuid type.
    /// 
    /// For example:
    /// 
    /// async fn test_url_for_get() {
    ///     use super::storage_service::StorageServer;
    ///     use mongodb::bson::Uuid;
    ///     let storage_server = StorageServer::new("./src/services/storage_service/credentials.json".to_string()).await;
    ///     assert!(storage_server.is_ok());
    ///     let storage_server = storage_server.unwrap();
    ///     let user_id = Uuid::new();
    ///     let file_id = Uuid::new();
    ///     let url = storage_server.url_for_get(user_id, file_id).await.unwrap();
    ///     println!("{}", url);
    ///     assert_eq!(1, 1)
    /// }
    /// 
    
    pub async fn url_for_get(&self, user_id: Uuid, file_id: Uuid) -> Result<String, SignedURLError> {
        let bucket = String::from("crispy-garbanzo");
        let object = String::from(user_id.to_string() + "/" + &file_id.to_string());
        let url = self.client
            .signed_url(&bucket, &object, None, None, SignedURLOptions{
                method: SignedURLMethod::GET,
                ..Default::default()
            })
            .await;
        match url {
            Ok(url) => Ok(url),
            Err(e) => Err(e),
        }
    }

    pub async fn put_file(&self, file: File, user_id: Uuid) -> Result<String, httpError> {
        let bucket_name = String::from("crispy-garbanzo");
        let file_id = Uuid::new();
        let mut media = Media::new(user_id.to_string() + "/" + &file_id.to_string());
        media.content_type = "image/jpeg".into();
        let upload_type = UploadType::Simple(media);
        let result = self.client.upload_object(
            &UploadObjectRequest{
                bucket: bucket_name.to_string(),
                ..Default::default()
            }, 
            file, 
            &upload_type
        ).await;
        match result {
            Ok(_) => {
                let url = self.url_for_get(user_id, file_id).await.unwrap();
                Ok(url)
            },
            Err(e) => Err(e),
        }
    }

}