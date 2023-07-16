use async_graphql as gql;
use async_graphql::{Context, InputObject, Object};
use gql::ErrorExtensions;

use crate::error::ServerError;
use crate::{
    data_source::mongo::MongoDB,
    models::users::User,
    services::{auth_service::AuthService, user_service::UserService},
};

pub struct Query;

#[derive(InputObject)]
struct LoginInfo {
    username: String,
    password: String,
}

#[derive(gql::SimpleObject)]
struct LoginResult {
    access_token: String,
    refresh_token: String,
}

#[Object]
impl Query {
    async fn login(&self, ctx: &Context<'_>, login_info: LoginInfo) -> gql::Result<LoginResult> {
        let db = ctx.data_unchecked::<MongoDB>();
        let rs =
            AuthService::authenticate(db, Some(login_info.username), None, login_info.password)
                .await;
        match rs {
            Ok(token) => Ok(LoginResult {
                access_token: token.0,
                refresh_token: token.1,
            }),
            Err(e) => Err(e.extend()),
        }
    }

    async fn user_detail(&self, ctx: &Context<'_>) -> gql::Result<User> {
        let db = ctx.data_unchecked::<MongoDB>();
        let token = ctx.data_unchecked::<Option<String>>();
        let token = match token {
            Some(token) => token,
            None => return Err(ServerError::Unauthorized.extend()),
        };
        let rs = AuthService::decode_token(token, true);
        let claims = match rs {
            Some(claims) => claims,
            None => return Err(ServerError::InvalidToken.extend()),
        };
        let rs = UserService::get_user_by_username(db, claims.sub).await;
        match rs {
            Ok(user) => Ok(user),
            Err(e) => Err(e.extend()),
        }
    }

    async fn refresh_token(&self, ctx: &Context<'_>, refresh_token: String) -> gql::Result<String> {
        let db = ctx.data_unchecked::<MongoDB>();
        let rs = AuthService::generate_new_access_token(db, refresh_token).await;
        match rs {
            Ok(token) => Ok(token),
            Err(e) => Err(e.extend()),
        }
    }
}
