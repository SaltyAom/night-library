use serde::{Deserialize, Serialize};

use diesel::prelude::*;

use crate::schema::lends;
use crate::schema::lends::dsl::*;

use crate::libraries::model::{in_library, Librarie};

use juniper::GraphQLObject;

#[derive(Clone, Deserialize, Serialize, Queryable, Insertable, GraphQLObject)]
pub struct Lend {
    pub username: String,
    pub books: Vec<String>,
}

#[derive(Deserialize)]
pub struct LendForm {
    pub books: Vec<String>,
}

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct LendResultQuery {
    pub success: bool,
    pub info: String,
    pub data: Vec<String>,
}

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct LendDetailResultQuery {
    pub success: bool,
    pub info: String,
    pub data: Vec<Librarie>,
}

impl Lend {
    pub fn append(&self, connection: &PgConnection) -> Result<Option<Lend>, diesel::result::Error> {
        // SQL append array is not support
        let book_data = self.list(connection).unwrap();

        let mut new_list = self.books.to_owned();

        if book_data.is_some() {
            let book_list = book_data.unwrap().books;

            new_list.extend(book_list.iter().cloned());
        };

        let appended = Lend {
            username: self.username.to_owned(),
            books: new_list.to_owned(),
        };

        diesel::insert_into(lends)
            .values(&appended)
            .on_conflict(username)
            .do_update()
            .set(books.eq(new_list))
            .execute(connection)
            .expect("Borrow books");

        Ok(Some(appended))
    }

    pub fn remove(&self, connection: &PgConnection) -> Result<Option<Lend>, diesel::result::Error> {
        let my_library_data = self.list(connection).unwrap();

        if my_library_data.is_none() {
            return Ok(None);
        };

        let mut my_books = my_library_data.unwrap().clone().books;
        let return_books = self.books.to_owned();

        // Remove return books and retain what not to be removed.
        my_books.retain(|my_book| !return_books.contains(&my_book));

        diesel::update(lends.filter(username.eq(self.username.to_owned())))
            .set(books.eq(my_books.to_owned()))
            .execute(connection)
            .expect("Return books");

        Ok(Some(Lend {
            username: self.username.to_owned(),
            books: my_books,
        }))
    }

    pub fn list(&self, connection: &PgConnection) -> Result<Option<Lend>, diesel::result::Error> {
        let book_list = lends
            .filter(username.eq(self.username.to_owned()))
            .first::<Lend>(connection)
            .optional()?;

        Ok(book_list)
    }

    pub fn list_detail(
        &self,
        connection: &PgConnection,
    ) -> Result<Option<Vec<Librarie>>, diesel::result::Error> {
        let borrowed_list = lends
            .filter(username.eq(self.username.to_owned()))
            .first::<Lend>(connection);

        let borrowed_books: Vec<Librarie> = in_library(borrowed_list.unwrap().books, connection)?;

        if borrowed_books.len() > 0 {
            Ok(Some(borrowed_books))
        } else {
            Ok(None)
        }
    }
}
