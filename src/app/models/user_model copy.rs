use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Debug, Queryable, FormBuilder)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub name: String,

}
