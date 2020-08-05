use crate::lends::model::{Lend, LendDetailResultQuery, LendForm, LendResultQuery};

use crate::database::ConnectionPool;

pub fn borrow_books_service(
    username: &Option<String>,
    lend_form: &LendForm,
    connection_pool: ConnectionPool,
) -> LendResultQuery {
    if username.is_none() {
        return LendResultQuery {
            success: false,
            info: "Unauthorized".to_owned(),
            data: vec![],
        };
    }

    let connection = connection_pool.get().expect("Connection Pool");

    let to_be_lend = Lend {
        username: username.as_ref().unwrap().to_owned(),
        books: lend_form.books.to_owned(),
    };

    let response = to_be_lend.append(&connection);

    match response {
        Ok(appended) => {
            if let Some(lend) = appended {
                LendResultQuery {
                    success: true,
                    info: "".to_owned(),
                    data: lend.books,
                }
            } else {
                LendResultQuery {
                    success: false,
                    info: "Failed to borrow books".to_owned(),
                    data: vec![],
                }
            }
        }
        Err(_) => LendResultQuery {
            success: false,
            info: "Something went wrong".to_owned(),
            data: vec![],
        },
    }
}

pub fn list_borrowed_service(
    username: &Option<String>,
    connection_pool: ConnectionPool,
) -> LendDetailResultQuery {
    if username.is_none() {
        return LendDetailResultQuery {
            success: false,
            info: "Unauthorized".to_owned(),
            data: vec![],
        };
    }

    let connection = connection_pool.get().expect("Connection Pool");

    let books = Lend {
        username: username.as_ref().unwrap().to_owned(),
        books: vec![],
    };

    let response = books.list_detail(&connection);

    match response {
        Ok(lend) => {
            if let Some(borrowed) = lend {
                LendDetailResultQuery {
                    success: true,
                    info: "".to_owned(),
                    data: borrowed,
                }
            } else {
                LendDetailResultQuery {
                    success: true,
                    info: "".to_owned(),
                    data: vec![],
                }
            }
        }
        Err(_) => LendDetailResultQuery {
            success: false,
            info: "Something went wrong".to_owned(),
            data: vec![],
        },
    }
}

pub fn return_books_service(
    username: &Option<String>,
    lend_form: &LendForm,
    connection_pool: ConnectionPool,
) -> LendResultQuery {
    if username.is_none() {
        return LendResultQuery {
            success: false,
            info: "Unauthorized".to_owned(),
            data: vec![],
        };
    }

    let connection = connection_pool.get().expect("Connection Pool");

    let to_be_lend = Lend {
        username: username.as_ref().unwrap().to_owned(),
        books: lend_form.books.to_owned(),
    };

    let response = to_be_lend.remove(&connection);

    match response {
        Ok(returned) => {
            if let Some(lend) = returned {
                LendResultQuery {
                    success: true,
                    info: "".to_owned(),
                    data: lend.books,
                }
            } else {
                LendResultQuery {
                    success: false,
                    info: "Failed to return books".to_owned(),
                    data: vec![],
                }
            }
        }
        Err(_) => LendResultQuery {
            success: false,
            info: "Something went wrong".to_owned(),
            data: vec![],
        },
    }
}
