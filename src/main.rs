#[macro_use]
extern crate diesel;

use std::io;
use std::env;
use dotenv::dotenv;

use actix_web::{ HttpServer, App };
use actix_identity::{ IdentityService, CookieIdentityPolicy };

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod routes;
mod libraries;
mod users;
mod schema;
mod lends;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    
    let connection_url = std::env::var("DATABASE_URL").expect("Database Url");
    let connection_manager = ConnectionManager::<PgConnection>::new(connection_url);
    let pool = r2d2::Pool::builder()
        .build(connection_manager)
        .expect("Connection Pool");

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
            .service(routes::common::index)
            .configure(libraries::routes::book)
            .configure(users::routes::user)
            .configure(lends::routes::lend_service)
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}