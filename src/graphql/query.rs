use async_graphql as gql;
use async_graphql::{futures_util::StreamExt, Context, InputObject, Object};
use gql::{connection, ErrorExtensions};
use mongodb::bson::oid::ObjectId;
use mongodb::options::AuthMechanism;

use crate::error::ServerError;
use crate::object_id::ScalarObjectId;
use crate::{
    data_source::mongo::{MongoDB, MongoForTesting},
    models::users::User,
    services::{auth_service::AuthService, user_service::UserService},
};

use super::authorization;

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
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
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
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        let claims = authorization(ctx)?;
        let rs = UserService::get_user_by_username(db, claims.sub).await;
        match rs {
            Ok(user) => Ok(user),
            Err(e) => Err(e.extend()),
        }
    }

    async fn refresh_token(&self, ctx: &Context<'_>, refresh_token: String) -> gql::Result<String> {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        let rs = AuthService::generate_new_access_token(db, refresh_token).await;
        match rs {
            Ok(token) => Ok(token),
            Err(e) => Err(e.extend()),
        }
    }

    async fn friendslist(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> gql::Result<
        connection::Connection<
            ScalarObjectId,
            User,
            connection::EmptyFields,
            connection::EmptyFields,
        >,
    > {
        let db = ctx
            .data_opt::<MongoDB>()
            .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>());
        authorization(ctx)?;
        let friends_list = UserService::friend_lists(db, user_id)
            .await
            .collect::<Vec<_>>()
            .await;
        connection::query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let friends_list = if let Some(after) = after {
                    friends_list
                        .into_iter()
                        .skip_while(|friend| friend.as_ref().unwrap().id != after)
                        .skip(1)
                        .map(|friend| friend)
                        .collect::<Vec<_>>()
                } else if let Some(before) = before {
                    friends_list
                        .into_iter()
                        .take_while(|friend| friend.as_ref().unwrap().id != before)
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
                    let size = friends_list.len() as usize;
                    friends_list
                        .into_iter()
                        .skip(size - last as usize)
                        .collect::<Vec<_>>()
                } else {
                    panic!("Must have either 'first' or 'last' argument")
                };
                let mut connection = connection::Connection::new(true, false);
                connection
                    .edges
                    .extend(friends_list.into_iter().map(|friend| {
                        connection::Edge::new(friend.as_ref().unwrap().id, friend.unwrap())
                    }));
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}
