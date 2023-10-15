use async_graphql::Enum;
use serde::{Serialize, Deserialize};

#[derive(Eq, Copy, Serialize, Deserialize, Clone, Debug, PartialEq, Enum)]
pub enum Sex{
    Female,
    Male,
    Others
}
