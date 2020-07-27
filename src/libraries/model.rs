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
        let new_book = Librarie {
            id: Uuid::new_v4().to_string(),
            title: self.title.to_owned(),
            author: self.title.to_owned(),
            price: self.price.to_owned(),
        };

        diesel::insert_into(libraries)
            .values(&new_book)
            .execute(connection)?;

        Ok(new_book)
    }

    pub fn list(&self, connection: &PgConnection) -> Result<Vec<Librarie>, diesel::result::Error> {
        let books = libraries.load(connection);

        books
    }
}
