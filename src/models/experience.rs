use async_graphql::{ComplexObject, InputObject, SimpleObject};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, SimpleObject, InputObject, Debug, PartialEq, Builder)]
#[builder(setter(into, prefix = "with", strip_option))]
#[graphql(input_name = "ExperienceInput")]
#[graphql(complex)]
pub struct Experience {
    title: String,
    company: String,
    employment_type: String,
    location: String,
    description: String,

    #[graphql(skip)]
    #[builder(default)]
    start_date: Option<DateTime>,
    #[graphql(skip)]
    #[builder(default)]
    end_date: Option<DateTime>,
}

#[ComplexObject]
impl Experience {
    async fn start_date(&self) -> Option<String> {
        self.start_date
            .map(|date| date.try_to_rfc3339_string().unwrap())
    }

    async fn end_date(&self) -> Option<String> {
        self.end_date
            .map(|date| date.try_to_rfc3339_string().unwrap())
    }
}
