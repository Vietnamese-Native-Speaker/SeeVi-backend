use async_graphql::{ComplexObject, InputObject, SimpleObject};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, SimpleObject, InputObject, Debug, PartialEq, Builder)]
#[graphql(input_name = "EducationInput")]
#[graphql(complex)]
pub struct Education {
    pub school: String,
    pub major: String,
    #[builder(default)]
    pub minor: Option<String>,
    pub degree: String,

    #[graphql(skip)]
    pub start_date: Option<DateTime>,
    #[graphql(skip)]
    pub end_date: Option<DateTime>,
}

#[ComplexObject]
impl Education {
    async fn start_date(&self) -> Option<String> {
        if let Some(date) = self.start_date {
            match date.try_to_rfc3339_string() {
                Ok(string_date) => Some(string_date),
                Err(err) => None,
            }
        } else {
            None
        }
    }

    async fn end_date(&self) -> Option<String> {
        if let Some(date) = self.end_date {
            match date.try_to_rfc3339_string() {
                Ok(string_date) => Some(string_date),
                Err(err) => None,
            }
        } else {
            None
        }
    }
}

