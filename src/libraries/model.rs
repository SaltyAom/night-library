use diesel::prelude::*;

use crate::schema::libraries;
use crate::schema::libraries::dsl::*;

use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use crate::bigdecimal::MyBigDecimal;
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, Queryable, Insertable, GraphQLObject)]
pub struct Librarie {
    pub id: String,
    pub title: String,
    pub author: String,
    pub price: MyBigDecimal,
}

#[derive(Deserialize)]
pub struct LibrarieForm {
    pub title: String,
    pub author: String,
    pub price: MyBigDecimal,
}

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct ListBookResultQuery {
    pub success: bool,
    pub info: String,
    pub data: Vec<Librarie>,
}

impl Default for Librarie {
    fn default() -> Librarie {
        Librarie {
            id: Uuid::new_v4().to_string(),
            title: "".to_owned(),
            author: "".to_owned(),
            price: MyBigDecimal::from_str(&"0"),
        }
    }
}

impl Librarie {
    pub fn add(&self, connection: &PgConnection) -> Result<Librarie, diesel::result::Error> {
        diesel::insert_into(libraries)
            .values(self)
            .execute(connection)?;

        Ok(self.to_owned())
    }

    pub fn list(&self, connection: &PgConnection) -> Result<Vec<Librarie>, diesel::result::Error> {
        libraries.load(connection)
    }

    pub fn remove(&self, connection: &PgConnection) -> Result<Librarie, diesel::result::Error> {
        diesel::delete(libraries.filter(id.eq(self.id.to_owned()))).execute(connection)?;

        Ok(self.to_owned())
    }
}

pub fn in_library(
    books: Vec<String>,
    connection: &PgConnection,
) -> Result<Vec<Librarie>, diesel::result::Error> {
    let in_books = libraries.filter(id.eq_any(books)).load(connection);

    in_books
}
