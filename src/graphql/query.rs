use async_graphql::*;
use async_graphql::{connection::OpaqueCursor, futures_util::StreamExt};

use crate::data_source::cv_data_source_error::CVDataSourceError;
use crate::{
    data_source::{cv_data_source::CVDataSource, mongo::MongoDB},
    models::{cv::CV, users::User},
    services::{auth_service::AuthService, user_service::UserService},
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

    async fn recommended_cvs(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<connection::Connection<String, CV, connection::EmptyFields, connection::EmptyFields>>
    {
        let db = ctx.data_unchecked::<MongoDB>();
        let cvs = db.get_recommended_cvs().await.collect::<Vec<_>>().await;
        connection::query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                log::info!(
                    "Get connection with: after: {:?}, before: {:?}, first: {:?}, last: {:?}",
                    after,
                    before,
                    first,
                    last
                );
                let cvs = if let Some(after) = after {
                    cvs.into_iter()
                        .skip_while(|cv| cv.as_ref().unwrap()._id.to_string() != after)
                        .skip(1)
                        .map(|cv| cv.unwrap())
                        .collect::<Vec<_>>()
                } else if let Some(before) = before {
                    cvs.into_iter()
                        .take_while(|cv| cv.as_ref().unwrap()._id.to_string() != before)
                        .map(|cv| cv.unwrap())
                        .collect::<Vec<_>>()
                } else {
                    cvs.into_iter().map(|cv| cv.unwrap()).collect::<Vec<_>>()
                };
                let cvs = if let Some(first) = first {
                    cvs.into_iter().take(first as usize).collect::<Vec<_>>()
                } else if let Some(last) = last {
                    let len = cvs.len();
                    if len > last as usize {
                        cvs.into_iter()
                            .skip(len - last as usize)
                            .collect::<Vec<_>>()
                    } else {
                        cvs
                    }
                } else {
                    panic!("Must provide either 'first' or 'last' argument.")
                };
                let mut connection = connection::Connection::new(true, false);
                connection.edges.extend(
                    cvs.into_iter()
                        .map(|cv| connection::Edge::new(cv._id.to_string(), cv)),
                );
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}
