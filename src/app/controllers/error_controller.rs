
use axum::{ extract::State, BoxError, response::IntoResponse, http::StatusCode };
use tera::{Context, Tera};
use crate::config::application::Config;
use std::sync::Arc;
use tokio::time::error;
use barkeel_lib::html::response;

pub async fn handler_404(State(config): State<Arc<Config>>) -> impl IntoResponse {
    let tera: &Tera = &config.template;
    let rendered = tera.render("404.html", &Context::new()).unwrap();
    response(StatusCode::NOT_FOUND, "text/html", rendered)
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