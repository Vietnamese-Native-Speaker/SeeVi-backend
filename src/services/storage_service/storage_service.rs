use google_cloud_auth::credentials::CredentialsFile;
use google_cloud_storage::{client::{ClientConfig, Client}, sign::{SignedURLOptions, SignedURLMethod}};
use google_cloud_default::WithAuthExt;
use google_cloud_storage::sign::SignedURLError;
use google_cloud_auth::error::Error;

use mongodb::bson::Uuid;

pub struct StorageServer {
    client: Client,
}

impl StorageServer {
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

    pub async fn url_for_put(&self, user_id: Uuid, file_id: Uuid) -> Result<String, SignedURLError> {
        let bucket = String::from("crispy-garbanzo");
        let object = String::from(user_id.to_string() + "/" + &file_id.to_string());
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