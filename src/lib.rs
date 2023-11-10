#[macro_use]
extern crate derive_builder;

use std::{convert::Infallible, sync::Arc};

use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use data_source::mongo::{self, MongoDB, MongoForTesting};
use filters::{graphql_sdl, with_auth_header};
use graphql::{mutation::Mutation, query::Query};
use mock_data::populate_mocked_data;
use warp::{hyper::StatusCode, Filter, Rejection};

pub mod data_source;
pub mod error;
pub mod filters;
pub mod graphql;
pub mod mock_data;
pub mod models;
pub mod object_id;
pub mod services;

#[derive(Clone)]
pub struct State {
    pub schema: Schema<Query, Mutation, EmptySubscription>,
}

pub async fn run_server_for_test() {
    pretty_env_logger::init();

    let mongo_ds = MongoDB::init_with_database_name("seevi_test").await;
    populate_mocked_data(mongo_ds.clone()).await;

    let mongo_ds = MongoForTesting::from(mongo_ds);

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(mongo_ds)
        .finish();

    let graphql_post = with_auth_header()
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(
            |header,
             (schema, request): (
                Schema<Query, Mutation, EmptySubscription>,
                async_graphql::Request,
            )| async move {
                Ok::<_, Rejection>(GraphQLResponse::from(
                    schema.execute(request.data(header)).await,
                ))
            },
        );
    let logger = warp::log("seevi_backend");

    let routes = warp::path!("graphql" / "playground")
        .and(filters::graphql_playground())
        .with(logger)
        .or(warp::path!("graphql" / "schema").and(graphql_sdl(schema)))
        .with(logger)
        .or(warp::path!("graphql").and(graphql_post))
        .with(logger)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

pub async fn run_server() {
    pretty_env_logger::init();

    let mongo_ds = mongo::MongoDB::init().await;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(mongo_ds)
        .finish();

    let graphql_post = with_auth_header()
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(
            |header,
             (schema, request): (
                Schema<Query, Mutation, EmptySubscription>,
                async_graphql::Request,
            )| async move {
                Ok::<_, Rejection>(GraphQLResponse::from(
                    schema.execute(request.data(header)).await,
                ))
            },
        );
    let logger = warp::log("seevi_backend");

    let routes = warp::path!("graphql" / "playground")
        .and(filters::graphql_playground())
        .with(logger)
        .or(warp::path!("graphql" / "schema").and(graphql_sdl(schema)))
        .with(logger)
        .or(warp::path!("graphql").and(graphql_post))
        .with(logger)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
