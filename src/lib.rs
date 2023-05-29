use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use graphql::query::Query;

pub mod data_source;
pub mod graphql;
pub mod models;
pub mod services;

#[derive(Clone)]
pub struct State {
    pub schema: Schema<Query, EmptyMutation, EmptySubscription>,
}
