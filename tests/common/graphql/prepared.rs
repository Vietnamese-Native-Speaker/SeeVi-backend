use graphql_query_maker::make_graphql;
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
query friendslist($after: String, $before: String, $first: Int, $last: Int) {
    userDetail {
        friends(first: $first, last: $last, after: $after, before: $before) {
            edges {
                node {
                    id,
                    username
                }
                cursor
            }
            pageInfo {
                hasNextPage,
                hasPreviousPage,
                startCursor,
                endCursor
            }
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
                "sex": "MALE",
                "primaryEmail": "someone@gmail.com",
                "otherEmails": [],
                "educations": [],
                "experiences": [],
                "skills": [],
                "personalities": [],
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

make_graphql!(
    mutation create_cv($user_id: ScalarObjectId!, $title: String!, $description: String!) {
        createCv(userId: $user_id, title: $title, description: $description) {
          id,
          authorId,
          title,
          description
        }
    }
);

make_graphql!(
    mutation delete_cv($id: ScalarObjectId!) {
        deleteCv(cvId: $id)
    }
);

make_graphql!(
    mutation change_cv_title($id: ScalarObjectId!, $title: String!) {
        changeCvTitle(cvId: $id, title: $title) {
            title
          }
    }
);

make_graphql!(
    mutation change_cv_description($id: ScalarObjectId!, $description: String!) {
        changeCvDescription(cvId: $id, description: $description) {
            description
        }
    }
);

make_graphql!(
    mutation add_tag($id: ScalarObjectId!, $tag: String!) {
        addOneTag(cvId: $id, tag: $tag) {
            tags
        }
    }
);

make_graphql!(
    mutation remove_tag($id: ScalarObjectId!, $tag: String!) {
        removeOneTag(cvId: $id, tag: $tag) {
            tags
        }
    }
);

make_graphql!(
    mutation add_comment($id: ScalarObjectId!, $author: ScalarObjectId!, $content: String!) {
        addComment(cvId: $id, authorId: $author, content: $content) {
            comments
        }
    }
);

make_graphql!(
    mutation remove_comment($id: ScalarObjectId!, $comment_id: ScalarObjectId!) {
        removeComment(cvId: $id, commentId: $comment_id) {
            comments
        }
    }
);

make_graphql!(
    mutation share_cv($cv_id: ScalarObjectId!, $user_id: ScalarObjectId!) {
        shareCv(cvId: $cv_id, userId: $user_id)
    }
);

make_graphql!(
    mutation unshare_cv($cv_id: ScalarObjectId!, $user_id: ScalarObjectId!) {
        unshareCv(cvId: $cv_id, userId: $user_id)
    }
);

make_graphql!(
    mutation like_cv($cv_id: ScalarObjectId!, $user_id: ScalarObjectId!) {
        likeCv(cvId: $cv_id, userId: $user_id)
    }
);

make_graphql!(
    mutation unlike_cv($cv_id: ScalarObjectId!, $user_id: ScalarObjectId!) {
        unlikeCv(cvId: $cv_id, userId: $user_id)
    }
);

make_graphql!(
    mutation bookmark_cv($cv_id: ScalarObjectId!, $user_id: ScalarObjectId!) {
        bookmarkCv(cvId: $cv_id, userId: $user_id)
    }
);

make_graphql!(
    mutation unbookmark_cv($cv_id: ScalarObjectId!, $user_id: ScalarObjectId!) {
        unbookmarkCv(cvId: $cv_id, userId: $user_id)
    }
);

make_graphql!(
    query cvs_list($filter: Cvdetails!, $after: String, $before: String, $first: Int, $last: Int) {
        cvsList(filter: $filter, after: $after, before: $before, first: $first, last: $last) {
            edges {
                node {
                    id,
                    title,
                    description,
                }
                cursor
            }
            pageInfo {
                hasNextPage,
                hasPreviousPage,
                startCursor,
                endCursor
            }
        }
    }
);
