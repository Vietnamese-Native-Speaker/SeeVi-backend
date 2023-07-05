use async_graphql::*;

use crate::{
    data_source::mongo::MongoDB,
    models::users::User,
    services::{user_service::UserService, auth_service::AuthService},
};

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
            AuthService::authenticate(db, Some(login_info.username), None, login_info.password)
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
        let rs = AuthService::decode_token(token);
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
}
