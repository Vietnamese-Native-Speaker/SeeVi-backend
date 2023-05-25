use async_graphql::*;

use crate::{services::temp::temp_function, data_source::mongo::DataSource};

pub struct Query;

#[Object]
impl Query {
    async fn placeholder_query(&self, ctx: &Context<'_>) -> Result<String> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        temp_function(db).await
    }
}
