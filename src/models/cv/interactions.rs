use crate::object_id::ScalarObjectId;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Key {
    pub from: ObjectId,
    pub to: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Like {
    #[serde(rename = "_id")]
    key: Key,
    #[graphql(skip)]
    pub created: DateTime,
}

#[ComplexObject]
impl Like {
    pub async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Bookmark {
    #[serde(rename = "_id")]
    key: Key,
    #[graphql(skip)]
    pub created: DateTime,
}

#[ComplexObject]
impl Like {
    pub async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}
