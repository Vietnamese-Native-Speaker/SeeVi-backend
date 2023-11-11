use graphql_query_maker::make_graphql;

make_graphql!(
    query login($info: LoginInfo!) {
        login(loginInfo: $info) {
            accessToken,
            refreshToken,
        },
        detail(id: 1) {
            id,
            name,
        }
    }
);

make_graphql!(
    query userDetail {
        detail(id: 1) {
            id,
            name,
        }
    }
);

fn main() {
    let s = query_login(serde_json::json!({ "user" : { "name": "test" }}));
    let s2 = query_user_detail();
    println!("{:#?}", s);
}
