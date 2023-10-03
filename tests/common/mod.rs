pub mod graphql;

use std::convert::Infallible;

use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use seevi_backend::{
    filters::with_auth_header,
    graphql::{mutation::Mutation, query::Query},
    object_id::ScalarObjectId,
};
use warp::{hyper::StatusCode, Filter, Rejection};

use crate::common;

pub fn print_json<T: serde::Serialize>(t: &T) {
    println!("{}", serde_json::to_string_pretty(t).unwrap());
}

pub fn default_route(
    schema: Schema<Query, Mutation, EmptySubscription>,
) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
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
    let logger = warp::log("testing");

    let routes = warp::path!("graphql")
        .and(graphql_post)
        .with(logger)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }
            panic!("unhandled error: {:?}", err)
        });
    routes
}

pub async fn make_refresh_token_request(
    refresh_token: &str,
    routes: &(impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone + 'static),
) -> serde_json::Value {
    let query = common::graphql::graphql_refresh_token(refresh_token);
    println!("query: {}", query);

    let refresh_token_request = warp::test::request()
        .method("POST")
        .path("/graphql")
        .body(query);
    let reply = refresh_token_request.reply(routes).await.body().clone();
    let reply = serde_json::from_slice::<serde_json::Value>(&reply).unwrap();
    reply
}

/// The field of return value is either "data" or "errors"
pub async fn make_register_request(
    username: &str,
    password: &str,
    routes: &(impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone + 'static),
) -> serde_json::Value {
    let query = common::graphql::graphql_user_register(username, password);
    println!("query: {}", query);

    let register_request = warp::test::request()
        .method("POST")
        .path("/graphql")
        .body(query);
    let reply = register_request.reply(routes).await.body().clone();
    let reply = serde_json::from_slice::<serde_json::Value>(&reply).unwrap();
    reply
}

/// The field of return value is either "data" or "errors"
pub async fn make_login_request(
    username: &str,
    password: &str,
    routes: &(impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone + 'static),
) -> serde_json::Value {
    let query = common::graphql::graphql_user_login(username, password);

    let login_request = warp::test::request()
        .method("POST")
        .path("/graphql")
        .body(query);
    let reply = login_request.reply(routes).await.body().clone();
    let reply = serde_json::from_slice::<serde_json::Value>(&reply).unwrap();
    reply.to_owned()
}

/// The field of return value is either "data" or "errors"
pub async fn user_detail(
    token: String,
    routes: &(impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone + 'static),
) -> serde_json::Value {
    let query = common::graphql::graphql_user_detail();

    let request = warp::test::request()
        .method("POST")
        .path("/graphql")
        .header("Authorization", "Bearer ".to_string() + &token)
        .body(query);
    let reply = request.reply(routes).await.body().clone();
    let reply = serde_json::from_slice::<serde_json::Value>(&reply).unwrap();
    reply.to_owned()
}

pub async fn send_friend_request(
    token: String,
    user_id: ScalarObjectId,
    friend_id: ScalarObjectId,
    message: Option<&str>,
    routes: &(impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone + 'static),
) -> serde_json::Value {
    let query = common::graphql::graphql_send_friend_request(user_id, friend_id, message);
    print_json(&query);

    let request = warp::test::request()
        .method("POST")
        .path("/graphql")
        .header("Authorization", "Bearer ".to_string() + &token)
        .body(query);
    let reply = request.reply(routes).await.body().clone();
    let reply = serde_json::from_slice::<serde_json::Value>(&reply).unwrap();
    reply.to_owned()
}

pub async fn accept_friend_request(
    token: String,
    user_id: ScalarObjectId,
    friend_id: ScalarObjectId,
    routes: &(impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone + 'static),
) -> serde_json::Value {
    let query = common::graphql::graphql_accept_friend_request(user_id, friend_id);
    print_json(&query);

    let request = warp::test::request()
        .method("POST")
        .path("/graphql")
        .header("Authorization", "Bearer ".to_string() + &token)
        .body(query);
    let reply = request.reply(routes).await.body().clone();
    let reply = serde_json::from_slice::<serde_json::Value>(&reply).unwrap();
    reply.to_owned()
}

pub async fn decline_friend_request(
    token: String,
    user_id: ScalarObjectId,
    friend_id: ScalarObjectId,
    routes: &(impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone + 'static),
) -> serde_json::Value {
    let query = common::graphql::graphql_decline_friend_request(user_id, friend_id);
    print_json(&query);

    let request = warp::test::request()
        .method("POST")
        .path("/graphql")
        .header("Authorization", "Bearer ".to_string() + &token)
        .body(query);
    let reply = request.reply(routes).await.body().clone();
    let reply = serde_json::from_slice::<serde_json::Value>(&reply).unwrap();
    reply.to_owned()
}

pub async fn friendslist(
    token: String,
    user_id: ScalarObjectId,
    routes: &(impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone + 'static),
) -> serde_json::Value {
    let query = common::graphql::graphql_friendslist(user_id, None, None, Some(10), None);
    print_json(&query);

    let request = warp::test::request()
        .method("POST")
        .path("/graphql")
        .header("Authorization", "Bearer ".to_string() + &token)
        .body(query);
    let reply = request.reply(routes).await.body().clone();
    let reply = serde_json::from_slice::<serde_json::Value>(&reply).unwrap();
    reply.to_owned()
}

