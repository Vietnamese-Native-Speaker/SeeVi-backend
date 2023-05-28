use async_graphql::{SimpleObject, InputObject};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, SimpleObject, InputObject)]
#[graphql(input_name="EducationInput")]
pub struct Education {
    pub institution: String,
    pub course: Option<String>,
    pub degree: Option<String>
}