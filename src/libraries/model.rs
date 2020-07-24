use std::str::FromStr;

use serde::{Deserialize, Serialize};

use diesel::prelude::*;

use crate::schema::libraries;
use crate::schema::libraries::dsl::*;

use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
pub struct Librarie {
    pub id: String,
    pub title: String,
    pub author: String,
    pub price: BigDecimal,
}

#[derive(Deserialize)]
pub struct LibrarieForm {
    pub title: String,
    pub author: String,
    pub price: BigDecimal,
}

impl Default for Librarie {
    fn default() -> Librarie {
        Librarie {
            id: Uuid::new_v4().to_string(),
            title: "".to_owned(),
            author: "".to_owned(),
            price: BigDecimal::from_str(&"0").unwrap(),
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
