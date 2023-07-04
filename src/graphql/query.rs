use async_graphql::*;

use crate::{
    data_source::mongo::MongoDB,
    services::{temp::temp_function, user_service::UserService},
};

use log::info;

pub struct Query;

#[derive(InputObject)]
struct LoginInfo {
    username: String,
    password: String,
}

#[Object]
impl Query {
    async fn login(&self, ctx: &Context<'_>, login_info: LoginInfo) -> Result<String> {
        let db = ctx.data_unchecked::<MongoDB>();
        let rs =
            UserService::authenticate(db, Some(login_info.username), None, login_info.password)
                .await;
        match rs {
            Ok(token) => Ok(token),
            Err(e) => Err(e.into()),
        }
    }

    async fn placeholder_query(&self, ctx: &Context<'_>) -> Result<String> {
        let auth = ctx.data::<String>();
        match auth {
            Ok(auth) => println!("Auth: {}", auth),
            Err(e) => info!("Error: {:?}", e),
        }
        let db = ctx.data_unchecked::<MongoDB>().db.clone();
        temp_function(db).await
    }
}
