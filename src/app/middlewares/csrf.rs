use axum::{
    http::{header,HeaderValue, HeaderMap, StatusCode},
    response::Response,
    middleware::Next,
    extract:: {Extension, Request, State},
};
use axum::response::IntoResponse;

use cookie::Cookie;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use rand::Rng;
use barkeel_lib::csrf::CSRFManager;
use crate::config::application::Config;
use crate::app::controllers::error_controller;

#[derive(Clone)]
pub struct UniqueId(pub String);

pub async fn unique_id_middleware(request: Request, next: Next) -> Response {
    let headers: &HeaderMap = request.headers();
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            if cookie_str.contains("unique_id=") {
                let response = next.run(request).await;
                return response;
            }
        }
    }
    let unique_id = generate_unique_id();
    let cookie = Cookie::build(("unique_id", unique_id)).path("/").http_only(true);
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );
    response
}

pub async fn ensure_csrf_is_valid_middleware( headers: HeaderMap, Extension(config): Extension<Arc<Config>>, request: Request, next: Next) -> impl IntoResponse {
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in Cookie::split_parse(cookie_str) {
                let cookie = cookie.unwrap();
                match cookie.name() {
                    "unique_id" => {
                        let csrf_manager: &CSRFManager = &config.csrf_manager;
                        if csrf_manager.is_csrf_token_valid(cookie.value().to_string(), "sfsfdsfdsf".to_string()) {
                            return next.run(request).await;
                        }
                    },
                    _ => {}
                }
            }
        }
    }
    let error_response = error_controller::handler_error(
        Arc::clone(&config),
        StatusCode::UNAUTHORIZED,
        "Invalid CSRF token".to_string(),
    );
    error_response.into_response()
}

fn generate_unique_id() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    let random: u32 = rand::thread_rng().gen();
    format!("{}-{}", now, random)
}