use mongodb::{Client, options::ClientOptions, Database};

pub struct DataSource {
    client: Client,
    pub db: Database,
}

#[allow(dead_code)]
impl DataSource {
    pub async fn client(&self) -> Client {
        self.client.clone()
    }

    pub async fn init() -> DataSource {
        let mut client_options = 
            ClientOptions::parse("mongodb://127.0.0.1:27017")
            .await
            .expect("Failed to parse options!");
        client_options.app_name =
            Some("SeeVi".to_string());
        let client = Client::with_options(client_options)
            .expect("Failed to initialize database!");
        let db = client.database("tmp");
        DataSource { client, db }
    }
}
