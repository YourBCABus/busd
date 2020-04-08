#[macro_use] extern crate juniper;

mod api;

use warp::{http::Response, Filter};
use juniper_warp::{graphiql_filter, make_graphql_filter};
use crate::api::root_node;

#[tokio::main]
async fn main() {
    let state = warp::any().map(|| ());
    let graphql_filter = make_graphql_filter(root_node(), state.boxed());

    warp::serve(
        warp::path::end().map(|| {
            Response::builder().body(format!("Hello, world!"))
        }).or(warp::get().and(warp::path("graphiql")).and(graphiql_filter("/api")))
          .or(warp::path("graphql").and(graphql_filter))
    ).run(([127, 0, 0, 1], 3000)).await
}
