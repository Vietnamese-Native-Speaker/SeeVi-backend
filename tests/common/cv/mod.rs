use std::convert::Infallible;

use seevi_backend::{models::cv::CreateCVInput, object_id::ScalarObjectId};
use warp::Filter;

use self::prepared_request::{
    graphql_add_comment_to_cv, graphql_comment_replies, graphql_create_cv, graphql_cv_comments,
    graphql_get_cv, graphql_reply_comment,
};

use super::print_json;

mod prepared_request;

pub async fn make_get_cv_request(
    token: &str,
    id: ScalarObjectId,
    routes: &(impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone + 'static),
) -> serde_json::Value {
    let query = graphql_get_cv(id);
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

pub async fn make_get_replies_of_comment_request(
    token: &str,
    comment_id: ScalarObjectId,
    routes: &(impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone + 'static),
) -> serde_json::Value {
    let query = graphql_comment_replies(comment_id, None, None, Some(10), None);
    let request = warp::test::request()
        .method("POST")
        .path("/graphql")
        .header("Authorization", "Bearer ".to_string() + &token)
        .body(query);
    let reply = request.reply(routes).await.body().clone();
    let reply = serde_json::from_slice::<serde_json::Value>(&reply).unwrap();
    reply.to_owned()
}

pub async fn make_get_comments_of_cv_request(
    token: &str,
    cv_id: ScalarObjectId,
    routes: &(impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone + 'static),
) -> serde_json::Value {
    let query = graphql_cv_comments(cv_id, None, None, Some(10), None);
    let request = warp::test::request()
        .method("POST")
        .path("/graphql")
        .header("Authorization", "Bearer ".to_string() + &token)
        .body(query);
    let reply = request.reply(routes).await.body().clone();
    let reply = serde_json::from_slice::<serde_json::Value>(&reply).unwrap();
    reply.to_owned()
}

pub async fn make_create_cv_request(
    token: &str,
    input: CreateCVInput,
    routes: &(impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone + 'static),
) -> serde_json::Value {
    let query = graphql_create_cv(input);
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

pub async fn make_add_comment_to_cv_request(
    token: &str,
    cv_id: ScalarObjectId,
    author_id: ScalarObjectId,
    content: &str,
    routes: &(impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone + 'static),
) -> serde_json::Value {
    let query = graphql_add_comment_to_cv(&token, cv_id, author_id, content);
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

pub async fn make_add_reply_to_comment_request(
    token: &str,
    comment_id: ScalarObjectId,
    content: &str,
    routes: &(impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone + 'static),
) -> serde_json::Value {
    let query = graphql_reply_comment(&token, comment_id, &content);
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
