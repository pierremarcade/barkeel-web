
use diesel::prelude::*;
use barkeel_derives::FormBuilder2;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Queryable, FormBuilder2)]
#[diesel(table_name = poles)]
pub struct Pole {
    pub id: i64,
    pub pole: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

}
