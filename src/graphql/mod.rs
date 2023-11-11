use async_graphql::ErrorExtensions;

use crate::{
    error::ServerError,
    services::auth_service::{AuthService, Claims},
};

pub mod mutation;
pub mod query;
mod cv;
mod user;

pub type GqlResult<T> = Result<T, async_graphql::Error>;

fn authorization(ctx: &async_graphql::Context<'_>) -> GqlResult<Claims> {
    let token = ctx.data_unchecked::<Option<String>>();
    let token = token.as_ref().ok_or_else(|| ServerError::Unauthorized.extend())?;
    let rs = AuthService::decode_token(token, true);
    rs.ok_or_else(|| ServerError::InvalidToken.extend())
}
