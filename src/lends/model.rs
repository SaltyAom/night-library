use serde::{Deserialize, Serialize};

use diesel::prelude::*;

use crate::schema::lends;
use crate::schema::lends::dsl::*;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
pub struct Lend {
    pub username: String,
    pub books: Vec<String>,
}

#[derive(Deserialize)]
pub struct LendForm {
    pub book: Vec<String>,
}

impl Lend {
    pub fn append(&self, connection: &PgConnection) -> Result<Option<Lend>, diesel::result::Error> {
        // SQL append array is not support
        let book_data = self.list(connection).unwrap();

        let new_list = self.books.to_owned();

        if book_data.is_some() {
            let mut book_list = book_data.unwrap().books;

            // Remove return duplicate books.
            book_list.retain(|book| new_list.contains(&book));
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
}
