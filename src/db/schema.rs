// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Int4,
        user_id -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    articles,
    users,
);
