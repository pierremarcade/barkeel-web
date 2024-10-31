use barkeel_lib::app::Config;
use crate::app::models::auth::AuthState;
use axum::{
    middleware::Next,
    extract::Request,
    http::{ header, StatusCode },
    body::Body,
    response::Response,
};
use cookie::Cookie;
use crate::config::constants::USER_COOKIE_NAME;

pub(crate) async fn auth(
    config: Config,
    mut request: Request,
    next: Next
) -> axum::response::Response {
    let cookie_header = request.headers().get(header::COOKIE);
    match cookie_header {
        Some(cookie_header) => {
            let cookies: Vec<Cookie> = cookie_header
                .to_str()
                .unwrap_or_default()
                .split(';')
                .filter_map(|s| s.trim().parse::<Cookie>().ok())
                .collect();
            let session_cookie = cookies.iter().find(|cookie| cookie.name() == USER_COOKIE_NAME);
            let path = request.uri().path().to_owned();
            match session_cookie {
                Some(cookie) => {
                    let mut auth_state = AuthState(
                        Some((cookie.value().to_string(), None, config))
                    );
                    request.extensions_mut().insert(auth_state.clone());
                    if auth_state.get_user().await.is_none() {
                        return auth_state.redirect_to_login();
                    } else if path == "/login" {
                        return Response::builder()
                            .status(StatusCode::FOUND)
                            .header("Location", "/")
                            .body(Body::empty())
                            .unwrap();
                    }
                }
                None => {
                    if path != "/login" && !path.starts_with("/public") {
                        return Response::builder()
                            .status(StatusCode::FOUND)
                            .header("Location", "/login")
                            .body(Body::empty())
                            .unwrap();
                    }
                }
            }
        }
        None => {
            return Response::builder()
                .status(StatusCode::FOUND)
                .header("Location", "/login")
                .body(Body::empty())
                .unwrap();
        }
    }
    next.run(request).await
}
