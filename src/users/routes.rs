use actix_identity::Identity;
use actix_web::{get, post, web, Error, HttpResponse};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::graphql::ResultQuery;
use crate::users::hash::hash;
use crate::users::jwt::{decode, encode};
use crate::users::model::{User, UserForm};
use crate::users::time::get_expire_time;

type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/login")]
pub async fn login(
    form: web::Form<UserForm>,
    connection_pool: web::Data<ConnectionPool>,
    identity: Identity,
) -> Result<HttpResponse, Error> {
    if identity.identity().is_some() {
        return Ok(HttpResponse::BadRequest().json(ResultQuery {
            success: false,
            info: "Already signin in".to_owned(),
            data: "".to_owned(),
        }));
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

        Ok(HttpResponse::Ok().json(ResultQuery {
            success: true,
            info: format!("Logged in as {}", user.username),
            data: "".to_owned(),
        }))
    } else {
        Ok(HttpResponse::Ok().json(ResultQuery {
            success: false,
            info: format!("Username or Password is incorrect"),
            data: "".to_owned(),
        }))
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
        return HttpResponse::BadRequest().finish();
    };

    let auth = decode(&user.identity().unwrap());

    if let Some(token) = auth {
        if token.exp < get_expire_time() {
            user.remember(encode(&token.name).unwrap());

            HttpResponse::Ok().body(token.name.to_owned())
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
    config
        .service(login)
        .service(register)
        .service(logout)
        .service(refresh);
}
