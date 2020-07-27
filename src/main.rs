#[macro_use]
extern crate diesel;

use std::io;
use std::env;
use std::sync::Arc;
use dotenv::dotenv;

use actix_web::{ HttpServer, App };
use actix_identity::{ IdentityService, CookieIdentityPolicy };

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
            .service(routes::common::index)
            .service(routes::common::graphql)
            .service(routes::common::graphiql)
            .configure(users::routes::user)
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}