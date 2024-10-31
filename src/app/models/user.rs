use barkeel_lib::app::Config;
use fluent_templates::Loader;
use diesel::prelude::*;
use barkeel_derives::{ FormBuilder, Crud };
use serde::{ Deserialize, Serialize };
use validator::Validate;
#[cfg(feature = "postgres")]
use barkeel_lib::database::postgres::DB;
#[cfg(feature = "mysql")]
use barkeel_lib::database::mysql::DB;
#[cfg(feature = "sqlite")]
use barkeel_lib::database::sqlite::DB;
use barkeel_lib::database::lower;

#[derive(Serialize, Deserialize, Queryable, FormBuilder, Validate, Clone, Crud)]
#[diesel(table_name = crate::db::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub session_token: Option<String>,
}
