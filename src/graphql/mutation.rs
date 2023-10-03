use async_graphql::{Context, ErrorExtensions, Object};

use crate::{
    data_source::mongo::{MongoDB, MongoForTesting},
    models::users::{CreateUserInput, User},
    object_id::ScalarObjectId,
    services::{auth_service::AuthService, user_service::UserService},
};

use super::{authorization, GqlResult};

pub struct Mutation;

#[Object]
impl Mutation {
    async fn user_register(&self, ctx: &Context<'_>, new_user: CreateUserInput) -> GqlResult<User> {
        let rs = AuthService::register(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            new_user,
        )
        .await;
        match rs {
            Ok(user) => Ok(user),
            Err(e) => Err(e.extend()),
        }
    }

    async fn send_friend_request(
        &self,
        ctx: &Context<'_>,
        user_id: ScalarObjectId,
        friend_id: ScalarObjectId,
        message: Option<String>,
    ) -> GqlResult<bool> {
        let rs = UserService::send_friend_request(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            user_id.into(),
            friend_id.into(),
            message,
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn accept_friend_request(
        &self,
        ctx: &Context<'_>,
        user_id: ScalarObjectId,
        friend_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let rs = UserService::accept_friend_request(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            user_id.into(),
            friend_id.into(),
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }

    async fn decline_friend_request(
        &self,
        ctx: &Context<'_>,
        user_id: ScalarObjectId,
        friend_id: ScalarObjectId,
    ) -> GqlResult<bool> {
        let rs = UserService::reject_friend_request(
            ctx.data_opt::<MongoDB>()
                .unwrap_or_else(|| ctx.data_unchecked::<MongoForTesting>()),
            user_id.into(),
            friend_id.into(),
        )
        .await;
        authorization(ctx)?;
        match rs {
            Ok(_) => Ok(true),
            Err(e) => Err(e.into()),
        }
    }
}
