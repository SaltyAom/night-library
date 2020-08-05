use juniper::{Context, FieldResult, GraphQLObject, RootNode};

use crate::bigdecimal::MyBigDecimal;
use crate::database::ConnectionPool;

use crate::lends::model::{LendDetailResultQuery, LendForm, LendResultQuery};
use crate::lends::services::{borrow_books_service, list_borrowed_service, return_books_service};

use crate::libraries::model::{Librarie, ListBookResultQuery};
use crate::libraries::services::{add_book_service, list_book_service, remove_book_service};

#[derive(Clone)]
pub struct Metadata {
    pub username: Option<String>,
    pub connection_pool: ConnectionPool,
}

impl Context for Metadata {}

pub struct QueryRoot {}

#[juniper::object(
    Context = Metadata
)]
impl QueryRoot {
    fn list_book(context: &Metadata) -> FieldResult<ListBookResultQuery> {
        Ok(list_book_service(context.connection_pool.to_owned()))
    }

    fn list_borrowed(context: &Metadata) -> FieldResult<LendDetailResultQuery> {
        Ok(list_borrowed_service(
            &context.username,
            context.connection_pool.to_owned(),
        ))
    }
}

pub struct Mutation;

#[juniper::object(
    Context = Metadata
)]
impl Mutation {
    fn add_book(
        context: &Metadata,
        title: String,
        author: String,
        price: i32,
    ) -> FieldResult<ResultQuery> {
        let book_price: String = price.to_string();

        let book = &Librarie {
            title: title,
            author: author,
            price: MyBigDecimal::from_str(&book_price),
            ..Default::default()
        };

        Ok(add_book_service(book, context.connection_pool.to_owned()))
    }

    fn remove_book(context: &Metadata, id: String) -> FieldResult<ResultQuery> {
        Ok(remove_book_service(&id, context.connection_pool.to_owned()))
    }

    fn borrow_books(context: &Metadata, books: Vec<String>) -> FieldResult<LendResultQuery> {
        let lend = &LendForm { books: books };

        Ok(borrow_books_service(
            &context.username,
            lend,
            context.connection_pool.to_owned(),
        ))
    }

    fn return_books(context: &Metadata, books: Vec<String>) -> FieldResult<LendResultQuery> {
        let lend = &LendForm { books: books };

        Ok(return_books_service(
            &context.username,
            lend,
            context.connection_pool.to_owned(),
        ))
    }
}

pub type Schema = RootNode<'static, QueryRoot, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation {})
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct ResultQuery {
    pub success: bool,
    pub info: String,
    pub data: String,
}
