// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (user_id) {
        user_id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 50]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_on -> Timestamp,
        last_login -> Nullable<Timestamp>,
    }
}

diesel::table! {
    articles (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
    }
}

diesel::joinable!(articles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    articles,
    users,
);
