use crate::graphql::ResultQuery;
use crate::libraries::model::{Librarie, ListBookResultQuery};

use crate::database::ConnectionPool;

pub fn add_book_service(library: &Librarie, connection_pool: ConnectionPool) -> ResultQuery {
    let connection = connection_pool.get().expect("Connection Pool");

    let response = library.add(&connection);

    match response {
        Ok(book) => ResultQuery {
            success: true,
            info: "".to_owned(),
            data: format!("Add {}", book.title),
        },
        Err(error) => {
            eprintln!("{}", error);

            ResultQuery {
                success: false,
                info: "Something went wrong".to_owned(),
                data: "".to_owned(),
            }
        }
    }
}

pub fn list_book_service(connection_pool: ConnectionPool) -> ListBookResultQuery {
    let connection = connection_pool.get().expect("Connection Pool");

    let library = Librarie {
        ..Default::default()
    };

    let response = library.list(&connection);

    match response {
        Ok(library) => ListBookResultQuery {
            success: true,
            info: "".to_owned(),
            data: library,
        },
        Err(error) => {
            eprintln!("{}", error);

            ListBookResultQuery {
                success: false,
                info: "Something went wrong".to_owned(),
                data: vec![],
            }
        }
    }
}

pub fn remove_book_service(id: &str, connection_pool: ConnectionPool) -> ResultQuery {
    let connection = connection_pool.get().expect("Connection Pool");

    let library = Librarie {
        id: id.to_owned(),
        ..Default::default()
    };

    let response = library.remove(&connection);

    match response {
        Ok(_) => ResultQuery {
            success: true,
            info: "".to_owned(),
            data: format!("Remove {}", id),
        },
        Err(_) => ResultQuery {
            success: false,
            info: "Unable to connect to database".to_owned(),
            data: "".to_owned(),
        },
    }
}
