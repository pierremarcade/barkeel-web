use barkeel_lib::workers::registery;
use std::sync::{Arc, Mutex};
use crate::app::jobs::mail::Mail;

pub fn register_jobs() {
    let mail = Arc::new(Mutex::new(Mail));
    registery::register_global_job("Mail".to_string(), mail);
    //register new job here
}