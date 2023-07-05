use async_graphql::*;

use crate::{
    data_source::mongo::MongoDB,
    models::users::User,
    services::{
        temp::temp_function,
        user_service::{self, UserService},
    },
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

    async fn user_detail(&self, ctx: &Context<'_>) -> Result<User> {
        let db = ctx.data_unchecked::<MongoDB>();
        let token = ctx.data_unchecked::<Option<String>>();
        let token = match token {
            Some(token) => token,
            None => return Err("No token provided".into()),
        };
        let rs = user_service::decode_token(token);
        let claims = match rs {
            Some(claims) => claims,
            None => return Err("Invalid token".into()),
        };
        let rs = UserService::get_user_by_username(db, claims.sub).await;
        match rs {
            Ok(user) => Ok(user),
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
