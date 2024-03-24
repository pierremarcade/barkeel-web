
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use barkeel_derives::FormBuilder;

#[derive(Serialize, Deserialize, Queryable, QueryableByName, FormBuilder, Clone)]
#[diesel(table_name = crate::db::schema::books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[form_builder(label = title, id = id)]
pub struct Book {
    pub id: i32,
    pub title: String,
}
