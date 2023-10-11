use async_graphql::{ComplexObject, SimpleObject};
use mongodb::bson::{self, DateTime, Uuid};
use serde::{Deserialize, Serialize};
use crate::models::{sex::Sex, range_values::RangeValues};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder)]
#[builder(pattern = "owned", setter(into, prefix = "with", strip_option))]
pub struct CVDetails {
    pub country: Option<String>,
    pub city: Option<String>,
    #[builder(setter(custom), field(type = "Vec<String>"))]
    pub personalities: Vec<String>,
    pub year_of_experience: Option<String>,
    pub major: Option<String>,
    #[builder(setter(custom), field(type = "Vec<String>"))]
    pub search_words: Vec<String>,
    pub rating: Option<RangeValues>,
    pub sex: Option<Sex>,
}

impl CVDetails{
    pub fn builder() -> CVDetailsBuilder {
        CVDetailsBuilder::default()
    }
}

impl CVDetailsBuilder {
    pub fn with_personalities<T: Into<String>>(mut self, personality: T) -> Self {
        self.personalities.push(personality.into());
        self
    }
    pub fn with_search_words<T: Into<String>>(mut self, search_word: T) -> Self {
        self.search_words.push(search_word.into());
        self
    }
}
