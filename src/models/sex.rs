use async_graphql::{SimpleObject, InputObject};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Sex{
    Female,
    Male,
    Others(String),
}