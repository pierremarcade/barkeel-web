use axum::{ http::{ header, HeaderValue }, middleware::Next, extract::Request };
use cookie::Cookie;
use serde::Deserialize;
use axum::extract::Query;
use axum::RequestPartsExt;
use crate::config::constants::{ LOCALE_COOKIE_NAME, DEFAULT_LOCALE };

#[derive(Deserialize, Debug)]
pub struct LocaleQuery {
    locale: Option<String>,
}

pub(crate) async fn change_locale(request: Request, next: Next) -> axum::response::Response {
    let cookie_header = request.headers().get(header::COOKIE);
    let mut cookie = match cookie_header {
        Some(cookie_header) => {
            let cookies: Vec<Cookie> = cookie_header
                .to_str()
                .unwrap_or_default()
                .split(';')
                .filter_map(|s| s.trim().parse::<Cookie>().ok())
                .collect();
            let locale_cookie = cookies
                .iter()
                .find(|cookie| cookie.name() == LOCALE_COOKIE_NAME);
            match locale_cookie {
                Some(cookie) => {
                    let mut cookie = cookie.clone().into_owned();
                    cookie.set_path("/");
                    cookie.set_http_only(true);
                    cookie
                }
                None =>
                    Cookie::build((LOCALE_COOKIE_NAME, DEFAULT_LOCALE))
                        .path("/")
                        .http_only(true)
                        .into(),
            }
        }
        None =>
            Cookie::build((LOCALE_COOKIE_NAME, DEFAULT_LOCALE)).path("/").http_only(true).into(),
    };

    let (mut parts, body) = request.into_parts();
    let params: Query<LocaleQuery> = parts.extract().await.expect("REASON");
    if let Some(locale) = &params.locale {
        cookie.set_value(locale);
    }
    let request = Request::from_parts(parts, body);
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(header::SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).unwrap());
    response
}
