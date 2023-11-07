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
    ).await;
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
    ).await;
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
    ).await;
    print_json(&decline_friend_request_rs);
    let decline_status = decline_friend_request_rs
        .get("data")
        .expect("should have 'data' field")
        .get("declineFriendRequest")
        .expect("should have 'declineFriendRequest' field")
        .as_bool()
        .unwrap();
    assert_eq!(decline_status, true);

    // get friends list of user1
    let friends_list_rs = common::friendslist(
        access_token1.clone(),
        user_id1.clone(),
        &routes,
    ).await;
    print_json(&friends_list_rs);
    let friends_list = friends_list_rs
        .get("data")
        .expect("should have 'data' field")
        .get("userDetail")
        .expect("should have 'userDetail' field")
        .get("friends")
        .expect("should have 'friends' field")
        .get("edges")
        .expect("should have 'edges' field")
        .as_array()
        .unwrap();
    assert_eq!(friends_list.len(), 1);
}

#[tokio::test]
async fn test_cv_apis() {
    dotenv::dotenv().ok();

    let mongo_ds = MongoForTesting::init().await;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(mongo_ds)
        .finish();
    let routes = default_route(schema);

    // create a user
    let user1 = make_register_request("ltp1", "ltp1", &routes).await;
    print_json(&user1);

    // login user
    let login_rs1 = make_login_request("ltp1", "ltp1", &routes).await;
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

    // get user id
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

    // create a cv
    let create_cv_rs = common::create_cv(
        access_token1.clone(),
        user_id1.clone(),
        "test title",
        "test description",
        &routes
    ).await;
    assert_eq!(create_cv_rs.get("data").unwrap().get("createCv").unwrap().get("title").unwrap().as_str().unwrap(), "test title");

    // get cv id
    let cv_id = create_cv_rs
        .get("data")
        .expect("should have 'data' field")
        .get("createCv")
        .expect("should have 'createCv' field")
        .get("id")
        .expect("should have 'id' field")
        .as_str()
        .unwrap()
        .to_string()
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();

    // change cv title
    let change_cv_title_rs = common::change_cv_title(
        access_token1.clone(),
        cv_id.clone(),
        "test change title",
        &routes
    ).await;
    assert_eq!(change_cv_title_rs.get("data").unwrap().get("changeCvTitle").unwrap().get("title").unwrap().as_str().unwrap(), "test change title");

    // change cv description
    let change_cv_description_rs = common::change_cv_description(
        access_token1.clone(),
        cv_id.clone(),
        "test change description",
        &routes
    ).await;
    assert_eq!(change_cv_description_rs.get("data").unwrap().get("changeCvDescription").unwrap().get("description").unwrap().as_str().unwrap(), "test change description");

    // add tag
    let add_tag_rs = common::add_tag(
        access_token1.clone(),
        cv_id.clone(),
        "test tag",
        &routes
    ).await;
    assert_eq!(add_tag_rs.get("data").unwrap().get("addOneTag").unwrap().get("tags").unwrap().as_array().unwrap().len(), 1);
    
    // remove tag
    let remove_tag_rs = common::remove_tag(
        access_token1.clone(),
        cv_id.clone(),
        "test tag",
        &routes
    ).await;
    assert_eq!(remove_tag_rs.get("data").unwrap().get("removeOneTag").unwrap().get("tags").unwrap().as_array().unwrap().len(), 0);

    // create user 2
    let user2 = make_register_request("ltp2", "ltp2", &routes).await;
    print_json(&user2);

    // login user 2
    let login_rs2 = make_login_request("ltp2", "ltp2", &routes).await;

    // get token of user 2
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

    // get user id 2
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

    // add comment from user 2 to cv
    let add_comment_rs = common::add_comment(
        access_token2.clone(),
        user_id2.clone(),
        cv_id.clone(),
        "test comment",
        &routes
    ).await;
    print_json(&add_comment_rs);
    assert_eq!(add_comment_rs.get("data").unwrap().get("addComment").unwrap().get("comments").unwrap().get("edges").unwrap().as_array().unwrap().len(), 1);

    assert_eq!(add_comment_rs.get("data").unwrap().get("addComment").unwrap().get("comments").unwrap().get("edges").unwrap().as_array().unwrap()[0].get("node").unwrap().get("content").unwrap().as_str().unwrap(), "test comment");

    // get comment id
    let comment_id = add_comment_rs
        .get("data")
        .expect("should have 'data' field")
        .get("addComment")
        .expect("should have 'addComment' field")
        .get("comments")
        .expect("should have 'comments' field")
        .get("edges")
        .expect("should have 'edges' field")
        .as_array()
        .unwrap()[0]
        .get("node")
        .expect("should have 'node' field")
        .get("id")
        .expect("should have 'id' field")
        .as_str()
        .unwrap()
        .to_string()
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();

    // remove comment from user 2 to cv
    let remove_comment_rs = common::remove_comment(
        access_token2.clone(),
        cv_id.clone(),
        comment_id.clone(),
        &routes
    ).await;
    assert_eq!(remove_comment_rs.get("data").unwrap().get("removeComment").unwrap().get("comments").unwrap().get("edges").unwrap().as_array().unwrap().len(), 0);

    // like cv from user 2
    let like_cv_rs = common::like_cv(
        access_token2.clone(),
        cv_id.clone(),
        user_id2.clone(),
        &routes
    ).await;
    assert_eq!(like_cv_rs.get("data").unwrap().get("shareCv").unwrap(), true); 

    // unlike cv from user 2
    let unlike_cv_rs = common::unlike_cv(
        access_token2.clone(),
        cv_id.clone(),
        user_id2.clone(),
        &routes
    ).await;
    assert_eq!(unlike_cv_rs.get("data").unwrap().get("unshareCv").unwrap(), true);

    // share cv from user 2
    let share_cv_rs = common::share_cv(
        access_token2.clone(),
        cv_id.clone(),
        user_id2.clone(),
        &routes
    ).await;
    assert_eq!(share_cv_rs.get("data").unwrap().get("shareCv").unwrap(), true);

    // unshare cv from user 2
    let unshare_cv_rs = common::unshare_cv(
        access_token2.clone(),
        cv_id.clone(),
        user_id2.clone(),
        &routes
    ).await;
    assert_eq!(unshare_cv_rs.get("data").unwrap().get("unshareCv").unwrap(), true);

    // bookmark cv from user 2
    let bookmark_cv_rs = common::bookmark_cv(
        access_token2.clone(),
        cv_id.clone(),
        user_id2.clone(),
        &routes
    ).await;
    assert_eq!(bookmark_cv_rs.get("data").unwrap().get("bookmarkCv").unwrap(), true);

    // unbookmark cv from user 2
    let unbookmark_cv_rs = common::unbookmark_cv(
        access_token2.clone(),
        cv_id.clone(),
        user_id2.clone(),
        &routes
    ).await;
    assert_eq!(unbookmark_cv_rs.get("data").unwrap().get("unbookmarkCv").unwrap(), true);

    // delete cv
    let delete_cv_rs = common::delete_cv(
        access_token1.clone(),
        cv_id.clone(),
        &routes
    ).await;
    assert_eq!(delete_cv_rs.get("data").unwrap().get("deleteCv").unwrap(), true);
} 

