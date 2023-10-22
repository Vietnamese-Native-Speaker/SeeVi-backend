use graphql_query_maker::make_graphql;

make_graphql!(
query getUser() {
    userDetail {
        id,
        username
    }
}
);

make_graphql!(
mutation userRegister($user: CreateUserInput!) {
    userRegister(newUser: $user) {
        id,
        username
    }
}
);

make_graphql!(
query refreshToken($token: String!) {
    refreshToken(refreshToken: $token)
}
);

make_graphql!(
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
}
);

#[test]
fn test_make_graphql() {
    query_refresh_token(serde_json::Value::Null);
    query_get_user();
    mutation_user_register(serde_json::json!({
        "user": {
            "username": "test",
            "password": "test"
        }
    }));
    query_friendslist(
        Option::<()>::None,
        Option::<()>::None,
        Option::<()>::None,
        Some(10),
    );
}
