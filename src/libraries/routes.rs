use actix_web::{get, post, web, Error, HttpResponse};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::libraries::model::{Librarie, LibrarieForm};

type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/library/add")]
pub async fn add_book(
    book: web::Form<LibrarieForm>,
    connection_pool: web::Data<ConnectionPool>,
) -> Result<HttpResponse, Error> {
    let connection = connection_pool.get().expect("Connection Pool");

    let request = web::block(move || {
        let library = Librarie {
            title: book.title.to_owned(),
            author: book.author.to_owned(),
            price: book.price.to_owned(),
            ..Default::default()
        };

        library.add(&connection)
    })
    .await
    .map_err(|error| {
        eprintln!("{}", error);
        HttpResponse::BadRequest().finish()
    })?;

    println!("Added");
    Ok(HttpResponse::Ok().json(request))
}

#[get("/library/list")]
pub async fn list_book(connection_pool: web::Data<ConnectionPool>) -> Result<HttpResponse, Error> {
    let connection = connection_pool.get().expect("Connection Pool");

    let request = web::block(move || {
        Librarie {
            ..Default::default()
        }
        .list(&connection)
    })
    .await
    .map_err(|error| {
        eprintln!("{}", error);
        HttpResponse::BadRequest().body(format!("{}", error))
    })?;

    Ok(HttpResponse::Ok().json(request))
}

pub fn book(config: &mut web::ServiceConfig) {
    config.service(add_book).service(list_book);
}
