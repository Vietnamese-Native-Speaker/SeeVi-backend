mod common;

use crate::common::{
    default_route, make_login_request, make_refresh_token_request, make_register_request,
    print_json, user_detail, JsonValueGetter,
};
use async_graphql::{EmptySubscription, Schema};
use mongodb::bson::oid::ObjectId;
use seevi_backend::{
    data_source::mongo::MongoForTesting,
    graphql::{mutation::Mutation, query::Query},
    models::cv::create_cv_input::{CreateCVInputBuilder, CreateCVInputBuilderError},
    object_id::ScalarObjectId,
};

#[tokio::test]
async fn basic_cv_comment_flow() {
    // Flow of this test:
    // 1. Register 3 users
    // 2. Login 3 users
    // 3. User 1 create new CV
    // 4. User 2 comment on CV of user 1
    // 5. User 3 comment on CV of user 1
    // 6. User 1 reply to comment of user 2
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
    let user_id1 = JsonValueGetter::new(user1)
        .field("data")
        .field("userRegister")
        .string("id")
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();
    let user_id2 = JsonValueGetter::new(user2)
        .field("data")
        .field("userRegister")
        .string("id")
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();
    let user_id3 = JsonValueGetter::new(user3)
        .field("data")
        .field("userRegister")
        .string("should have 'id' field")
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();

    // login 3 users
    let login_rs1 = make_login_request("ltp1", "ltp1", &routes).await;
    let login_rs2 = make_login_request("ltp2", "ltp2", &routes).await;
    let login_rs3 = make_login_request("ltp3", "ltp3", &routes).await;

    // get access tokens of 3 users
    let access_token1 = JsonValueGetter::new(login_rs1)
        .field("data")
        .field("login")
        .string("accessToken");
    let access_token2 = JsonValueGetter::new(login_rs2)
        .field("data")
        .field("login")
        .string("accessToken");
    let access_token3 = JsonValueGetter::new(login_rs3)
        .field("data")
        .field("login")
        .string("accessToken");
    let create_cv_result = crate::common::cv::make_create_cv_request(
        &access_token1,
        CreateCVInputBuilder::default()
            .with_title("User 1 CV")
            .with_description("CV of User 1")
            .build()
            .unwrap(),
        &routes,
    )
    .await;
    let cv_id1 = JsonValueGetter::new(create_cv_result)
        .field("data")
        .field("createCV")
        .string("id")
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();
    let add_comment_result = crate::common::cv::make_add_comment_to_cv_request(
        &access_token2,
        cv_id1,
        user_id2,
        "User 2 comment on User1's CV",
        &routes,
    )
    .await;
    let comment_id1 = JsonValueGetter::new(add_comment_result)
        .field("data")
        .field("addCommentToCV")
        .string("id")
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();

    let add_comment_result = crate::common::cv::make_add_comment_to_cv_request(
        &access_token3,
        cv_id1,
        user_id3,
        "User 3 comment on User1's CV",
        &routes,
    )
    .await;
    let comment_id2 = JsonValueGetter::new(add_comment_result)
        .field("data")
        .field("addCommentToCV")
        .string("id")
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();
    // User 1 reply to comment of user 2
    let reply_to_comment_result = crate::common::cv::make_add_reply_to_comment_request(
        &access_token1,
        comment_id1,
        "User 1 reply to User 2 comment",
        &routes,
    )
    .await;
    let reply_id1 = JsonValueGetter::new(reply_to_comment_result)
        .field("data")
        .field("addReplyToComment")
        .string("id")
        .parse::<ObjectId>()
        .map(Into::<ScalarObjectId>::into)
        .unwrap();
    // Right now CV 1 should have 2 comments
    print_json(&reply_id1);
    let cv1 = crate::common::cv::make_get_cv_request(&access_token1, cv_id1, &routes).await;
    let comments = JsonValueGetter::new(cv1)
        .field("data")
        .field("getCV")
        .array("comments");
    assert_eq!(comments.len(), 2);
    let comments =
        crate::common::cv::make_get_comments_of_cv_request(&access_token1, cv_id1, &routes).await;
    let comments = JsonValueGetter::new(comments).field("data").array("edges");
    let comments = comments
        .iter()
        .map(|v| {
            JsonValueGetter::new(v.clone())
                .field("node")
                .string("content")
        })
        .collect::<Vec<_>>();
    assert_eq!(comments.len(), 2);
    assert!(
        comments[0] == "User 2 comment on User1's CV"
            || comments[0] == "User 3 comment on User1's CV"
    );
    assert!(
        comments[1] == "User 2 comment on User1's CV"
            || comments[1] == "User 3 comment on User1's CV"
    );
    let replies = crate::common::cv::make_get_replies_of_comment_request(
        &access_token1,
        comment_id1,
        &routes,
    )
    .await;
    let replies = JsonValueGetter::new(replies)
        .field("data")
        .field("getRepliesOfComment")
        .array("edges");
    let replies = replies
        .iter()
        .map(|v| {
            JsonValueGetter::new(v.clone())
                .field("node")
                .string("content")
        })
        .collect::<Vec<_>>();
    assert_eq!(replies.len(), 1);
    assert_eq!(replies[0], "User 1 reply to User 2 comment");
}
