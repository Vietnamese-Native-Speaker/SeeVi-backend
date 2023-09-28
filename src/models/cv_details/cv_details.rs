use async_graphql::{ComplexObject, SimpleObject};
use mongodb::bson::{self, DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::{models::ResourceIdentifier, object_id::ScalarObjectId};
use super::{CreateCVInput, Sex};

/// Struct represents CV defined in the Diagram. Note that this struct only
/// represents the metadata of a CV. Actual implementation of the CV is to
/// be discussed.
#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject, PartialEq)]
#[graphql(complex)]

pub struct RangeValues{
    pub lower: int,
    pub upper: int
}
pub enum Sex{
    Female,
    Male,
    Others(String),
}
pub struct CVDetails {
    pub country: Option<String>,
    pub city: Option<String>,
    pub personalities: Vec<String>,
    pub year_of_experience: Option<String>,
    pub major: String,
    pub search_words: Vec<String>,
    pub range_values: Option<RangeValues>,
    pub sex: Option<Sex>,
}

#[ComplexObject]
impl CVDetails {
    async fn created(&self) -> String {
        self.created.try_to_rfc3339_string().unwrap()
    }
}