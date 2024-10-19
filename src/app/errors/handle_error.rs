use axum::http::StatusCode;
use tera::{Context, Tera};
use barkeel_lib::app::Config;
use barkeel_lib::app::http::response::Response;

pub fn handler_error(config: Config, status_code: StatusCode, message: String) -> Response<'static> {
    let template = if status_code == StatusCode::NOT_FOUND {
        "404.html"
    } else {
        "error.html"
    };
    let tera: &Tera = &config.template;
    let mut context = Context::new();
    context.insert("code", &status_code.to_string());
    context.insert("message", &message);
    let rendered = tera.render(template, &context).unwrap();
    Response {
        status_code,
        content_type: "text/html",
        datas: rendered,
    }
}

