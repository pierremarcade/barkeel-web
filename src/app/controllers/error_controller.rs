
use axum::{ extract::State, BoxError, response:: { Html, IntoResponse }, http::{header, HeaderMap, StatusCode} };
use tera::{Context, Tera};
use crate::config::application::Config;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::error;

pub async fn handler_404(State(config): State<Arc<Config>>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());
    let tera: &Tera = &config.template;
    let rendered = tera.render("404.html", &Context::new()).unwrap();
    (StatusCode::NOT_FOUND, headers, rendered)
}

pub async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {err}"),
        )
    }
}