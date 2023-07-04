use std::convert::Infallible;

use async_graphql::{
    http::GraphiQLSource, EmptySubscription, Schema,
};
use async_graphql_warp::GraphQLResponse;
use warp::{http, reject::Reject, Filter, Rejection};

use crate::graphql::{mutation::Mutation, query::Query};

#[derive(Debug)]
enum AuthorizationError {
    InvalidAuthorizationHeader,
}

impl Reject for AuthorizationError {}

fn parse_authorization_header(header: String) -> Result<String, Rejection> {
    if header.starts_with("Bearer ") {
        let token = header.trim_start_matches("Bearer ");
        return Ok(token.to_string());
    } else {
        return Err(warp::reject::custom(
            AuthorizationError::InvalidAuthorizationHeader,
        ));
    }
}

/// This filter get the Authorization Header and verify it.
/// Reject with `AuthorizationError` if the header is invalid.
/// The absence of the header will not create any error.
pub fn with_auth_header() -> impl Filter<Extract = (Option<String>,), Error = Rejection> + Clone {
    warp::header::optional("Authorization").and_then(|auth_header| async move {
        let header = match auth_header {
            Some(header) => Some(parse_authorization_header(header)?),
            None => None,
        };
        Ok::<Option<String>, Rejection>(header)
    })
}

/// The filter which receive the GraphQL request and execute it.
pub fn graphql_handler(schema: Schema<Query, Mutation, EmptySubscription>) -> impl Filter + Clone {
    with_auth_header()
        .and(async_graphql_warp::graphql(schema))
        .and_then(
            |header, (schema, request): (Schema<_, _, _>, async_graphql::Request)| async move {
                Ok::<_, Rejection>(GraphQLResponse::from(
                    schema.execute(request.data(header)).await,
                ))
            },
        )
}

/// The filter which serve the GraphQL Playground.
pub fn graphql_playground(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end().and(warp::get()).map(|| {
        http::Response::builder()
            .header("content-type", "text/html")
            .body(GraphiQLSource::build().endpoint("/graphql").finish())
    })
}

pub fn graphql_sdl(
    schema: Schema<Query, Mutation, EmptySubscription>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Infallible> + Clone {
    warp::any().map(move || {
        http::Response::builder()
            .header("content-type", "text/plain")
            .body(schema.sdl())
    })
}