#[tokio::test]
async fn test_cv_comment_apis() {
    // Flow of this test:
    // 1. Create 3 users
    // 2. Login 3 users
    // 3. Create a cv from user 1
    // 4. Add a comment from user 2 to cv
    // 5. Add a comment from user 3 to cv
    // 6. Add reply from user 1 to comment of user 2
    // 7. User 3 like comment of user 2
    // 8. User 3 unlike comment of user 2
    // 9. User 3 bookmark comment of user 2
    // 10. User 3 bookmark comment of user 2 (test duplicate bookmark)
    // 11. User 1 remove reply of user 1
    // 12. User 3 unbookmark comment of user 2
    // 13. User 2 edit comment content
    // 14. User 2 delete comment

    dotenv::dotenv().ok();

    let mongo_ds = MongoForTesting::init().await;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(mongo_ds)
        .finish();
    let routes = default_route(schema);

    // Create 3 users

    let user1 = make_register_request("ltp1", "ltp1", &routes).await;
    print_json(&user1);

    let user2 = make_register_request("ltp2", "ltp2", &routes).await;
    print_json(&user2);

    let user3 = make_register_request("ltp3", "ltp3", &routes).await;
    print_json(&user3);

    // Login 3 users

    let login_rs1 = make_login_request("ltp1", "ltp1", &routes).await;
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
    let id1 = login_rs1
        .get("data")
        .expect("should have 'data' field")
        .get("login")
        .expect("should have 'login' field")
        .get("id")
        .expect("should have 'id' field")
        .as_str()
        .unwrap()
        .to_string()
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();

    let login_rs2 = make_login_request("ltp2", "ltp2", &routes).await;
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
    let id2 = login_rs2
        .get("data")
        .expect("should have 'data' field")
        .get("login")
        .expect("should have 'login' field")
        .get("id")
        .expect("should have 'id' field")
        .as_str()
        .unwrap()
        .to_string()
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();

    let login_rs3 = make_login_request("ltp3", "ltp3", &routes).await;
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
    let id3 = login_rs3
        .get("data")
        .expect("should have 'data' field")
        .get("login")
        .expect("should have 'login' field")
        .get("id")
        .expect("should have 'id' field")
        .as_str()
        .unwrap()
        .to_string()
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();

    // Create a cv from user 1
    let cv1 = common::create_cv(
        access_token1.clone(),
        id1.clone(),
        "test title",
        "test description",
        &routes
    ).await;
    assert_eq!(cv1.get("data").unwrap().get("createCv").unwrap().get("title").unwrap().as_str().unwrap(), "test title");
    let cv_id1 = cv1
        .get("data")
        .expect("should have 'data' field")
        .get("createCv")
        .expect("should have 'createCv' field")
        .get("id")
        .expect("should have 'id' field")
        .as_str()
        .unwrap()
        .to_string()
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();

    // Add a comment from user 2 to cv
    let comment_from_user2 = common::add_comment(
        access_token2.clone(),
        id2.clone(),
        cv_id1.clone(),
        "test comment from user 2",
        &routes
    ).await;
    let comment_id_from_user2 = comment_from_user2
        .get("data")
        .expect("should have 'data' field")
        .get("addCommentToCv")
        .expect("should have 'addCommentToCv' field")
        .get("comments")
        .expect("should have 'comments' field")
        .get("edges")
        .expect("should have 'edges' field")
        .as_array()
        .unwrap()[0]
        .get("node")
        .expect("should have 'node' field")
        .get("id")
        .expect("should have 'id' field")
        .as_str()
        .unwrap()
        .to_string()
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();
    assert_eq!(comment_from_user2.get("data").unwrap().get("addCommentToCv").unwrap().get("comments").unwrap().get("edges").unwrap().as_array().unwrap().len(), 1);
    assert_eq!(comment_from_user2.get("data").unwrap().get("addCommentToCv").unwrap().get("comments").unwrap().get("edges").unwrap().as_array().unwrap()[0].get("node").unwrap().get("content").unwrap().as_str().unwrap(), "test comment from user 2");

    // Add a comment from user 3 to cv
    let comment_from_user3 = common::add_comment(
        access_token3.clone(),
        id3.clone(),
        cv_id1.clone(),
        "test comment from user 3",
        &routes
    ).await;
    assert_eq!(comment_from_user3.get("data").unwrap().get("addCommentToCv").unwrap().get("comments").unwrap().get("edges").unwrap().as_array().unwrap().len(), 2);

    // Add reply from user 1 to comment of user 2
    let reply_from_user1 = common::add_reply_to_comment(
        access_token1.clone(),
        comment_id_from_user2.clone(),
        id1.clone(),
        "test reply from user 1",
        &routes
    ).await;
    assert_eq!(reply_from_user1.get("data").unwrap().get("addReplyToComment").unwrap().get("replies").unwrap().get("edges").unwrap().as_array().unwrap().len(), 1);

    // User 3 like comment of user 2
    let like_comment_from_user3 = common::like_comment(
        access_token3.clone(),
        comment_id_from_user2.clone(),
        id3.clone(),
        &routes
    ).await;
    assert_eq!(like_comment_from_user3.get("data").unwrap().get("likeComment").unwrap(), true);

    // User 3 unlike comment of user 2
    let unlike_comment_from_user3 = common::unlike_comment(
        access_token3.clone(),
        comment_id_from_user2.clone(),
        id3.clone(),
        &routes
    ).await;
    assert_eq!(unlike_comment_from_user3.get("data").unwrap().get("unlikeComment").unwrap(), true);

    // User 3 bookmark comment of user 2
    let bookmark_comment_from_user3 = common::bookmark_comment(
        access_token3.clone(),
        comment_id_from_user2.clone(),
        id3.clone(),
        &routes
    ).await;
    assert_eq!(bookmark_comment_from_user3.get("data").unwrap().get("bookmarkComment").unwrap(), true);

    // User 3 bookmark comment of user 2 (test duplicate bookmark)
    let bookmark_comment_from_user3 = common::bookmark_comment(
        access_token3.clone(),
        comment_id_from_user2.clone(),
        id3.clone(),
        &routes
    ).await;
    assert_eq!(bookmark_comment_from_user3.get("data").unwrap().get("bookmarkComment").unwrap(), false);

    // User 3 unbookmark comment of user 2
    let unbookmark_comment_from_user3 = common::unbookmark_comment(
        access_token3.clone(),
        comment_id_from_user2.clone(),
        id3.clone(),
        &routes
    ).await;
    assert_eq!(unbookmark_comment_from_user3.get("data").unwrap().get("unbookmarkComment").unwrap(), true);

    // User 1 remove reply of user 1
    let remove_reply_from_user1 = common::remove_reply_from_comment(
        access_token1.clone(),
        comment_id_from_user2.clone(),
        id1.clone(),
        &routes
    ).await;
    assert_eq!(remove_reply_from_user1.get("data").unwrap().get("removeReplyFromComment").unwrap(), true);

    // User 2 edit comment content
    let edit_comment_from_user2 = common::update_content_comment(
        access_token2.clone(),
        comment_id_from_user2.clone(),
        "test edit comment from user 2",
        &routes
    ).await;
    assert_eq!(edit_comment_from_user2.get("data").unwrap().get("updateContentComment").unwrap().get("content").unwrap().as_str().unwrap(), "test edit comment from user 2");

    // User 2 delete comment
    let delete_comment_from_user2 = common::remove_comment(
        access_token2.clone(),
        cv_id1.clone(),
        comment_id_from_user2.clone(),
        &routes
    ).await;
    assert_eq!(delete_comment_from_user2.get("data").unwrap().get("deleteComment").unwrap(), true);
}