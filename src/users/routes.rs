use actix_identity::Identity;
use actix_web::{get, post, web, Error, HttpResponse};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::users::model::{User, UserForm};
use crate::users::hash::hash;
use crate::users::time::get_expire_time;
use crate::users::jwt::{ encode, decode };

type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/login")]
pub async fn login(
    form: web::Form<UserForm>,
    connection_pool: web::Data<ConnectionPool>,
    identity: Identity,
) -> Result<HttpResponse, Error> {
    if identity.identity().is_some() {
        return Ok(HttpResponse::BadRequest().body("Already logged in".to_owned()));
    }

    let connection = connection_pool.get().expect("Connection Pool");

    let user = web::block(move || {
        let user = User {
            username: form.username.to_owned(),
            password: hash(&form.password).to_owned(),
        };

        user.login(&connection)
    })
    .await
    .map_err(|error| {
        eprintln!("{}", error);
        HttpResponse::BadRequest().body(format!("{}", error))
    })?;

    if let Some(user) = user {
        let auth_token = encode(&user.username);

        identity.remember(auth_token.unwrap());

        Ok(HttpResponse::Ok().body(format!("Logged in as {}", user.username.to_owned())))
    } else {
        Ok(HttpResponse::Unauthorized().body("Username or password is incorrect".to_owned()))
    }
}

#[post("/register")]
pub async fn register(
    form: web::Form<UserForm>,
    connection_pool: web::Data<ConnectionPool>,
) -> Result<HttpResponse, Error> {
    let connection = connection_pool.get().expect("Connection Pool");

    let user = web::block(move || {
        let user = User {
            username: form.username.to_owned(),
            password: hash(&form.password).to_owned(),
        };

        user.register(&connection)
    })
    .await
    .map_err(|error| {
        eprintln!("{}", error);
        HttpResponse::BadRequest().body(format!("{}", error))
    })?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/logout")]
pub async fn logout(user: Identity) -> String {
    user.forget();

    "Logged out".to_owned()
}

#[post("/api/refresh")]
pub async fn refresh(user: Identity) -> HttpResponse {
    if user.identity().is_none() {
        return HttpResponse::BadRequest().finish()
    };

    let auth = decode(&user.identity().unwrap());

    if let Some(token) = auth {
        if token.exp < get_expire_time() {
            user.remember(encode(&token.name).unwrap());

            HttpResponse::Ok().body("Refreshed".to_owned())
        } else {
            user.forget();

            HttpResponse::BadRequest().finish()
        }

    } else {
        user.forget();

        HttpResponse::BadRequest().finish()
    }
}

pub fn user(config: &mut web::ServiceConfig) {
    config.service(login).service(register).service(logout).service(refresh);
}