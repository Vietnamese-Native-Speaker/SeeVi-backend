use async_graphql::{Context, Object};

use crate::{
    data_source::mongo::MongoDB,
    models::users::{CreateUserInput, User},
    services::{auth_service::AuthService, user_service::UserService},
};

use super::GqlResult;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn user_register(&self, ctx: &Context<'_>, new_user: CreateUserInput) -> GqlResult<User> {
        let rs = AuthService::register(ctx.data_unchecked::<MongoDB>(), new_user).await;
        match rs {
            Ok(user) => Ok(user),
            Err(e) => Err(e.into()),
        }
    }
}
