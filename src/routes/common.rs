use std::sync::Arc;

use actix_identity::Identity;
use actix_web::{get, post, web, Error, HttpResponse};

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::database::get_connection_pool;
use crate::graphql::{Metadata, Schema};
use crate::users::jwt::decode;

#[get("/")]
pub async fn index() -> String {
    "Hello World".to_owned()
}

#[get("/graphiql")]
pub async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("http://127.0.0.1:80/graphql"))
}

#[post("/graphql")]
pub async fn graphql(
    data: web::Data<Arc<Schema>>,
    request: web::Json<GraphQLRequest>,
    user: Identity,
) -> Result<HttpResponse, Error> {
    let connection_pool = get_connection_pool();

    let username: Option<String> = if let Some(identity) = user.identity() {
        Some(decode(&identity).unwrap().name)
    } else {
        None
    };

    let context = Metadata {
        username: username,
        connection_pool: connection_pool,
    };

    let result = web::block(move || {
        let graphql_response = request.execute(&data, &context);

        Ok::<_, serde_json::error::Error>(serde_json::to_string(&graphql_response)?)
    })
    .await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result))
}
