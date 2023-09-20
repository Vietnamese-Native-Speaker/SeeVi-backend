use std::collections::HashMap;

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
                "otherMails": [],
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
