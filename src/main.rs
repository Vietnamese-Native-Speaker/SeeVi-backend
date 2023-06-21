use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use seevi_backend::{data_source::mongo, graphql::mutation::Mutation};
use std::convert::Infallible;

use seevi_backend::graphql::query::Query;

use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use warp::http::StatusCode;
use warp::{http::Response as HttpResponse, Filter, Rejection};

#[tokio::main]
async fn main() {
    run().await
}

async fn run() {
    let mongo_ds = mongo::MongoDB::init().await;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(mongo_ds)
        .finish();

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, Mutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    let graphiql = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(GraphiQLSource::build().endpoint("/").finish())
    });

    let routes = graphiql
        .or(graphql_post)
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
