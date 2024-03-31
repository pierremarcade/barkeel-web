
use axum::{ extract::State, BoxError, response::IntoResponse, http::StatusCode };
use tera::{Context, Tera};
use crate::config::application::Config;
use std::sync::Arc;
use tokio::time::error;
use crate::app::utils::response::Response;

pub async fn handler_404(State(config): State<Arc<Config>>) -> impl IntoResponse {
    render_404(config)
}

pub fn render_404(config: Arc<Config>) -> Response<'static> {
    let tera: &Tera = &config.template;
    let rendered = tera.render("404.html", &Context::new()).unwrap();
    Response{status_code: StatusCode::NOT_FOUND, content_type: "text/html", datas: rendered}
}

pub fn handler_error(config: Arc<Config>, status_code: StatusCode, message: String) -> Response<'static> {
    let tera: &Tera = &config.template;
    let tera = tera.clone();
    let mut context = Context::new();
    context.insert("code", &status_code.to_string());
    context.insert("messge", &message);
    let rendered = tera.render("error.html", &context).unwrap();
    Response{status_code, content_type: "text/html", datas: rendered}
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