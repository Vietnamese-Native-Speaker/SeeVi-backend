use async_graphql::{SimpleObject, InputObject};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub struct RangeValues{
    pub lower: i32,
    pub upper: i32
}
