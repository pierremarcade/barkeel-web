use axum::http::HeaderMap;
use axum::http::header;
use barkeel_lib::session::CSRFManager;
use cookie::Cookie;
use std::sync::Arc;
use crate::config::application::Config;

pub fn get_content_type(headers: HeaderMap) -> String {
    let header_value = headers.get("Content-Type");
    let mut content_type = String::new();
    if let Some(header_value) = header_value {
        content_type = header_value.to_str().unwrap().to_owned();
    }
    content_type
}

pub fn csrf_token_is_valid(headers: HeaderMap, config: Arc<Config>, csrf_token: String) -> bool {
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in Cookie::split_parse(cookie_str) {
                let cookie = cookie.unwrap();
                match cookie.name() {
                    "session_token" => {
                        let csrf_manager: &CSRFManager = &config.csrf_manager;
                        if csrf_manager.is_csrf_token_valid(cookie.value().to_string(), csrf_token.clone()) {
                            return true;
                        }
                    },
                    _ =>  {  }
                }
            }
        }
    }
    return false;
}
