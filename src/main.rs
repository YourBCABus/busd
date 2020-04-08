#[macro_use] extern crate juniper;

mod api;

use actix_web::{HttpServer, App, web, HttpResponse, get, post, Error};
use actix_cors::Cors;
use std::sync::Arc;
use std::io;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use crate::api::{schema, Schema};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to busd!")
}

#[post("/api")]
async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    }).await?;
    Ok(HttpResponse::Ok().content_type("application/json").body(user))
}

#[get("/graphiql")]
async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:3000/api");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let schema = Arc::new(schema());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(index)
            .wrap(Cors::new().supports_credentials().finish())
            .service(graphql)
            .service(graphiql)
    }).bind("127.0.0.1:3000")?.run().await
}
