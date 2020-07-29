use std::sync::Arc;

use actix_identity::Identity;
use actix_web::{get, post, web, Error, HttpResponse};

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::database::get_connection_pool;
use crate::graphql::{Metadata, Schema};
use crate::users::jwt::decode;

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


lazy_static! {
    static ref LANDING_PAGE: &'static str = include_str!("../../static/index.html");
    static ref ME_PAGE: &'static str = include_str!("../../static/me/index.html");
    static ref MY_BOOK_PAGE: &'static str = include_str!("../../static/my-book/index.html");
    static ref SIGNIN_PAGE: &'static str = include_str!("../../static/signin/index.html");
    static ref SIGNUP_PAGE: &'static str = include_str!("../../static/signup/index.html");
    static ref NOT_FOUND_PAGE: &'static str = include_str!("../../static/404/index.html");
}

#[get("/")]
pub async fn client_landing() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            LANDING_PAGE.to_owned()
        )
}

#[get("/me")]
pub async fn client_me() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            ME_PAGE.to_owned()
        )
}

#[get("/my-book")]
pub async fn client_my_book() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            MY_BOOK_PAGE.to_owned()
        )
}

#[get("/signin")]
pub async fn client_signin() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            SIGNIN_PAGE.to_owned()
        )
}

#[get("/signup")]
pub async fn client_signup() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            SIGNUP_PAGE.to_owned()
        )
}

pub async fn client_404() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            NOT_FOUND_PAGE.to_owned()
        )
}

pub fn client(config: &mut web::ServiceConfig) {
    config
        .service(client_landing)
        .service(client_me)
        .service(client_my_book)
        .service(client_signin)
        .service(client_signup);
}