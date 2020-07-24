use actix_identity::Identity;
use actix_web::{post, web, Error, HttpResponse};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::lends::model::{Lend, LendForm};
use crate::users::jwt::decode;

type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/library/borrow")]
pub async fn lend(
    user: Identity,
    lend_form: web::Json<LendForm>,
    connection_pool: web::Data<ConnectionPool>,
) -> Result<HttpResponse, Error> {
    if user.identity().is_none() {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized".to_owned()));
    }

    let connection = connection_pool.get().expect("Connection Pool");

    let to_be_lend = Lend {
        username: decode(&user.identity().unwrap()).unwrap().name,
        books: lend_form.book.to_owned(),
    };

    let appended = web::block(move || {    
        to_be_lend.append(&connection)
    })
    .await
    .map_err(|error| {
        eprintln!("{}", error);
        HttpResponse::BadRequest().body(format!("{}", error))
    })?;

    if let Some(book) = appended {
        Ok(HttpResponse::Ok().json(book))
    } else {
        Ok(HttpResponse::BadRequest().finish())
    }
}

#[post("/library/my-book")]
pub async fn list(user: Identity, connection_pool: web::Data<ConnectionPool>) -> Result<HttpResponse, Error> {
    if user.identity().is_none() {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized".to_owned()));
    }

    let connection = connection_pool.get().expect("Connection Pool");

    let books = Lend {
        username: decode(&user.identity().unwrap()).unwrap().name,
        books: vec![],
    };

    let borrowed = web::block(move || {        
        books.list(&connection)
    })
    .await
    .map_err(|error| {
        eprintln!("{}", error);
        HttpResponse::BadRequest().body(format!("{}", error))
    })?;

    if let Some(book) = borrowed {
        Ok(HttpResponse::Ok().json(book))
    } else {
        Ok(HttpResponse::Ok().body("[]".to_owned()))
    }
}

#[post("/library/return")]
pub async fn return_book(
    user: Identity,
    lend_form: web::Json<LendForm>,
    connection_pool: web::Data<ConnectionPool>,
) -> Result<HttpResponse, Error> {
    if user.identity().is_none() {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized".to_owned()));
    }

    let connection = connection_pool.get().expect("Connection Pool");

    let to_be_lend = Lend {
        username: decode(&user.identity().unwrap()).unwrap().name,
        books: lend_form.book.to_owned(),
    };

    let returned = web::block(move || {            
        to_be_lend.remove(&connection)
    })
    .await
    .map_err(|error| {
        eprintln!("{}", error);
        HttpResponse::BadRequest().body(format!("{}", error))
    })?;

    if let Some(book) = returned {
        Ok(HttpResponse::Ok().json(book))
    } else {
        Ok(HttpResponse::BadRequest().finish())
    }
}

pub fn lend_service(config: &mut web::ServiceConfig) {
    config.service(lend).service(list).service(return_book);
}
