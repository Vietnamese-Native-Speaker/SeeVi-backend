use async_graphql::{SimpleObject, InputObject, ComplexObject};
use mongodb::bson::DateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, SimpleObject, InputObject, Debug, PartialEq, Builder)]
#[graphql(input_name="EducationInput")]
#[graphql(complex)]
pub struct Education {
    pub school: String,
    pub major: String,
    pub minor: Option<String>,
    pub degree: String,

    #[graphql(skip)]
    pub start_date: Option<DateTime>,
    #[graphql(skip)]
    pub end_date: Option<DateTime>,
}

#[ComplexObject]
impl Education {
    async fn start_date(&self) -> String {
        if let Some(date) = self.start_date{
            date.try_to_rfc3339_string().unwrap()
        }
        else {
            String::new()
        }
    }

    async fn end_date(&self) -> String {
        if let Some(date) = self.end_date{
            date.try_to_rfc3339_string().unwrap()
        }
        else {
            String::new()
        }
    }
}