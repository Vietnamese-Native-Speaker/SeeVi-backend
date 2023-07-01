#[macro_use]
extern crate derive_builder;

use async_graphql::{EmptySubscription, Schema};
use graphql::{mutation::Mutation, query::Query};

pub mod data_source;
pub mod graphql;
pub mod models;
pub mod services;
pub mod filters;

#[derive(Clone)]
pub struct State {
    pub schema: Schema<Query, Mutation, EmptySubscription>,
}
