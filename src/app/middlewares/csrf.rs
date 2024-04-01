use axum::{
    http::{header,HeaderValue, HeaderMap},
    response::Response,
    middleware::Next,
    extract::Request,
};
use cookie::Cookie;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

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

fn generate_unique_id() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    let random: u32 = rand::thread_rng().gen();
    format!("{}-{}", now, random)
}