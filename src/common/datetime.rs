use async_graphql::{Scalar, ScalarType};
use mongodb::bson;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct DateTime(bson::DateTime);

impl DateTime {
    pub fn now() -> Self {
        Self(bson::DateTime::now())
    }
}

impl From<bson::DateTime> for DateTime {
    fn from(dt: bson::DateTime) -> Self {
        Self(dt)
    }
}

impl From<DateTime> for bson::DateTime {
    fn from(dt: DateTime) -> Self {
        dt.0
    }
}

#[Scalar]
impl ScalarType for DateTime {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        let dt = bson::DateTime::parse_rfc3339_str(value.into_value().to_string()).unwrap();
        Ok(dt.into())
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.try_to_rfc3339_string().unwrap())
    }
}

impl std::ops::Deref for DateTime {
    type Target = bson::DateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
