use async_graphql::*;

use crate::{services::temp::temp_function, data_source::mongo::MongoDB};

use log::info;

pub struct Query;

#[Object]
impl Query {
    async fn placeholder_query(&self, ctx: &Context<'_>) -> Result<String> {
        let auth = ctx.data::<String>();
        match auth {
            Ok(auth) => println!("Auth: {}", auth),
            Err(e) => info!("Error: {:?}", e),
        }
        let db = ctx.data_unchecked::<MongoDB>().db.clone();
        temp_function(db).await
    }
}
