use barkeel_lib::workers::registery;
use async_std::sync::{Arc, Mutex};
use crate::app::jobs::mail::Mail;

pub async fn register_jobs() {
    let mail = Arc::new(Mutex::new(Mail));
    registery::register_global_job("Mail".to_string(), mail).await;
    //register new job here
}