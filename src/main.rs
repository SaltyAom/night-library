#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

use std::io;
use std::env;
use std::sync::Arc;
use dotenv::dotenv;

use actix_web::{ HttpServer, App, web, middleware };
use actix_identity::{ IdentityService, CookieIdentityPolicy };
use actix_files as fs;

// ? Use in dev environment
// use actix_web::http;
// use actix_cors::Cors;

use crate::database::get_connection_pool;
use crate::graphql::create_schema;

mod routes;
mod libraries;
mod users;
mod schema;
mod lends;
mod graphql;
mod database;
mod bigdecimal;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let schema = Arc::new(create_schema());
    let pool = get_connection_pool();

    HttpServer::new(move || {
        App::new()
            // ? Use in dev environment
            // .wrap(
            //     Cors::new()
            //         .allowed_origin("http://localhost:8080")
            //         .allowed_methods(vec!["GET", "POST"])
            //         .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            //         .allowed_header(http::header::CONTENT_TYPE)
            //         .supports_credentials()
            //         .max_age(86400)
            //         .finish()
            // )
            .wrap(middleware::Compress::default())
            .wrap(
                IdentityService::new(
                    CookieIdentityPolicy::new(
                        env::var("cookie_secret").unwrap().as_bytes()
                    )                            
                    .name("auth")
                    .secure(false)
                )
            )
            .data(pool.clone())
            .data(schema.clone())
            .service(routes::common::graphql)
            .service(routes::common::graphiql)
            .configure(users::routes::user)
            .configure(routes::common::client)
            .service(fs::Files::new("/", "./static"))
            .default_service(web::route().to(routes::common::client_404))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}