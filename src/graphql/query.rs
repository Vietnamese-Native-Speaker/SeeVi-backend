use async_graphql as gql;
use async_graphql::{Context, InputObject, Object};
use gql::{connection, ErrorExtensions};

use crate::error::ServerError;
use crate::{
    data_source::friends_list_datasource::FriendsListDataSource,
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

    async fn friendslist(
        &self,
        ctx: &Context<'_>,
        UserId: String,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> gql::Result<
        connection::Connection<String, User, connection::EmptyFields, connection::EmptyFields>,
    > {
        let db = ctx.data_unchecked::<MongoDB>();
        let friends_list = db.get_friends_list(UserId).await.collect::<Vec<_>>();
        connection::query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let friends_list = if let Some(after) = after {
                    friends_list
                        .into_iter()
                        .skip_while(|friend| friend.as_ref().unwrap()._id != after)
                        .skip(1)
                        .map(|friend| friend)
                        .collect::<Vec<_>>()
                } else if let Some(before) = before {
                    friends_list
                        .into_iter()
                        .take_while(|friend| friend.as_ref().unwrap()._id != before)
                        .map(|friend| friend)
                        .collect::<Vec<_>>()
                } else {
                    friends_list.into_iter().collect::<Vec<_>>()
                };
                let friends_list = if let Some(first) = first {
                    friends_list
                        .into_iter()
                        .take(first as usize)
                        .collect::<Vec<_>>()
                } else if let Some(last) = last {
                    friends_list
                        .into_iter()
                        .skip(friends_list.len() - last as usize)
                        .collect::<Vec<_>>()
                } else {
                    panic!("Must have either 'first' or 'last' argument")
                };
                let mut connection = connection::Connection::new(true, false);
                connection.edges.extend(
                    friends_list
                        .into_iter()
                        .map(|friend| connection::Edge::new(friend.unwrap(), None)),
                );
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}
