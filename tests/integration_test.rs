mod common;

use crate::common::{
    default_route, make_login_request, make_refresh_token_request, make_register_request,
    print_json, user_detail,
};
use async_graphql::{EmptySubscription, Schema};
use mongodb::bson::oid::ObjectId;
use seevi_backend::data_source::mongo::MongoForTesting;
use seevi_backend::graphql::mutation::Mutation;
use seevi_backend::graphql::query::Query;
use seevi_backend::object_id::ScalarObjectId;

#[tokio::test]
async fn register_and_login() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let mongo_ds = MongoForTesting::init().await;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(mongo_ds)
        .finish();
    let routes = default_route(schema);

    let rs = make_register_request("ltp", "ltp", &routes).await;
    print_json(&rs);
    let login_result = make_login_request("ltp", "ltp", &routes).await;
    print_json(&login_result);
    let access_token = login_result
        .get("data")
        .expect("should have 'data' field")
        .get("login")
        .expect("should have 'login' field")
        .get("accessToken")
        .expect("should have 'accessToken' field")
        .as_str()
        .unwrap()
        .to_string();

    let user_rs = user_detail(access_token, &routes).await;
    print_json(&user_rs);
    let user_rs = user_rs.get("data").unwrap().get("userDetail").unwrap();
    assert_eq!(user_rs.get("username").unwrap().as_str().unwrap(), "ltp");

    let login_result = make_login_request("ltp", "ltp1405", &routes).await;
    print_json(&login_result);
    login_result
        .get("errors")
        .expect("should have error due to wrong password");
}

#[tokio::test]
async fn register_login_refresh_access() {
    dotenv::dotenv().ok();

    let mongo_ds = MongoForTesting::init().await;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(mongo_ds)
        .finish();
    let routes = default_route(schema);

    make_register_request("ltp", "ltp", &routes).await;
    let login_result = make_login_request("ltp", "ltp", &routes).await;
    print_json(&login_result);
    let refresh_token = login_result
        .get("data")
        .expect("should have 'data' field")
        .get("login")
        .expect("should have 'login' field")
        .get("refreshToken")
        .expect("should have 'refreshToken' field")
        .as_str()
        .unwrap()
        .to_string();
    print_json(&refresh_token);

    // make a false refresh token request
    let refresh_result = make_refresh_token_request("false_token", &routes).await;
    print_json(&refresh_result);
    refresh_result
        .get("errors")
        .expect("should have error due to wrong refresh token");

    let refresh_result = make_refresh_token_request(&refresh_token, &routes).await;
    print_json(&refresh_result);
    let access_token = refresh_result
        .get("data")
        .expect("should have 'data' field")
        .get("refreshToken")
        .expect("should have 'refreshToken' field")
        .as_str()
        .unwrap()
        .to_string();

    let user_rs = user_detail(access_token, &routes).await;
    print_json(&user_rs);
    let user_rs = user_rs.get("data").unwrap().get("userDetail").unwrap();
    assert_eq!(user_rs.get("username").unwrap().as_str().unwrap(), "ltp");

    // make a false login request
    let login_result = make_login_request("ltp", "ltp1405", &routes).await;
    print_json(&login_result);
    login_result
        .get("errors")
        .expect("should have error due to wrong password");
}

