use diesel::prelude::*;
use barkeel_derives::FormBuilder;
use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;
use chrono::Utc;

#[derive(Serialize, Deserialize, Queryable, FormBuilder, Clone)]
#[diesel(table_name = crate::db::schema::customers)]
pub struct Customer {
    pub id: i64,
    pub customer_id: Option<String>,
    pub name: Option<String>,
    pub agency_id: Option<String>,
    pub sage_id: Option<String>,
    pub agency_name: Option<String>,
    pub surikate_option: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub modified_at: Option<NaiveDateTime>,
    pub team_id: i64,
    pub primary_contact_id: Option<String>,
    pub supervision_or_backup_contract: Option<bool>,
    pub host_group_id: Option<i32>,

}
