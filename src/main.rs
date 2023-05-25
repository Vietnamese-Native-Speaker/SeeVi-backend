use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_std::task;
use data_source::mongo;
use tide::*;

use crate::graphql::{
    graphql,
    query::Query,
};
// use tide::prelude::*;
mod graphql;
mod services;
mod data_source;


#[derive(Clone)]
pub struct State {
    pub schema: Schema<Query, EmptyMutation, EmptySubscription>,
}

fn main() -> Result<()> {
    task::block_on(run())
}

async fn run() -> Result<()> {
    let mongo_ds = mongo::DataSource::init().await;

    let schema = Schema::build(
        Query,
        EmptyMutation,
        EmptySubscription
    )
    .data(mongo_ds)
    .finish();

    let app_state = State { schema };
    let mut app = tide::with_state(app_state);

    app.at("/graphql").post(graphql);

    app.at("/").get(|_| async move {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_body(Body::from_string(
            GraphiQLSource::build().endpoint("/graphql").finish(),
        ));
        resp.set_content_type(http::mime::HTML);
        Ok(resp)
    });
    app.listen("localhost:8080").await?;
    Ok(())
}