use axum::http::HeaderMap;
use crate::config::constants::DEFAULT_LOCALE;
use crate::app::models::auth::AuthState;
use crate::app::models::user::User;

barkeel_lib::template_handler!();

pub mod index_controller;
pub mod error_controller;
pub mod auth_controller;