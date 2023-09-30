use async_graphql::{ComplexObject, SimpleObject};
use mongodb::bson::{self, DateTime, Uuid};
use serde::{Deserialize, Serialize};
use crate::models::{sex::Sex, range_values::RangeValues};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
