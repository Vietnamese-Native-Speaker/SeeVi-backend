use std::collections::HashMap;

use seevi_backend::object_id::ScalarObjectId;

use super::make_graphql;

pub static USER_DETAIL: &str = r#"
query getUser() {
    userDetail {
        id,
        username
    }
}"#;

pub static USER_LOGIN: &str = r#"
query login($info: LoginInfo!) {
    login(loginInfo: $info) {
        accessToken,
        refreshToken,
    }
}"#;

pub static USER_REFRESH_TOKEN: &str = r#"
query refreshToken($token: String!) {
    refreshToken(refreshToken: $token)
}"#;

pub static USER_REGISTER: &str = r#"
mutation userRegister($user: CreateUserInput!) {
    userRegister(newUser: $user) {
        id,
        username
    }
}"#;

pub static USER_FRIENDSLIST: &str = r#"
query friendslist($id: ObjectId!, $after: String, $before: String, $first: Int, $last: Int) {
    friendslist(userId: $id, after: $after, before: $before, first: $first, last: $last) {
        edges {
            node {
                id,
                username
            }
        }
        pageInfo {
            hasNextPage,
            hasPreviousPage,
            startCursor,
            endCursor
        }
    }
}"#;

pub static SEND_FRIEND_REQUEST: &str = r#"
mutation sendFriendRequest($user_id: ScalarObjectId!, $friend_id: ScalarObjectId!, $message: String) {
    sendFriendRequest(userId: $user_id, friendId: $friend_id, message: $message)
}"#;

pub static ACCEPT_FRIEND_REQUEST: &str = r#"
mutation acceptFriendRequest($user_id: ScalarObjectId!, $friend_id: ScalarObjectId!) {
    acceptFriendRequest(userId: $user_id, friendId: $friend_id)
}"#;

pub static DECLINE_FRIEND_REQUEST: &str = r#"
mutation declineFriendRequest($user_id: ScalarObjectId!, $friend_id: ScalarObjectId!) {
    declineFriendRequest(userId: $user_id, friendId: $friend_id)
}"#;

pub fn graphql_refresh_token(refresh_token: &str) -> String {
    make_graphql(
        USER_REFRESH_TOKEN,
        "refreshToken",
        serde_json::json!({ "token": refresh_token }),
    )
}

pub fn graphql_user_register(username: &str, password: &str) -> String {
    make_graphql(
        USER_REGISTER,
        "userRegister",
        serde_json::json!({
            "user": {
                "username": username,
                "password": password,
                "firstName": "firstname",
                "lastName": "lastname",
                "primaryEmail": "someone@gmail.com",
                "otherEmails": [],
                "education": [],
                "skills": []
            }
        }),
    )
}

pub fn graphql_user_login(username: &str, password: &str) -> String {
    make_graphql(
        USER_LOGIN,
        "login",
        serde_json::json!({
            "info": {
                "username": username,
                "password": password
            }
        }),
    )
}

pub fn graphql_user_detail() -> String {
    make_graphql(USER_DETAIL, "getUser", serde_json::json!({}))
}

pub fn graphql_friendslist(
    user_id: ScalarObjectId,
    after: Option<String>,
    before: Option<String>,
    first: Option<usize>,
    last: Option<usize>,
) -> String {
    make_graphql(
        USER_FRIENDSLIST,
        "friendslist",
        serde_json::json!({
            "id": user_id.to_string(),
            "after": after,
            "before": before,
            "first": first,
            "last": last
        }),
    )
}

pub fn graphql_send_friend_request(
    user_id: ScalarObjectId,
    friend_id: ScalarObjectId,
    message: Option<&str>,
) -> String {
    make_graphql(
        SEND_FRIEND_REQUEST,
        "sendFriendRequest",
        serde_json::json!({
            "user_id": user_id.to_string(),
            "friend_id": friend_id.to_string(),
            "message": message
        }),
    )
}

pub fn graphql_accept_friend_request(user_id: ScalarObjectId, friend_id: ScalarObjectId) -> String {
    make_graphql(
        ACCEPT_FRIEND_REQUEST,
        "acceptFriendRequest",
        serde_json::json!({
            "user_id": user_id.to_string(),
            "friend_id": friend_id.to_string()
        }),
    )
}

pub fn graphql_decline_friend_request(
    user_id: ScalarObjectId,
    friend_id: ScalarObjectId,
) -> String {
    make_graphql(
        DECLINE_FRIEND_REQUEST,
        "declineFriendRequest",
        serde_json::json!({
            "user_id": user_id.to_string(),
            "friend_id": friend_id.to_string()
        }),
    )
}
