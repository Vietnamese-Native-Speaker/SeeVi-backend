pub mod query;

use async_graphql::http::receive_json;
use tide::*;
use crate::State;

pub async fn graphql(req: Request<State>) -> tide::Result {
    let schema = req.state().schema.clone();
    let gql_resp = schema.execute(receive_json(req).await?).await;

    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body(Body::from_json(&gql_resp)?);

    Ok(resp.into())
}
