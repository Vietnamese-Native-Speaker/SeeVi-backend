mod common;

use crate::common::{
    default_route, make_login_request, make_refresh_token_request, make_register_request,
    print_json, user_detail, JsonValueGetter,
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
    let access_token = JsonValueGetter::new(login_result)
        .field("data")
        .field("login")
        .string("accessToken");

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
    let refresh_token = JsonValueGetter::new(login_result)
        .field("data")
        .field("login")
        .string("refreshToken");
    print_json(&refresh_token);

    // make a false refresh token request
    let refresh_result = make_refresh_token_request("false_token", &routes).await;
    print_json(&refresh_result);
    refresh_result
        .get("errors")
        .expect("should have error due to wrong refresh token");

    let refresh_result = make_refresh_token_request(&refresh_token, &routes).await;
    print_json(&refresh_result);
    let access_token = JsonValueGetter::new(refresh_result)
        .field("data")
        .field("refreshToken")
        .string("refreshToken");

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

    // register 3 users
    let user1 = make_register_request("ltp1", "ltp1", &routes).await;
    print_json(&user1);
    let user2 = make_register_request("ltp2", "ltp2", &routes).await;
    print_json(&user2);
    let user3 = make_register_request("ltp3", "ltp3", &routes).await;
    print_json(&user3);

    // get user ids of 3 users
    let user_id1 = user1
        .get("data")
        .expect("should have 'data' field")
        .get("userRegister")
        .expect("should have 'userRegister' field")
        .get("id")
        .expect("should have 'id' field")
        .as_str()
        .unwrap()
        .to_string()
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();
    let user_id2 = user2
        .get("data")
        .expect("should have 'data' field")
        .get("userRegister")
        .expect("should have 'userRegister' field")
        .get("id")
        .expect("should have 'id' field")
        .as_str()
        .unwrap()
        .to_string()
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();
    let user_id3 = user3
        .get("data")
        .expect("should have 'data' field")
        .get("userRegister")
        .expect("should have 'userRegister' field")
        .get("id")
        .expect("should have 'id' field")
        .as_str()
        .unwrap()
        .to_string()
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();

    // login 3 users
    let login_rs1 = make_login_request("ltp1", "ltp1", &routes).await;
    let login_rs2 = make_login_request("ltp2", "ltp2", &routes).await;
    let login_rs3 = make_login_request("ltp3", "ltp3", &routes).await;

    // get access tokens of 3 users
    let access_token1 = login_rs1
        .get("data")
        .expect("should have 'data' field")
        .get("login")
        .expect("should have 'login' field")
        .get("accessToken")
        .expect("should have 'accessToken' field")
        .as_str()
        .unwrap()
        .to_string();
    let access_token2 = login_rs2
        .get("data")
        .expect("should have 'data' field")
        .get("login")
        .expect("should have 'login' field")
        .get("accessToken")
        .expect("should have 'accessToken' field")
        .as_str()
        .unwrap()
        .to_string();
    let access_token3 = login_rs3
        .get("data")
        .expect("should have 'data' field")
        .get("login")
        .expect("should have 'login' field")
        .get("accessToken")
        .expect("should have 'accessToken' field")
        .as_str()
        .unwrap()
        .to_string();

    // send friend request from user1 to user2
    let send_friend_request_rs = common::send_friend_request(
        access_token1.clone(),
        user_id1.clone(),
        user_id2.clone(),
        None,
        &routes,
    )
    .await;
    print_json(&send_friend_request_rs);
    let send_status = send_friend_request_rs
        .get("data")
        .expect("should have 'data' field")
        .get("sendFriendRequest")
        .expect("should have 'sendFriendRequest' field")
        .as_bool()
        .unwrap();
    assert_eq!(send_status, true);

    // send friend request from user1 to user3
    let send_friend_request_rs = common::send_friend_request(
        access_token1.clone(),
        user_id1.clone(),
        user_id3.clone(),
        None,
        &routes,
    )
    .await;
    print_json(&send_friend_request_rs);
    let send_status = send_friend_request_rs
        .get("data")
        .expect("should have 'data' field")
        .get("sendFriendRequest")
        .expect("should have 'sendFriendRequest' field")
        .as_bool()
        .unwrap();
    assert_eq!(send_status, true);

    // accept friend request from user1 to user2 (user2 accept user1's request)
    let accept_friend_request_rs = common::accept_friend_request(
        access_token2.clone(),
        user_id2.clone(),
        user_id1.clone(),
        &routes,
    )
    .await;
    print_json(&accept_friend_request_rs);
    let accept_status = accept_friend_request_rs
        .get("data")
        .expect("should have 'data' field")
        .get("acceptFriendRequest")
        .expect("should have 'acceptFriendRequest' field")
        .as_bool()
        .unwrap();
    assert_eq!(accept_status, true);

    // decline friend request from user1 to user3 (user3 decline user1's request)
    let decline_friend_request_rs = common::decline_friend_request(
        access_token3.clone(),
        user_id3.clone(),
        user_id1.clone(),
        &routes,
    )
    .await;
    print_json(&decline_friend_request_rs);
    let decline_status = JsonValueGetter::new(decline_friend_request_rs)
        .field("data")
        .bool("declineFriendRequest");
    assert_eq!(decline_status, true);

    // get friends list of user1
    let friends_list_rs =
        common::friendslist(access_token1.clone(), user_id1.clone(), &routes).await;
    print_json(&friends_list_rs);
    let friends_list = friends_list_rs
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