#[tokio::test]
async fn send_accept_decline_friends_request() {
    dotenv::dotenv().ok();

    let mongo_ds = MongoForTesting::init().await;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(mongo_ds)
        .finish();
    let routes = default_route(schema);

    // register 2 users
    make_register_request("ltp", "ltp", &routes).await;
    let login_result = make_login_request("ltp", "ltp", &routes).await;
    print_json(&login_result);

    let access_token = login_result
        .get("data")
        .expect("should have 'data' field")
        .get("login")
        .expect("should have 'login' field")
        .get("accessToken")
        .expect("should have 'accessToken' field")
        .as_str()
        .unwrap()
        .to_string();

    let user_rs = user_detail(access_token, &routes).await;
    print_json(&user_rs);
    let user_rs = user_rs.get("data").unwrap().get("userDetail").unwrap();

    // id of user 1
    let user_id_str = user_rs.get("id").unwrap().as_str().unwrap().to_string();
    let user_id = user_id_str
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();

    make_register_request("ltp2", "ltp2", &routes).await;
    let login_result = make_login_request("ltp2", "ltp2", &routes).await;
    print_json(&login_result);

    let access_token = login_result
        .get("data")
        .expect("should have 'data' field")
        .get("login")
        .expect("should have 'login' field")
        .get("accessToken")
        .expect("should have 'accessToken' field")
        .as_str()
        .unwrap()
        .to_string();

    let user_rs = user_detail(access_token, &routes).await;
    print_json(&user_rs);
    let user_rs = user_rs.get("data").unwrap().get("userDetail").unwrap();

    // id of user 2
    let friend_id_str = user_rs.get("id").unwrap().as_str().unwrap().to_string();
    let friend_id = friend_id_str
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();

    // send friend request
    let send_friend_request_result =
        crate::common::send_friend_request(user_id, friend_id, None, &routes).await;
    print_json(&send_friend_request_result);
    let send_status = send_friend_request_result
        .get("data")
        .expect("should have 'data' field")
        .get("sendFriendRequest")
        .expect("should have 'sendFriendRequest' field")
        .as_bool()
        .unwrap();
    assert_eq!(send_status, true);

    // accept friend request
    let accept_friend_request_result =
        crate::common::accept_friend_request(friend_id, user_id, &routes).await;
    let accept_status = accept_friend_request_result
        .get("data")
        .expect("should have 'data' field")
        .get("acceptFriendRequest")
        .expect("should have 'acceptFriendRequest' field")
        .as_bool()
        .unwrap();
    assert_eq!(accept_status, true);

    // get friends list
    let friends_list_result = crate::common::friendslist(user_id, &routes).await;
    print_json(&friends_list_result);
    let friends_list = friends_list_result
        .get("data")
        .expect("should have 'data' field")
        .get("friendslist")
        .expect("should have 'friendslist' field")
        .get("edges")
        .expect("should have 'edges' field")
        .as_array()
        .unwrap();
    assert_eq!(friends_list.len(), 1);

    // create another user
    make_register_request("ltp3", "ltp3", &routes).await;
    let login_result = make_login_request("ltp3", "ltp3", &routes).await;
    print_json(&login_result);

    let access_token = login_result
        .get("data")
        .expect("should have 'data' field")
        .get("login")
        .expect("should have 'login' field")
        .get("accessToken")
        .expect("should have 'accessToken' field")
        .as_str()
        .unwrap()
        .to_string();

    let user_rs = user_detail(access_token, &routes).await;
    print_json(&user_rs);
    let user_rs = user_rs.get("data").unwrap().get("userDetail").unwrap();

    // id of user 3
    let friend_id_str_2 = user_rs.get("id").unwrap().as_str().unwrap().to_string();
    let friend_id_2 = friend_id_str_2
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();

    // send friend request
    let send_friend_request_result =
        crate::common::send_friend_request(user_id, friend_id_2, None, &routes).await;
    let send_status = send_friend_request_result
        .get("data")
        .expect("should have 'data' field")
        .get("sendFriendRequest")
        .expect("should have 'sendFriendRequest' field")
        .as_bool()
        .unwrap();
    assert_eq!(send_status, true);

    // decline friend request
    let decline_friend_request_result =
        crate::common::decline_friend_request(friend_id_2, user_id, &routes).await;
    let decline_status = decline_friend_request_result
        .get("data")
        .expect("should have 'data' field")
        .get("declineFriendRequest")
        .expect("should have 'declineFriendRequest' field")
        .as_bool()
        .unwrap();
    assert_eq!(decline_status, true);

    // get friends list
    let friends_list_result = crate::common::friendslist(user_id, &routes).await;
    let friends_list = friends_list_result
        .get("data")
        .expect("should have 'data' field")
        .get("friendslist")
        .expect("should have 'friendslist' field")
        .get("edges")
        .expect("should have 'edges' field")
        .as_array()
        .unwrap();
    assert_eq!(friends_list.len(), 1);
}
