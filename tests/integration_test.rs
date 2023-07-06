mod common;

use async_graphql::{EmptySubscription, Schema};
use seevi_backend::graphql::query::Query;
use seevi_backend::{data_source::mongo, graphql::mutation::Mutation};

use crate::common::{
    default_route, make_login_request, make_register_request, print_json, user_detail,
};

#[tokio::test]
async fn register_and_login() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let mongo_ds = mongo::MongoDB::init_test().await;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(mongo_ds)
        .finish();
    let routes = default_route(schema);

    make_register_request("ltp", "ltp", &routes).await;
    let token = make_login_request("ltp", "ltp", &routes).await;
    print_json(&token);
    let token = token
        .get("data")
        .unwrap()
        .get("login")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let user_rs = user_detail(token, &routes).await;
    print_json(&user_rs);
    let user_rs = user_rs.get("data").unwrap().get("userDetail").unwrap();
    assert_eq!(user_rs.get("username").unwrap().as_str().unwrap(), "ltp");

    let token = make_login_request("ltp", "ltp1405", &routes).await;
    print_json(&token);
    token
        .get("errors")
        .expect("should have error due to wrong password");
}
