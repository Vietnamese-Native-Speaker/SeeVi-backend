use async_graphql::ParseRequestError;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use seevi_backend::filters::{self, with_auth_header};
use seevi_backend::{data_source::mongo, graphql::mutation::Mutation};
use std::convert::Infallible;
use warp::path;
use warp::reject::{self, Reject};

use seevi_backend::graphql::query::Query;

use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use warp::http::StatusCode;
use warp::{http::Response as HttpResponse, Filter, Rejection};

#[tokio::main]
async fn main() {
    run().await
}

async fn run() {
    pretty_env_logger::init();

    let mongo_ds = mongo::MongoDB::init().await;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(mongo_ds)
        .finish();

    let graphql_post = with_auth_header()
        .and(async_graphql_warp::graphql(schema))
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
