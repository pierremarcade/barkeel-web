use axum::{
    http::{header, HeaderValue, HeaderMap},
    response::Response,
    middleware::Next,
    extract::Request,
};
use cookie::Cookie;
use barkeel_lib::session::CSRFManager;
use crate::config::constants::SESSION_COOKIE_NAME;

#[derive(Clone)]
pub struct UniqueId(pub String);

pub async fn unique_id_middleware(request: Request, next: Next) -> Response {
    let headers: &HeaderMap = request.headers();
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            if cookie_str.contains(format!("{}=", SESSION_COOKIE_NAME).as_str()) {
                let response = next.run(request).await;
                return response;
            }
        }
    }
    let csrf_manager = CSRFManager::new();
    let unique_id = csrf_manager.generate_csrf_token();
    let cookie = Cookie::build((SESSION_COOKIE_NAME, unique_id)).path("/").http_only(true);
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );
    response
}