use async_graphql::*;
use mongodb::bson::Uuid;

use crate::{
    data_source::mongo::MongoDB,
    models::{cv::CV, users::User},
    services::temp::{temp_function, GqlResult},
};

pub struct Query;

#[Object]
impl Query {
    async fn placeholder_query(&self, ctx: &Context<'_>) -> Result<String> {
        let db = ctx.data_unchecked::<MongoDB>().db.clone();
        temp_function(db).await
    }

    async fn user_login(&self, ctx: &Context<'_>) -> GqlResult<String> {
        todo!()
    }

    // NOTE: Using graphql paging maybe good in this case
    async fn get_user_cvs(&self, ctx: &Context<'_>, user_id: Uuid) -> GqlResult<Vec<CV>> {
        todo!()
    }

    async fn get_user_by_username(&self, ctx: &Context<'_>, username: String) -> GqlResult<User> {
        todo!()
    }

    async fn get_user_photo(&self, ctx: &Context<'_>) -> GqlResult<String> {
        todo!()
    }
}
