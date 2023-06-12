use google_cloud_auth::credentials::CredentialsFile;
use google_cloud_storage::{client::{ClientConfig, Client}, sign::{SignedURLOptions, SignedURLMethod}};
use google_cloud_default::WithAuthExt;
use google_cloud_storage::sign::SignedURLError;

use mongodb::bson::Uuid;
#[allow(dead_code)]

pub struct StorageServer {
    client: Client,
}

impl StorageServer {
    pub async fn new(credentials_path: String) -> tide::Result<Self> {
        let cred = CredentialsFile::new_from_file(credentials_path).await.unwrap();
        let config = ClientConfig::default().with_credentials(cred).await.unwrap();
        let client = Client::new(config);
        Ok(StorageServer{client})
    }

    pub async fn url_for_get(&self, user_id: Uuid, file_id: Uuid) -> Result<String, SignedURLError> {
        let bucket = user_id.to_string();
        let object = file_id.to_string();
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

    pub async fn url_for_put(&self, user_id: Uuid, file_id: Uuid) -> Result<String, SignedURLError> {
        let bucket = user_id.to_string();
        let object = file_id.to_string();
        let url = self.client
            .signed_url(&bucket, &object, None, None, SignedURLOptions{
                method: SignedURLMethod::PUT,
                ..Default::default()
            })
            .await; 
        match url {
            Ok(url) => Ok(url),
            Err(e) => Err(e),
        }
    }
    
}