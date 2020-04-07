#[macro_use] extern crate juniper;

mod api;

use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use futures::Future;
use crate::api::root_node;

async fn handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, world!".into()))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let root = root_node();
    let ctx = Arc::new(());

    let make_svc = make_service_fn(move |_conn| {
        let root = root.clone();
        let ctx = ctx.clone();

        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| -> Box<dyn Future<Item = _, Error = _, Output = _> + Send> {
                let root = root.clone();
                let ctx = ctx.clone();
                async move {
                    match(req.method(), req.uri().path()) {
                        (&Method::GET, "/api") | (&Method::POST, "/api") => {
                            Box::new(juniper_hyper::graphql(root, ctx, req))
                        }
                        _ => {
                            let mut response = Response::new(Body::empty());
                            *response.status_mut() = StatusCode::NOT_FOUND;
                            Box::new(Ok(response))
                        }
                    }
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
