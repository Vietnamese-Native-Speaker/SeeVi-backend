mod common;

use async_graphql::{EmptySubscription, Schema};
use seevi_backend::graphql::query::Query;
use seevi_backend::{data_source::mongo, graphql::mutation::Mutation};

use crate::common::{
    default_route, make_login_request, make_register_request, print_json, user_detail,
};

#[serial_test::serial]
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
    let login_result = make_login_request("ltp", "ltp", &routes).await;
    let access_token = login_result
        .get("data")
        .unwrap()
        .get("access")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    
    let refresh_token = login_result
        .get("data")
        .unwrap()
        .get("refresh")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    print_json(&refresh_token);
    
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
