use crate::{
    data_source::mongo::MongoDB,
    models::{
        cv::{CreateCVInput, CV},
        users::CreateUserInput,
    },
    services::temp::{temp_function, GqlResult},
};
use async_graphql::Context;
use async_graphql::Object;
use mongodb::bson::Uuid;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn user_register(
        &self,
        ctx: &Context<'_>,
        new_user: CreateUserInput,
    ) -> GqlResult<String> {
        todo!()
    }

    async fn user_change_password(&self, ctx: &Context<'_>) -> GqlResult<String> {
        todo!()
    }

    async fn cv_create(
        &self,
        ctx: &Context<'_>,
        user_id: Uuid,
        cv: CreateCVInput,
    ) -> GqlResult<String> {
        todo!()
    }
}
