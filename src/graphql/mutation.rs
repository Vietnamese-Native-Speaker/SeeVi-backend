use async_graphql::{Context, Object};

use crate::{
    data_source::mongo::MongoDB,
    models::users::{CreateUserInput, User},
    services::{temp::GqlResult, user_service::UserService},
};

pub struct Mutation;

#[Object]
impl Mutation {
    async fn user_register(&self, ctx: &Context<'_>, new_user: CreateUserInput) -> GqlResult<User> {
        todo!()
    }
}
