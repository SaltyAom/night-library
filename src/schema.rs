table! {
    lends (username) {
        username -> Varchar,
        books -> Array<Text>,
    }
}

table! {
    libraries (id) {
        id -> Varchar,
        title -> Varchar,
        author -> Varchar,
        price -> Numeric,
    }
}

table! {
    users (username) {
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(lends -> users (username));

allow_tables_to_appear_in_same_query!(
    lends,
    libraries,
    users,
);
