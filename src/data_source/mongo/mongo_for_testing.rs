use std::ops::{Deref, DerefMut};

use super::MongoDB;
use mongodb::bson::Uuid;

/// Wrapper for MongoDB for testing purpose. It will create a new database with a random name
/// and drop it when it is dropped.
pub struct MongoForTesting(MongoDB);

impl MongoForTesting {
    pub async fn init() -> Self {
        let uuid = Uuid::new().to_string();
        let mongo = MongoDB::init_with_database_name(&uuid).await;
        Self(mongo)
    }
}

impl Drop for MongoForTesting {
    fn drop(&mut self) {
        let db = self.0.db.clone();
        tokio::task::spawn_blocking(|| async move {
            db.drop(None).await.unwrap();
        });
    }
}

impl Deref for MongoForTesting {
    type Target = MongoDB;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MongoForTesting {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
