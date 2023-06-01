use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use mongodb::bson::Uuid;

use crate::models::{cv::CV, users::User};

use super::user_data_source::UserDataSource;

pub struct InMemoryDB {
    db: Arc<Mutex<DB>>,
}

struct DB {
    users: HashMap<Uuid, User>,
    cvs: HashMap<Uuid, CV>,
}

impl InMemoryDB {
    pub fn init() -> Self {
        Self {
            db: Arc::new(Mutex::new(DB {
                users: HashMap::new(),
                cvs: HashMap::new(),
            })),
        }
    }
}
