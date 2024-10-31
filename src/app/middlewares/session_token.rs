use crate::config::constants::SESSION_COOKIE_NAME;
use axum::{
    http::{ header, HeaderValue, HeaderMap },
    response::Response,
    middleware::Next,
    extract::Request,
    http::{header, HeaderMap, HeaderValue},
    middleware::Next,
    response::Response,
};
use barkeel_lib::session::CSRFManager;
use cookie::Cookie;

#[derive(Clone)]
pub struct UniqueId(pub String);

pub async fn unique_id_middleware(request: Request, next: Next) -> Response {
    let headers: &HeaderMap = request.headers();
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        let cookies: Vec<Cookie> = cookie_header
            .to_str()
            .unwrap_or_default()
            .split(';')
            .filter_map(|s| s.trim().parse::<Cookie>().ok())
            .collect();
        let session_cookie = cookies.iter().find(|cookie| cookie.name() == SESSION_COOKIE_NAME);
        match session_cookie {
            Some(cookie) => {
                let value = cookie.value().to_string();
                let csrf_manager = CSRFManager::new();
                if csrf_manager.is_token_valid(value.clone()) {
                    let response = next.run(request).await;
                    return response;
                } else {
                    return set_token_cookie(request, next).await;
                }
            }
            None => {
                return set_token_cookie(request, next).await;
            }
        }
    }
    set_token_cookie(request, next).await
}

async fn set_token_cookie(request: Request, next: Next) -> Response {
    let csrf_manager = CSRFManager::new();
    let unique_id = csrf_manager.generate_csrf_token(2*60*60);
    let mut cookie = Cookie::build((SESSION_COOKIE_NAME, unique_id)).path("/").http_only(true);
    if request.uri().scheme_str() == Some("https") {
        cookie = cookie.secure(true);
    }
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(header::SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).unwrap());
    response
}
