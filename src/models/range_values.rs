use async_graphql::{SimpleObject, InputObject};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub struct RangeValues{
    pub lower: f64,
    pub upper: f64
}
