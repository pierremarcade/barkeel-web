use axum::{extract::State, BoxError, response::IntoResponse, http::StatusCode};
use barkeel_lib::app::Config;
use crate::app::errors::handle_error::handler_error;
use tokio::time::error;

pub async fn handler_404(State(config): State<Config>) -> impl IntoResponse {
    handler_error(config, StatusCode::NOT_FOUND, "Sorry, we couldn’t find the page you’re looking for.".to_string())
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