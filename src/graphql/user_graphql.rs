use async_graphql::{ComplexObject, Context};

use crate::{models::users::User, services::temp::GqlResult};

#[ComplexObject]
impl User {
    async fn avatar(&self, ctx: &Context<'_>) -> GqlResult<String> {
        // TODO: Request signed url from storage service and
        // return the url
        todo!()
    }

    async fn cover_photo(&self, ctx: &Context<'_>) -> GqlResult<String> {
        // TODO: Request signed url from storage service and
        // return the url
        todo!()
    }
}
