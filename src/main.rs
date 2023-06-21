use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use seevi_backend::{data_source::mongo, graphql::mutation::Mutation};
use tide::*;

use seevi_backend::graphql::{graphql, query::Query};
use tide::prelude::*;

use seevi_backend::State;

#[async_std::main]
async fn main() -> Result<()> {
    run().await
}

async fn run() -> Result<()> {
    let mongo_ds = mongo::MongoDB::init().await;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
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

